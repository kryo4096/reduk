use os_bootinfo::BootInfo;
use spin::Mutex;
use x86_64::structures::paging::{Page, Size4KB};
use x86_64::VirtAddr;

const BOOT_INFO_ADDR: u64 = 0xb0071f0000;

lazy_static! {
    pub static ref BOOT_INFO: Mutex<&'static mut BootInfo> = Mutex::new(unsafe {
        let page: Page<Size4KB> = Page::containing_address(VirtAddr::new(BOOT_INFO_ADDR));
        let ptr: *mut BootInfo = page.start_address().as_mut_ptr();

        &mut *ptr
    });
}

pub fn show() {
    let boot_info = BOOT_INFO.lock();

    kprintln!("bootinfo version: {}", boot_info.version);
    kprintln!("p4 table address: {:#x}", boot_info.p4_table_addr);

    for region in boot_info.memory_map.iter() {
        let start = region.range.start_addr();
        let end = region.range.end_addr();
        let rtype = region.region_type;

        kprintln!("{:#x} - {:#x} : {:?}", start, end, rtype);
    }
}
