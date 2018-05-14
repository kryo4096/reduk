use os_bootinfo::{FrameRange, MemoryMap, MemoryRegion, MemoryRegionType};

use memory::*;

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
        use self::MemoryRegionType::{InUse, Usable};

        let usable_addr = { 
            let first_usable_region = self.memory_map
                .iter_mut()
                .filter(|reg| reg.region_type == Usable)
                .nth(0);

            if first_usable_region == None {
                return Err(());
            }

            let first_usable_region = first_usable_region.unwrap();

            first_usable_region.range.start_frame_number += 1;

            first_usable_region.range.start_addr()

        };

        self.memory_map.add_region(MemoryRegion {
            range: FrameRange::new(usable_addr - 1, usable_addr),
            region_type: InUse,
        });

        let currently_usable = self.memory_map
            .iter()
            .position(|reg| reg.region_type == Usable)
            .expect("Usable region not found! This shouldn't happen.");

        assert_eq!(self.memory_map[currently_usable].region_type, Usable);

        let currently_in_use = currently_usable - 1;

        assert_eq!(self.memory_map[currently_in_use].region_type, InUse);

        self.currently_in_use = currently_in_use;
        self.currently_usable = currently_usable;

        Ok(())
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
        self.currently_in_use().range.end_frame_number =
            self.currently_usable().range.start_frame_number;

        Some(frame)
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        unimplemented!()
    }
}
