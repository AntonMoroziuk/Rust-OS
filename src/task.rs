// During context switch we push 8 registers to stack
const CONTEXT_SIZE: usize = 8;

static tasks: [Option<Box<[u32]>>; TASK_LIMIT] = [None; TASK_LIMIT];
static stacks: [Option<&mut [u32]>; TASK_LIMIT] = [None; TASK_LIMIT];
static mut cur_task: usize = 0;

pub fn task_delete() {
    let temp = tasks[cur_task].take().unwrap();

    for i in cur_task..TASK_LIMIT - 1 {
        tasks[i] = tasks[i + 1].take();
        stacks[i] = stacks[i + 1].take();
    }

    tasks[TASK_LIMIT - 1] = None;
    stacks[TASK_LIMIT - 1] = None;
    drop(temp);
    exit();
}

pub fn task_add(task_code: fn(), stack_size: usize) -> Result<(), &'static str> {
    let i = tasks.iter().position(|task| task.is_none())
        .ok_or("Task limit reached")?;

    let task = vec![0u32; stack_size].into_boxed_slice();
    tasks[i] = Some(task);

    /*
     * During first context switch we will pop 8 registers
     * and pc register from stack, so we need space for them
     * */
    let stack = &mut tasks[i].as_mut().unwrap()[..stack_size - CONTEXT_SIZE];
    stack[stack.len()] = task_code as u32;
    stacks[i] = Some(stack);
    Ok(())
}

pub fn task_scheduler() {
    loop {
        unsafe {
            cur_task = 0;
            while let Some(stack) = stacks[cur_task] {
                activate(stack);
                cur_task += 1;
            }
        }
    }
}

pub fn activate(stack: &mut [u32]) {
    unsafe {
        asm!("ldr r1, [r0]");
        asm!("msr psp, r1")
        asm!("push {r0-r7, lr}")

        asm!("ldr r1, =task_delete")
        asm!("mov lr, r1")

        /* Use process stack pointer */
        asm!("mrs r1, control")
        asm!("add r1, #2")
        asm!("msr control, r1")

        asm!("pop {r0-r7, pc}")
    }
}
