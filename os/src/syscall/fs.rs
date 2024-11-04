const FD_STDOUT: usize = 1;
const USER_STACK_SIZE: usize = 4096 ;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;
use crate::batch::get_user_stack_sp;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            if(((buf as usize)  >= get_user_stack_sp() - USER_STACK_SIZE) && ((buf as usize) + len <= get_user_stack_sp())) 
            || (((buf as usize) + len <= APP_SIZE_LIMIT + APP_BASE_ADDRESS) && ((buf as usize) >= APP_BASE_ADDRESS)){
                let slice = unsafe { core::slice::from_raw_parts(buf, len) };
                let str = core::str::from_utf8(slice).unwrap();
                print!("{}", str);
                len as isize
            } else {
                -1
            }
        },
        _ => {
            //panic!("Unsupported fd in sys_write!");
            -1
        }
    }
}