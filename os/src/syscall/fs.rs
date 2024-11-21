const FD_STDOUT: usize = 1;
const USER_STACK_SIZE: usize = 4096 ;
const APP_SIZE_LIMIT: usize = 0x20000;
use crate::task::*;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let app_address = get_task_address();
    let sp = get_user_stack_sp();
    match fd {
        FD_STDOUT => {
            if(((buf as usize)  >= sp - USER_STACK_SIZE) && ((buf as usize) + len <= sp)) 
            || (((buf as usize) + len <= APP_SIZE_LIMIT + app_address) && ((buf as usize) >= app_address)){
                let slice = unsafe { core::slice::from_raw_parts(buf, len) };
                let str = core::str::from_utf8(slice).unwrap();
                print!("{}", str);
                len as isize
            } else {
                -1
            }
        },
        _ => {
            -1
        }
    }
}