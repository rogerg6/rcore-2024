# 2024操作系统春季培训

# ch01 
- bare metal app

# ch02
Batch system & Privilege system : 实现一个能自动连续加载运行多个用户程序的批处理操作系统, 实现特权机制隔离用户态和内核态

### 静态程序
我们把app(用户程序)作为程序的data section编译到os里面, 运行的时候一起加载到内存中执行.

### 动态程序
os被加载到0x80200000处, 7个app被加载到0x80400000处
os执行顺序:
[kernel]
1. trap::init()初始化中断向量表
2. batch::init()打印7个app在内存中存放的位置(os data section加载到内存的位置, 不是运行位置)
3. batch::run_next_app(): 开始运行下一个app
     load_app加载下一个app到0x80400000处
     执行__restore(定义在os/src/trap/trap.S)
        TrapContext::app_init_context构建user_ctx
        kernel_stack.push(user_ctx)
        __restore(user_ctx)
            sp -> kernel_stack
            保存kernel_stack上的user_ctx到regs中
            释放kernel_stack
            交换sp, sscratch, 结果sp->user_stack, sscratch->kernel_stack
            sret从S-mode到U-mode
[kernel]
[user]
    ....
    调用系统调用时, 最终会执行ecall(user/src/syscall.rs), 陷入S-mode, 跳到stvec处(中断向量表), 也就是:
[user]
[kernel]
    执行__alltraps(os/src/trap/trap.S)
        交换sp, sscratch, 结果sp->kernel_stack, sscratch->user_stack
        保存regs(用户态的regs)的内容到sp(kernel_stack)上
        trap_handler(kernel_stack)
            根据不同的exception处理(如正常系统调用, 他就读取x10,x11,x12为参数,返回值保存在x10, 这里的参数就是ecall调用时传入到这些reg中的)
            接下来按trap_handler后面就是__restore
    执行__restore(定义在os/src/trap/trap.S)
       __restore(user_ctx)
           sp -> kernel_stack
           保存kernel_stack上的trap_ctx到regs
           释放kernel_stack
           交换sp, sscratch, 结果sp->user_stack, sscratch->kernel_stack
           sret从S-mode到U-mode
    -----
    syscall简而言之:
    kernel_stack = user_ctx; sscra
    trap_handler()
    regs = kernel_stack
    -----
[kernel]
[user]
    执行的syscall返回, 可以从x10中获取syscall的结果, 相当与函数调用(但是和函数调用有本质区别)
    ...
    继续运行app
    ...
    运行结束, 会调用sys_exit, 这也是系统调用, 进入内核态
[user]
[kernel]
    他会直接运行下一个app,直到运行完所有的app后, 调用QEMUExit退出os
[kernel]

    