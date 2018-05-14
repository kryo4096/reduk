use core::ops::{Index, IndexMut};
use core::marker::PhantomData;

use memory::paging::entry::*;
use memory::paging::ENTRY_COUNT;
use memory::paging::VirtualAddress;
use memory::FrameAllocator;

use memory::paging::entry;

pub const P4: *mut Table<Level4> = 0xffffffff_fffff000 as *mut _;

pub struct Table<L> {
    entries: [Entry; ENTRY_COUNT as usize],
    level: PhantomData<L>,
}

impl<L> Table<L> where L: TableLevel {
    fn zero(&mut self) {
        self.entries.iter_mut().for_each(Entry::set_unused);
    }
}


impl<L> Table<L> where L: HierarchicalLevel {

    fn next_table_address(&self, index: u64) -> Option<VirtualAddress> {
        let entry_flags = self[index].flags();
        if entry_flags.contains(EntryFlags::PRESENT) && !entry_flags.contains(EntryFlags::HUGE_PAGE) {
            let table_address = self as *const _ as u64;
            Some((table_address << 9) | (index << 12) as u64)
        } else {
            None
        }
    }

    pub fn next_table(&self, index: u64) -> Option<&Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    pub fn next_table_mut(&mut self, index: u64) -> Option<&mut Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &mut *(address as *mut _) })
    }

    pub fn next_table_create<A>(&mut self, index: u64, allocator: &mut A) -> &mut Table<L::NextLevel> where A: FrameAllocator {
        
        if self.next_table(index).is_none() {
            assert!(!self[index].flags().contains(EntryFlags::HUGE_PAGE), "huge pages not supported");

            let frame = allocator.allocate_frame().expect("out of memory");
            self[index].set(frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
            self.next_table_mut(index).unwrap().zero();
        }

        self.next_table_mut(index).unwrap()
    }
}

impl<L> Index<u64> for Table<L> where L: TableLevel {
    type Output = Entry;

    fn index(&self, index: u64) -> &Entry {
        &self.entries[index as usize]
    }
}

impl<L> IndexMut<u64> for Table<L> where L: TableLevel {

    fn index_mut(&mut self, index: u64) -> &mut Entry {
        &mut self.entries[index as usize]
    }
}

pub trait TableLevel {}
pub trait HierarchicalLevel : TableLevel {
    type NextLevel: TableLevel;
}

pub enum Level4 {}
pub enum Level3 {}
pub enum Level2 {}
pub enum Level1 {}

impl TableLevel for Level1 {}
impl TableLevel for Level2 {}
impl TableLevel for Level3 {}
impl TableLevel for Level4 {}

impl HierarchicalLevel for Level2 {
    type NextLevel = Level1;
}
impl HierarchicalLevel for Level3 {
    type NextLevel = Level2;
}
impl HierarchicalLevel for Level4 {
    type NextLevel = Level3;
}


