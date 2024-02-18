use crate::batch::{is_in_code_space, USER_STACK};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let start = buf as usize;
            let end = start + len - 1;
            let in_code_space = is_in_code_space(start) && is_in_code_space(end);
            let in_stack = USER_STACK.is_in_stack(start) && USER_STACK.is_in_stack(end);
            if !in_stack && !in_code_space {
                // [buf; len] exceeds current app space
                return -1;
            }
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            // Unsupported fd in sys_write
            -1
        }
    }
}
