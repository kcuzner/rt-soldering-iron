//! Minimal RTOS, based loosly on kos-avr

use core::u32;

pub struct Task<'a> {
    func: &'static Fn(),
    next: Option<&'a mut Task<'a>>
}

impl<'a> Task<'a> {
    pub fn new(f: &'static Fn()) -> Task<'a> {
        Task {
            func: f,
            next: None
        }
    }
}

pub struct TaskGroup<'a> {
    head: Option<&'a mut Task<'a>>
}

impl<'a> TaskGroup<'a> {
    pub fn new() -> TaskGroup<'a> {
        TaskGroup {
            head: None
        }
    }
    pub fn add_task(mut self, tcb: &'a mut Task<'a>) -> TaskGroup<'a> {
        match self.head {
            None => { self.head = Some(tcb); self },
            Some(h) => { tcb.next = Some(h); self.head = Some(tcb); self }
        }
    }
    pub fn run(mut self) {
    }
}

