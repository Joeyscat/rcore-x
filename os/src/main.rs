//! The main module and entrypoint
//! 
//! Various facilities of the kernels are implemented as submodules. The most
//! important ones are:
//! 
//! - [`trap`]: Handles all cases of switching from userspace to the kernel
//! - [`task`]: Task management
//! - [`syscall`]: System call handing nad implementation
//! 
//! The operating system also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality. (See its source cde for
//! details.)
//! 
//! We then call [`task::run_first_task()`] and for the first time go to
//! userspace.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)] // PanicInfo::message 获取报错信息需要

use core::{arch::global_asm};
use log::{debug, info, trace};

mod config;
#[macro_use]
mod console;
mod lang_items;
mod loader;
mod logging;
mod sbi;
mod sync;
pub mod syscall;
pub mod task;
pub mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    trace!("clear bss finish");
    trace!("logging init bss finish");

    debug!("Hello, World!");
    print_sections();

    trap::init();

    loader::load_apps();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}

/// clear BSS segment
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
