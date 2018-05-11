use os_bootinfo::{FrameRange, MemoryMap, MemoryRegion, MemoryRegionType};
use spin::Mutex;
use memory::paging::{PhysFrame, PhysFrameRange};
use x86_64::PhysAddr;

use boot_info;

const MAP_SIZE: usize = 32;

lazy_static! {
    pub static ref ALLOC: Mutex<FrameAllocator> = Mutex::new(FrameAllocator::init(
        &boot_info::BOOT_INFO.lock().memory_map
    ));
}

struct RegionList {
    array: [Option<PhysFrameRange>; MAP_SIZE],
    length: usize,
}

impl RegionList {
    pub fn new() -> Self {
        let array = [None; MAP_SIZE];
        let length = 0;

        Self { array, length }
    }

    pub fn add(&mut self, range: FrameRange) {
        if self.length >= MAP_SIZE {
            panic!("Region list overflow!")
        }

        let frame_range = PhysFrameRange::from(range);

        self.array[self.length] = Some(frame_range);
        self.length += 1;
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn remove_last(&mut self) {
        self.array[0] = None;

        for i in 1..self.array.len() {
            self.array[i - 1] = self.array[i];
        }

        self.length -= 1;
    }

    pub fn first(&mut self) -> PhysFrameRange {
        self.array[0].expect("Out of memory")
    }
}

pub struct FrameAllocator {
    usable: RegionList,
}

impl FrameAllocator {
    pub fn init(map: &MemoryMap) -> Self {
        let mut usable = RegionList::new();

        let map_iter = map.iter();

        for region in map_iter {
            if region.region_type == MemoryRegionType::Usable {
                usable.add(region.range);
            }
        }

        if usable.length() < 1 {
            panic!("No usable memory");
        }

        FrameAllocator { usable }
    }

    pub fn allocate_frame(&mut self) -> PhysFrame {
        let mut current_region = self.usable.first();

        if current_region.is_empty() {
            self.usable.remove_last();
            current_region = self.usable.first();
        }

        let frame = current_region.start;

        if let Some(ref mut range) = self.usable.array[0] {
            range.start += 1;
        }

        frame
    }

    pub fn deallocate_frame(&mut self, frame: PhysFrame) {
        unimplemented!()
    }
}
