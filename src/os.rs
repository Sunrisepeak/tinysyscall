mod linux;

#[cfg(target_arch = "riscv64")]
pub use linux::riscv64::SAL as syscall;

#[cfg(target_arch = "x86_64")]
pub use linux::x86_64::SAL as syscall;