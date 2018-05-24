#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(const_fn)]
#![feature(compiler_builtins_lib)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(alloc, global_allocator, allocator_api,allocator_internals)] 

pub extern crate spin;
pub extern crate alloc;

extern crate alloc_cortex_m;
extern crate rlibc;

//pub mod memstub;
use core::{fmt,ptr};
pub use spin::Mutex;

use alloc_cortex_m::CortexMHeap;
#[global_allocator]
static GLOBAL: CortexMHeap = CortexMHeap::empty();

//use memstub::Solo5Allocator;

//#[global_allocator]
//static GLOBAL: Solo5Allocator = Solo5Allocator { heap_start: ptr::null_mut(), heap_size: 0 };

#[no_mangle]
pub extern "C" fn __floatundisf() {
    panic!()
}

#[lang = "oom"]
#[no_mangle]
pub fn rust_oom() -> ! {
    loop{}
}

#[allow(improper_ctypes)]
extern { pub fn rust_main(cmdline : &str) -> isize; }

unsafe fn strlen(buf : *const u8) -> usize {
	let mut idx = 0;
	while *buf.offset(idx) != 0 {
		idx += 1;
	}

	return idx as usize;
}

pub enum solo5_result {
    SOLO5_R_OK = 0,
    SOLO5_R_AGAIN = 1,
    SOLO5_R_EINVAL = 2,
    SOLO5_R_EUNSPEC = 3
}

#[repr(C)]
#[derive(Debug)]
pub struct solo5_start_info {
    pub cmdline: *const u8,
    pub heap_start: usize,
    pub heap_size: usize
}


#[no_mangle]
extern "C" {
	pub fn solo5_net_write_sync(data: *mut u8, len: isize) -> isize;
    pub fn solo5_net_read_sync(data: *mut u8, len: *mut isize) -> isize;
    pub fn solo5_net_mac_str() -> *const u8;
    
    pub fn solo5_blk_write_sync(sec: u64, data: *mut u8, n: isize)-> isize;
    pub fn solo5_blk_read_sync(sec:u64, data : *mut u8, n : *mut isize) -> isize;
    pub fn solo5_blk_sector_size() -> isize;
    pub fn solo5_blk_sectors() -> u64;
    pub fn solo5_blk_rw() -> isize;

    pub fn solo5_console_write(buf : *const u8, len : usize ) -> isize;
    pub fn solo5_exit(result: isize) -> !;

    pub fn solo5_clock_monotonic() -> u64;
    pub fn solo5_clock_wall() -> u64;
    pub fn solo5_poll(nsec:u64) -> isize;
}

pub struct Console;

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
		unsafe {
            solo5_console_write(s.as_ptr(), s.len());
		};
        Ok(())
    }
}

pub static CONSOLE : Mutex<Console> = Mutex::new(Console{});

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::CONSOLE.lock();
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[lang = "eh_personality"] extern fn eh_personality() {}                                                                                              
#[lang = "panic_fmt"] 
#[no_mangle] 
pub extern fn panic_fmt(
        _args: ::core::fmt::Arguments,
        _file: &'static str,
        _line: u32 ) -> ! {
    println!("panic {} occured at {}:{}", _args, _file, _line);
	unsafe {solo5_exit(1);}
}

#[no_mangle]
pub unsafe fn solo5_app_main(info : *const solo5_start_info) -> isize {
	CONSOLE.force_unlock();
    // init allocator
    println!("{:?}",*info);
    GLOBAL.init((*info).heap_start,(*info).heap_size);
	let p = core::str::from_utf8(core::slice::from_raw_parts((*info).cmdline, strlen((*info).cmdline) as usize)).unwrap();
	solo5_exit(rust_main(p))
}

