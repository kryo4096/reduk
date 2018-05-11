mod frame_allocator;


pub use x86_64::structures::paging;
pub use x86_64::{VirtAddr,PhysAddr};
use self::paging::PhysFrame;
pub use self::frame_allocator::ALLOC as FRAME_ALLOCATOR;


pub fn allocate_frame() -> PhysFrame {
    frame_allocator::ALLOC.lock().allocate_frame()
}
