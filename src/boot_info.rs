use os_bootinfo::BootInfo;
use spin::Mutex;
use x86_64::structures::paging::{Page, Size4KB};
use x86_64::VirtAddr;

const BOOT_INFO_ADDR: u64 = 0xb0071f0000;

pub unsafe fn get_boot_info() -> &'static mut BootInfo {
        let page: Page<Size4KB> = Page::containing_address(VirtAddr::new(BOOT_INFO_ADDR));
        &mut *page.start_address().as_mut_ptr()
}



