//! PID Control Loop
//!
//! The implementation is based on a brief description written in the Kiel CMSIS library on their
//! publicly available documentation.

use embedded_hal::PwmPin;
use nb;


/// Process controlled by a PID controller
pub trait Process {
    fn set(&mut self, value: u16);
}

/// PID Constants
pub struct Constants {
    a0: i32,
    a1: i32,
    a2: i32,
}

impl Constants {
    /// Creates a new constants structure
    pub fn new(p: u16, i: u16, d:u16) -> Self {
        let p_32 = p as i32;
        let i_32 = i as i32;
        let d_32 = d as i32;
        Constants {
            a0: p_32 + i_32 + d_32,
            a1: (-p_32) - (2*d_32),
            a2: d_32,
        }
    }
}

/// PID controller for a process
pub struct PID<P> where P: PwmPin<Duty=u16> {
    k: Constants,
    process: P,
    last_value: u16,
    last_feedback: [u16; 2],
}

impl<P: PwmPin<Duty=u16>> PID<P> {
    /// Creates a new PID controller that controls a process
    pub fn new(k: Constants, process: P) -> Self {
        PID {
            k: k,
            process: process,
            last_value: 0,
            last_feedback: [0; 2],
        }
    }

    /// Runs an iteration of the PID control loop
    pub fn step(&mut self, feedback: u16) {
        let fb_n = feedback as i32;
        let fb_n1 = self.last_feedback[0] as i32;
        let fb_n2 = self.last_feedback[1] as i32;
        let v_n1 = self.last_value as i32;
        let mut value = v_n1 + self.k.a0 * fb_n + self.k.a1 * fb_n1 + self.k.a2 * fb_n2;
        if value > self.process.get_max_duty() as i32 {
            value = self.process.get_max_duty() as i32;
        }
        else if value < 0 {
            value = 0;
        }
        self.process.set_duty(value as u16);
        self.last_value = value as u16;
        self.last_feedback = [fb_n as u16, fb_n1 as u16];
    }
}
