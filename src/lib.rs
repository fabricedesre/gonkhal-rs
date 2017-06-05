// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This crate provides access to features of the gonk HAL

mod vibrator;
mod hw_module;
mod lights;

pub use vibrator::{PatternGuard, Vibrator};
pub use lights::{LightsModule, LightsDevice, LightKind, LightState, BrightnessMode, FlashMode};
