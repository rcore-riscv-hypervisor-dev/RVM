use crate::{HostPhysAddr, HostVirtAddr};

/// Allocate physical frame
pub fn alloc_frame() -> Option<HostPhysAddr> {
    unsafe { rvm_alloc_frame() }
}

/// Deallocate physical frame
pub fn dealloc_frame(paddr: HostPhysAddr) {
    unsafe { rvm_dealloc_frame(paddr) }
}

// RISC-V requires 16KB-aligned first level page table for GPA translation.
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub fn alloc_frame_x4() -> Option<HostPhysAddr> {
    unsafe { rvm_alloc_frame_x4() }
}
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub fn dealloc_frame_x4(paddr: HostPhysAddr) {
    unsafe { rvm_dealloc_frame_x4(paddr) }
}

/// Convert physical address to virtual address
pub fn phys_to_virt(paddr: HostPhysAddr) -> HostVirtAddr {
    unsafe { rvm_phys_to_virt(paddr) }
}

/// Whether need to inject timer interrupt to guest when an interrupt occurs on host
#[cfg(target_arch = "x86_64")]
pub fn is_host_timer_interrupt(vector: u8) -> bool {
    unsafe { rvm_is_host_timer_interrupt(vector) }
}

/// Whether need to inject serial interrupt to guest when an interrupt occurs on host
#[cfg(target_arch = "x86_64")]
pub fn is_host_serial_interrupt(vector: u8) -> bool {
    unsafe { rvm_is_host_serial_interrupt(vector) }
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub fn riscv_trap_handler_no_frame(sepc: usize) {
    let mut sepc_new = sepc;
    unsafe { rvm_riscv_trap_handler_no_frame(&mut sepc_new) }
    if sepc_new != sepc {
        panic!("user_fixup called in trap handler, which should not happen for a vm trap.");
    }
}
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub fn riscv_check_hypervisor_extension() -> bool {
    unsafe { rvm_riscv_check_hypervisor_extension() }
}

extern "Rust" {
    fn rvm_alloc_frame() -> Option<HostPhysAddr>;
    fn rvm_dealloc_frame(_paddr: HostPhysAddr);
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    fn rvm_alloc_frame_x4() -> Option<HostPhysAddr>;
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    fn rvm_dealloc_frame_x4(_paddr: HostPhysAddr);
    fn rvm_phys_to_virt(_paddr: HostPhysAddr) -> HostVirtAddr;

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    fn rvm_riscv_check_hypervisor_extension() -> bool;
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    fn rvm_riscv_trap_handler_no_frame(sepc: &mut usize);

    #[cfg(target_arch = "x86_64")]
    fn rvm_is_host_timer_interrupt(vector: u8) -> bool;
    #[cfg(target_arch = "x86_64")]
    fn rvm_is_host_serial_interrupt(vector: u8) -> bool;

}
