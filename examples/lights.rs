// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate gonkhal;

use gonkhal::{BrightnessMode, FlashMode, LightKind, LightState, LightsModule};
use std::{thread, time};

fn main() {
    println!("GonkHal lights demo...");

    if let Some(module) = LightsModule::new() {
        let colors = vec![(255, 0, 0), (0, 255, 0), (0, 0, 255), (0, 0, 0)];

        for color in colors {
            let state = LightState {
                color: color,
                flash_mode: FlashMode::Timed,
                flash_on_ms: 500,
                flash_off_ms: 500,
                brightness_mode: BrightnessMode::User,
            };

            if let Some(device) = module.get_device(LightKind::Attention) {
                if color.0 + color.1 + color.2 == 0 {
                    device.off();
                } else {
                    println!(
                        "Blinking in #{:02x}{:02x}{:02x} for 2 seconds",
                        color.0, color.1, color.2
                    );
                    device.set(state.clone());
                }
            }

            if let Some(device) = module.get_device(LightKind::Attention) {
                if color.0 + color.1 + color.2 == 0 {
                    device.off();
                } else {
                    device.set(state.clone());
                    thread::sleep(time::Duration::from_millis(2000));
                }
            }
        }
    } else {
        println!("This device doesn't have a lights module.");
    }
}
