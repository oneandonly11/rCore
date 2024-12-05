use core::ptr;

use crate::config::PAGE_SIZE;
use crate::task::{
    get_current_page_table,
    real_mmap,
    real_unmap,
    suspend_current_and_run_next,
    exit_current_and_run_next,
};
use crate::timer::get_time_us;
use crate::mm::translated_byte_buffer;
use crate::task::current_user_token;
use crate::mm::*;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let _us = get_time_us();
    let buffers = translated_byte_buffer(
        current_user_token(),
        _ts as *const u8,
        core::mem::size_of::<TimeVal>());
    
    let ref ts = TimeVal {
        sec: _us / 1_000_000,
        usec: _us % 1_000_000,
    };

    let ptr = ts as *const TimeVal as *const u8;

    for (idx,buffer) in buffers.into_iter().enumerate() {
        let len = buffer.len();
        buffer.copy_from_slice(unsafe {
            core::slice::from_raw_parts(
                ptr.wrapping_byte_add(idx * len) as *const u8,
                len,
            )
        });
    };
    0
}

pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize{
    if start % PAGE_SIZE != 0 ||
        prot & !0x7 != 0 ||
        prot & 0x7 == 0 ||
        start >= usize::MAX  {
            return -1;
    }
    let start_v: VirtPageNum = VirtAddr::from(start).floor();
    let end_v: VirtPageNum = VirtAddr::from(start + len).ceil();
    let vpns = VPNRange::new(start_v, end_v);
    for vpn in vpns {
       if let Some(pte) = get_current_page_table(vpn) {
            if pte.is_valid() {
                return -1;
            }
       }
    }

    real_mmap(start_v.into(),
             end_v.into(),
             MapPermission::from_bits_truncate((prot << 1) as u8) | MapPermission::U);
    0
}

pub fn sys_munmap(start: usize, len: usize) -> isize{
    let max = usize::MAX;
    if start >= max || start % PAGE_SIZE != 0 {
        return -1;
    }
    
    let mut mlen = len;
    if start > max - len {
        mlen = max - start;
    }
    real_unmap(start, mlen)
}