//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.

mod address;
// mod frame_allocator;
mod heap_allocator;
mod page_table;
mod frame_allocator;
mod memory_set;
// mod memory_set;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{FrameTracker, frame_alloc};
pub use memory_set::remap_test;
pub use memory_set::{KERNEL_SPACE, MapPermission, MemorySet};
use page_table::{PTEFlags, PageTable};
pub use page_table::{PageTableEntry, translated_byte_buffer, translated_str, translated_refmut};

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    println!("[Kernel] Heap allocator initialized");
    frame_allocator::init_frame_allocator();
    println!("[Kernel] Frame allocator initialized");
    KERNEL_SPACE.exclusive_access().activate();
    println!("[Kernel] Kernel space activated");
}