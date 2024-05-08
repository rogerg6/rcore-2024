use core::arch::asm;

const SBI_CONSOLE_PUTCHAR: usize = 1;

fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "li x16, 0",
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

// const SBI_SHUTDOWN: usize = 8;

// pub fn shutdown() -> ! {
//     sbi_call(SBI_SHUTDOWN, 0, 0, 0);
//     panic!("It should be shutdown!");
// }
use crate::board::QEMUExit;
pub fn shutdown() -> ! {
    crate::board::QEMU_EXIT_HANDLE.exit_failure();
}
