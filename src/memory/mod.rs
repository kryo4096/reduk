mod frame_allocator;
mod paging;

pub const PAGE_SIZE: u64 = 4096;

pub use self::frame_allocator::AreaFrameAllocator;

pub use self::paging::test_paging;

use self::paging::PhysicalAddress;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: u64,
}

impl Frame {
    fn containing_address(address: u64) -> Frame {
        Frame {
            number: address / PAGE_SIZE,
        }
    }

    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}
