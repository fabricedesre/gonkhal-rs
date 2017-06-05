// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::os::raw::c_int;
use std::{thread, time};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[link(name = "hardware_legacy")]
extern "C" {
    // Return whether the device has a vibrator.
    // @return 1 if a vibrator exists, 0 if it doesn't.
    fn vibrator_exists() -> c_int;

    // Turn on vibrator
    // @param timeout_ms number of milliseconds to vibrate
    // @return 0 if successful, -1 if error
    fn vibrator_on(timeout_ms: c_int) -> c_int;

    // Turn off vibrator
    // @return 0 if successful, -1 if error
    fn vibrator_off() -> c_int;
}

#[derive(Clone)]
pub struct Vibrator;

#[derive(Clone)]
pub struct PatternGuard {
    canceled: Arc<AtomicBool>,
}

impl Default for PatternGuard {
    fn default() -> Self {
        PatternGuard { canceled: Arc::new(AtomicBool::new(false)) }
    }
}

impl PatternGuard {
    /// Cancels an ongoing vibration pattern. This will immediately turn
    /// the vibrator off and exit the pattern thread as soon as possible.
    pub fn cancel(&mut self) {
        unsafe {
            if vibrator_exists() == 1 {
                vibrator_off();
            }
        }
        self.canceled.store(true, Ordering::Relaxed);
    }

    /// Checks if this pattern guard has been canceled.
    pub fn is_canceled(&self) -> bool {
        self.canceled.load(Ordering::Relaxed)
    }
}

impl Vibrator {
    /// Creates a `Vibrator` if the hardware supports it, or None.
    pub fn new() -> Option<Self> {
        if unsafe { vibrator_exists() } == 1 {
            Some(Vibrator)
        } else {
            None
        }
    }

    /// Turns the vibrator on for some period of time.
    /// Returns true if successful.
    pub fn on(&self, timeout_ms: isize) -> bool {
        unsafe { vibrator_on(timeout_ms as c_int) == 0 }
    }

    /// Turns the vibrator off.
    /// Returns true if successful.
    pub fn off(&self) -> bool {
        unsafe { vibrator_off() == 0 }
    }

    /// Vibrates according to a pattern of `on, off` sequence.
    /// This happens on a different thread
    pub fn pattern(vibrator: &Vibrator, pattern: Vec<isize>) -> PatternGuard {
        let v = vibrator.clone();

        let guard = PatternGuard::default();
        let g = guard.clone();
        thread::Builder::new()
            .name("vibrator".to_owned())
            .spawn(move || {
                for (i, val) in pattern.iter().enumerate() {

                    // Early return if this pattern has been canceled.
                    if g.is_canceled() {
                        return;
                    }

                    if i % 2 == 0 {
                        v.on(*val);
                    }
                    // In all cases, wait for the expected duration since on()
                    // is not a blocking call.
                    thread::sleep(time::Duration::from_millis(*val as u64));
                }
            })
            .expect("Failed to start vibrator thread!");

        guard
    }
}
