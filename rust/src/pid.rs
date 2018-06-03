//! PID Control Loop
//!
//! The implementation is based on a brief description written in the Kiel CMSIS library on their
//! publicly available documentation.

use embedded_hal::PwmPin;
use nb;
use cast::u16;
use fpa::I24F8;

/// Process controlled by a PID controller
pub trait Process {
    fn set(&mut self, value: u16);
}

/// PID Constants
pub struct Constants {
    a0: I24F8,
    a1: I24F8,
    a2: I24F8,
}

impl Constants {
    /// Creates a new constants structure
    pub fn new(p: f64, i: f64, d: f64) -> Self {
        let p_fxp = I24F8(p).unwrap();
        let i_fxp = I24F8(i).unwrap();
        let d_fxp = I24F8(d).unwrap();
        Constants {
            a0: p_fxp + i_fxp + d_fxp,
            a1: (-p_fxp) - (2*d_fxp),
            a2: d_fxp,
        }
    }
}

/// PID controller for a process
pub struct PID<P> where P: PwmPin<Duty=u16> {
    k: Constants,
    process: P,
    last_value: I24F8,
    last_feedback: [I24F8; 2],
}

impl<P: PwmPin<Duty=u16>> PID<P> {
    /// Creates a new PID controller that controls a process
    pub fn new(k: Constants, process: P) -> Self {
        PID {
            k: k,
            process: process,
            last_value: I24F8(0i8),
            last_feedback: [I24F8(0i8); 2],
        }
    }

    /// Runs an iteration of the PID control loop
    pub fn step(&mut self, setpoint: u16, feedback: u16) -> u16 {
        let fb_n = I24F8(setpoint) - I24F8(feedback);
        let fb_n1 = self.last_feedback[0];
        let fb_n2 = self.last_feedback[1];
        let v_n1 = self.last_value;
        let mut value = v_n1 + self.k.a0 * fb_n + self.k.a1 * fb_n1 + self.k.a2 * fb_n2;
        if value > I24F8(self.process.get_max_duty()) {
            value = I24F8(self.process.get_max_duty());
        }
        else if value < I24F8(0i8) {
            value = I24F8(0i8);
        }
        self.process.set_duty(u16(value).unwrap());
        self.last_value = value;
        self.last_feedback = [fb_n, fb_n1];
        u16(value).unwrap()
    }
}

