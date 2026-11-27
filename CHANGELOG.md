# Changelog

## 0.1.0 (2026-11-27)

The first release in ten years: Apple silicon support, a modern toolchain,
and fixes for the Intel API.

### Added

- Apple silicon (arm64) support with a safe API: `Vm`, `GuestMemory`,
  `Vcpu` (not `Send`: a vCPU belongs to the thread that created it),
  `VcpuHandle`/`exit_vcpus`, `VcpuExit` and exception decoding (`Exception`,
  `ExceptionClass`, `DataAbort`, `SysRegAccess`), registers (`Reg`,
  `SimdFpReg`, `SysReg`), interrupts, debug traps, the virtual timer,
  `VcpuConfig`, `max_vcpu_count` and `page_size`. Raw bindings are in
  `hypervisor::ffi`.
- Runnable examples for both architectures (`arm64_hello`, `x86_64_hello`),
  answering #1, and runtime tests that run real guests on Apple silicon.
- `entitlements.plist` and `scripts/run-signed.sh`, used as the cargo runner,
  to sign binaries with the `com.apple.security.hypervisor` entitlement.
- `VMXCap` variants for the remaining capabilities in `hv_vmx.h`.
- GitHub Actions CI on Apple silicon, Intel and Linux. The Intel runner
  supports VT-x, so the x86_64 example and runtime test run a real guest there.

### Changed

- Every function returns `Result<T, hypervisor::Error>` instead of an error
  code (thanks @stlankes, #2). `Error` no longer has a `Success` variant, is
  `#[non_exhaustive]`, implements `Display` and `std::error::Error`, and its
  variants were renamed (`BadArg` is now `BadArgument`, `NoRes` is
  `NoResources`, `NoDev` is `NoDevice`, `Unsupp` is `Unsupported`, `Error`
  is `Generic`), with new `Denied` and `Fault` variants.
- `map_mem` is now `unsafe`: the guest can write to the mapped memory and it
  must outlive the mapping, which a `&[u8]` cannot guarantee.
- The Intel API is only compiled for `x86_64`; on other macOS targets and
  other operating systems it is absent (the crate is empty off macOS).
- Edition 2024; minimum supported Rust version 1.85. The `libc` dependency
  is only used on Apple silicon. Documentation moved to docs.rs.

### Fixed

- `vCPU::write_msr` passed a pointer to the value instead of the value, so it
  wrote a stack address into the MSR.
- Compiler warnings (thanks @stlankes, #2).

## 0.0.8 (2016-05-24)

- Renamed `destory_vm()` to `destroy_vm()` and `destory()` to `destroy()`.
