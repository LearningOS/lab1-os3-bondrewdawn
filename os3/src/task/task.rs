//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add Whatever you need about the Task
    // pub task_begin_time: usize,
    // pub syscall_times: [u32; MAX_SYSCALL_NUM],
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Running, Exited
pub enum TaskStatus {
    UnInit, // Not init
    Ready,  // ready for run
    Running,// running
    Exited, // exited
}
