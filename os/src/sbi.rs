//use core::arch::asm;

//const SBI_CONSOLE_PUTCHAR: usize = 1;
//
//#[inline(always)]
//fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
//    let mut ret;
//    unsafe {
//        asm!(
//            // 具有用户态到内核态到执行环境切换能力的函数调用指令
//            "ecall",
//            // 寄存器a0, inlateout 表示此寄存器只保存输入参数也保存系统调用后的返回值
//            // 并且使用 ${in} => ${out} 表示，其中前面放置输入的参数，后面放置输出参数
//            inlateout("x10") arg0 => ret,
//            // 寄存器a1, in 表示只保存输入参数
//            in("x11") arg1,
//            // 寄存器a2, 输入参数
//            in("x12") arg2,
//            // 寄存器a7, 系统调用which
//            in("x17") which,
//        )
//    }
//    ret
//}

pub fn console_putchar(c: usize) {
    //sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
