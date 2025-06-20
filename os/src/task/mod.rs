mod context;
mod manager;
mod pid;
mod processor;
mod switch;
#[allow(clippy::module_inception)]
mod task;

use crate::loader::get_app_data_by_name;
use crate::sbi::shutdown;
use alloc::sync::Arc;
use lazy_static::*;
pub use manager::{TaskManager, fetch_task};
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;
pub use manager::add_task;
pub use pid::{KernelStack, PidAllocator, PidHandle, pid_alloc};
pub use processor::{
    Processor, current_task, current_trap_cx, current_user_token, run_tasks, schedule,
    take_current_task,
};
/// Suspend the current 'Running' task and run the next task in task list.
pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current PCB

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

/// pid of usertests app in make run TEST=1
pub const IDLE_PID: usize = 0;

/// Exit the current 'Running' task and run the next task in task list.
pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();

    let pid = task.getpid();
    if pid == IDLE_PID {
        println!(
            "[kernel] Idle process exit with exit_code {} ...",
            exit_code
        );
        if exit_code != 0 {
            //crate::sbi::shutdown(255); //255 == -1 for err hint
            shutdown(true)
        } else {
            //crate::sbi::shutdown(0); //0 for success hint
            shutdown(false)
        }
    }

    // **** access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // do not move to its parent but under initproc

    // ++++++ access initproc TCB exclusively
    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ release parent PCB

    inner.children.clear();
    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // **** release current PCB
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    schedule(&mut _unused as *mut _);
}

lazy_static! {
    ///Globle process that init user shell
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("initproc").unwrap()
    ));
}
///Add init process to the manager
pub fn add_initproc() {
    add_task(INITPROC.clone());
    // i want to see how 0x132f0 be translated to a Physical Address
    println!("[Debug] Info of 0x13000 in Initproc:");
    println!("[Debug] is_valid {}", INITPROC.inner_exclusive_access().memory_set.translate(
        crate::mm::VirtPageNum::from(crate::mm::VirtAddr::from(0x13000 as usize))
    ).unwrap().is_valid());
    println!("[Debug] readable {}", INITPROC.inner_exclusive_access().memory_set.translate(
        crate::mm::VirtPageNum::from(crate::mm::VirtAddr::from(0x13000 as usize))
    ).unwrap().readable());
    println!("[Debug] writable {}", INITPROC.inner_exclusive_access().memory_set.translate(
        crate::mm::VirtPageNum::from(crate::mm::VirtAddr::from(0x13000 as usize))
    ).unwrap().writable());
    println!("[Debug] executable {}", INITPROC.inner_exclusive_access().memory_set.translate(
        crate::mm::VirtPageNum::from(crate::mm::VirtAddr::from(0x13000 as usize))
    ).unwrap().executable());
    println!("[Debug] ppn {}", INITPROC.inner_exclusive_access().memory_set.translate(
        crate::mm::VirtPageNum::from(crate::mm::VirtAddr::from(0x13000 as usize))
    ).unwrap().ppn().0);

}