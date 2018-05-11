use os_bootinfo::{FrameRange, MemoryMap, MemoryRegion, MemoryRegionType};

use memory::*;

use boot_info;

const MAP_SIZE: usize = 32;

pub struct AreaFrameAllocator<'a> {
    memory_map: &'a mut MemoryMap,
    currently_usable: usize,
    currently_in_use: usize,
}

impl<'a> AreaFrameAllocator<'a> {
    pub fn new(memory_map: &'a mut MemoryMap) -> Self {
        let mut frame_alloc = AreaFrameAllocator {
            memory_map: memory_map,
            currently_usable: 0,
            currently_in_use: 0,
        };

        frame_alloc.next_area().unwrap();

        frame_alloc
    }

    fn currently_usable(&mut self) -> &mut MemoryRegion {
        let region = &mut self.memory_map[self.currently_usable];
        assert_eq!(region.region_type, MemoryRegionType::Usable);
        region
    }

    fn currently_in_use(&mut self) -> &mut MemoryRegion {
        let region = &mut self.memory_map[self.currently_in_use];
        assert_eq!(region.region_type, MemoryRegionType::InUse);
        region
    }

    fn next_area(&mut self) -> Result<(), ()> {
        
        let mut region_in_use = None;

        /* Find usable memory region */
        for region in self.memory_map.iter_mut() {
            if region.region_type == MemoryRegionType::Usable {

                region_in_use = Some(MemoryRegion {
                    range: {
                        let usable_addr = region.range.start_addr();
                        FrameRange::new(usable_addr, usable_addr + 1)
                    },
                    region_type: MemoryRegionType::InUse,
                });

                region.range.start_frame_number += 1;
            }
        }

        if region_in_use.is_none() {
            return Err(());
        }

        self.memory_map.add_region(region_in_use.unwrap());

        let mut usable_frame = None;

        for (i, region) in self.memory_map.iter().enumerate() {
            if region.region_type == MemoryRegionType::Usable {
                usable_frame = Some(region.range.start_frame_number);
                self.currently_usable = i;
            }
        }

        let usable_frame = usable_frame.unwrap();

        for (i, region) in self.memory_map.iter().enumerate() {
            if region.region_type == MemoryRegionType::InUse
                && region.range.start_frame_number == usable_frame - 1
            {
                self.currently_in_use = i;
                return Ok(());
            }
        }

        unreachable!();

    }

    pub fn print_memory_map(&self) {
        for region in self.memory_map.iter() {
            let start = region.range.start_addr();
            let end = region.range.end_addr();
            let rtype = region.region_type;

            kprintln!("{:#x} - {:#x} : {:?}", start, end, rtype);
        }
    }
}

impl<'a> FrameAllocator for AreaFrameAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<Frame> {
        if self.currently_usable().range.is_empty() {
            self.currently_usable().region_type = MemoryRegionType::Empty;

            if self.next_area() == Err(()) {
                return None;
            }
        }

        let frame = Frame::containing_address(self.currently_usable().range.start_addr());

        self.currently_usable().range.start_frame_number += 1;
        self.currently_in_use().range.end_frame_number = self.currently_usable().range.start_frame_number;

        Some(frame)
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        unimplemented!()
    }
}
