// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ffi::CString;
use std::os::raw;

#[link(name = "hardware_legacy")]
extern "C" {
    pub fn acquire_wake_lock(lock: raw::c_int, id: *const raw::c_char) -> raw::c_int;

    pub fn release_wake_lock(id: *const raw::c_char) -> raw::c_int;
}

/// The two kind of wake locks supported.
pub enum WakelockLevel {
    /// The cpu stays on, but the screen is off.
    Partial = 1,
    /// The cpu and the screen stay on.
    Full = 2,
}

/// A Wakelock that can be manually released, or that will
/// release itself when dropped.
pub struct Wakelock {
    name: String,
}

impl Wakelock {
    /// Creates a new Wakelock with the given name and level.
    pub fn new(name: &str, level: WakelockLevel) -> Option<Wakelock> {
        let id = CString::new(name).expect(&format!("Malformed lock name: {}", name));
        let res = unsafe { acquire_wake_lock(level as raw::c_int, id.as_ptr()) };
        if res == 0 {
            return Some(Wakelock { name: name.to_owned() });
        }
        None
    }

    /// Release the Wakelock.
    pub fn release(&self) {
        let id = CString::new(self.name.clone())
            .expect(&format!("Malformed lock name: {}", self.name));
        unsafe {
            release_wake_lock(id.as_ptr());
        }
    }
}

impl Drop for Wakelock {
    /// Release the Wakelock.
    fn drop(&mut self) {
        self.release();
    }
}
