//! Guest physical memory.

use core::ffi::c_void;
use core::marker::PhantomData;
use core::ops::{BitOr, BitOrAssign};
use core::ptr::{self, NonNull};

use super::{Vm, check, ffi};
use crate::{Error, Result};

/// Guest memory permissions (`hv_memory_flags_t`).
///
/// Combine with `|`: `MemPerm::READ | MemPerm::EXEC`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MemPerm(u64);

impl MemPerm {
    /// No access.
    pub const NONE: Self = Self(0);
    /// The guest may read.
    pub const READ: Self = Self(ffi::HV_MEMORY_READ);
    /// The guest may write.
    pub const WRITE: Self = Self(ffi::HV_MEMORY_WRITE);
    /// The guest may execute.
    pub const EXEC: Self = Self(ffi::HV_MEMORY_EXEC);
    /// Read and write.
    pub const RW: Self = Self(ffi::HV_MEMORY_READ | ffi::HV_MEMORY_WRITE);
    /// Read and execute.
    pub const RX: Self = Self(ffi::HV_MEMORY_READ | ffi::HV_MEMORY_EXEC);
    /// Read, write and execute.
    pub const RWX: Self = Self(ffi::HV_MEMORY_READ | ffi::HV_MEMORY_WRITE | ffi::HV_MEMORY_EXEC);

    /// The raw `hv_memory_flags_t` bits.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Whether all permissions in `other` are present in `self`.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl BitOr for MemPerm {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for MemPerm {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Returns the host page size (16 KiB on Apple silicon).
///
/// Host addresses, guest addresses and sizes passed to [`Vm::map`] and
/// friends must be multiples of it.
pub fn page_size() -> usize {
    // SAFETY: sysconf has no preconditions.
    let size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    usize::try_from(size).unwrap_or(16 * 1024)
}

/// Zeroed host memory that is mapped into the guest physical address space
/// for as long as it lives.
///
/// The memory is allocated with `mmap`, so it is always page aligned. It is
/// unmapped from the guest and freed when dropped. It borrows the [`Vm`], so
/// it cannot outlive it.
///
/// The guest may change the memory at any time while a vCPU runs, so the
/// host only gets copies in and out ([`read`](Self::read),
/// [`write`](Self::write)), never Rust references into it.
///
/// ```no_run
/// use hypervisor::{GuestMemory, MemPerm, Vm};
///
/// let vm = Vm::new()?;
/// let mut ram = GuestMemory::new(&vm, 0x4000_0000, 1 << 20, MemPerm::RWX)?;
/// ram.write(0, &0xd400_0002u32.to_le_bytes())?; // hvc #0
/// # Ok::<(), hypervisor::Error>(())
/// ```
#[derive(Debug)]
pub struct GuestMemory<'vm> {
    host: NonNull<u8>,
    size: usize,
    ipa: u64,
    _vm: PhantomData<&'vm Vm>,
}

// SAFETY: GuestMemory owns its allocation. Host access only happens through
// `read` (shared) and `write` (exclusive), which copy through raw pointers.
unsafe impl Send for GuestMemory<'_> {}
unsafe impl Sync for GuestMemory<'_> {}

impl<'vm> GuestMemory<'vm> {
    /// Allocates `size` bytes (rounded up to a whole number of host pages)
    /// and maps them at guest physical address `ipa` with permissions `perm`.
    ///
    /// `ipa` must be page aligned and the range must not overlap an existing
    /// mapping; otherwise the framework returns [`Error::BadArgument`].
    pub fn new(vm: &'vm Vm, ipa: u64, size: usize, perm: MemPerm) -> Result<Self> {
        if size == 0 {
            return Err(Error::BadArgument);
        }
        let page = page_size();
        let size = size
            .checked_next_multiple_of(page)
            .ok_or(Error::BadArgument)?;
        // SAFETY: anonymous private mapping, no aliasing with existing memory.
        let addr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_ANON | libc::MAP_PRIVATE,
                -1,
                0,
            )
        };
        if addr == libc::MAP_FAILED {
            return Err(Error::NoResources);
        }
        let host = NonNull::new(addr.cast::<u8>()).ok_or(Error::NoResources)?;
        // SAFETY: the allocation is page aligned, `size` bytes long, and is
        // only freed in Drop after it has been unmapped from the guest.
        if let Err(err) = unsafe { vm.map(host.as_ptr(), ipa, size, perm) } {
            unsafe { libc::munmap(addr, size) };
            return Err(err);
        }
        Ok(Self {
            host,
            size,
            ipa,
            _vm: PhantomData,
        })
    }

    /// The guest physical address of the first byte.
    pub fn ipa(&self) -> u64 {
        self.ipa
    }

    /// The size in bytes (a multiple of the host page size).
    pub fn size(&self) -> usize {
        self.size
    }

    /// Copies `buf.len()` bytes starting at `offset` out of guest memory.
    pub fn read(&self, offset: usize, buf: &mut [u8]) -> Result<()> {
        self.check_range(offset, buf.len())?;
        // SAFETY: in bounds; no Rust references into guest memory exist.
        unsafe {
            ptr::copy_nonoverlapping(self.host.as_ptr().add(offset), buf.as_mut_ptr(), buf.len())
        };
        Ok(())
    }

    /// Copies `data` into guest memory starting at `offset`.
    pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<()> {
        self.check_range(offset, data.len())?;
        // SAFETY: in bounds; `&mut self` excludes other host accesses.
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), self.host.as_ptr().add(offset), data.len())
        };
        Ok(())
    }

    /// Changes the guest's permissions for this memory.
    pub fn protect(&mut self, perm: MemPerm) -> Result<()> {
        check(unsafe { ffi::hv_vm_protect(self.ipa, self.size, perm.bits()) })
    }

    /// The host address of the memory.
    ///
    /// Writing through it while a vCPU may access the memory is the
    /// caller's responsibility.
    pub fn as_ptr(&self) -> *mut u8 {
        self.host.as_ptr()
    }

    fn check_range(&self, offset: usize, len: usize) -> Result<()> {
        match offset.checked_add(len) {
            Some(end) if end <= self.size => Ok(()),
            _ => Err(Error::BadArgument),
        }
    }
}

impl Drop for GuestMemory<'_> {
    fn drop(&mut self) {
        // Unmap first so the guest can no longer reach the pages, then free.
        // Unmapping fails only if the mapping is already gone (for example
        // after `Vm::unmap`), which is fine.
        unsafe {
            ffi::hv_vm_unmap(self.ipa, self.size);
            libc::munmap(self.host.as_ptr().cast::<c_void>(), self.size);
        }
    }
}
