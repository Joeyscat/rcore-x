#![no_std]
#![no_main]
#![feature(panic_info_message)] // PanicInfo::message 获取报错信息需要

pub mod batch;
#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;
mod sync;
pub mod syscall;
pub mod trap;

use core::arch::global_asm;
use log::{debug, info, trace};

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    trace!("clear bss finish");
    trace!("logging init bss finish");

    debug!("Hello, World!");
    print_sections();

    trap::init();
    batch::init();
    batch::run_next_app();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        // unsafe
        unsafe {
            // 对bss段进行清零
            (a as *mut u8).write_volatile(0)
        }
    })
}

fn print_sections() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
    }

    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    // info("load range : [%d, %d] start = %d\n", s, e, start);
}
