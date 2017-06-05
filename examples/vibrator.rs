// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate gonkhal;

use gonkhal::Vibrator;
use std::{thread, time};

fn main() {
    println!("GonkHal vibrator demo...");

    if let Some(vibrator) = Vibrator::new() {
        println!("Vibrating for one second...");
        vibrator.on(1000);
        thread::sleep(time::Duration::from_millis(1500));

        println!("Sending a morse code S.O.S");
        Vibrator::pattern(&vibrator,
                          vec![100, 30, 100, 30, 100, 200, 200, 30, 200, 30, 200, 200, 100, 30,
                               100, 30, 100]);
        thread::sleep(time::Duration::from_millis(2000));

        println!("Start a 3s vibration but cancel it after 1s.");
        let mut guard = Vibrator::pattern(&vibrator, vec![3000]);
        thread::sleep(time::Duration::from_millis(1000));
        guard.cancel();
        println!("Should stop now!");
        thread::sleep(time::Duration::from_millis(3000));

        println!("All done.");
    } else {
        println!("This device doesn't have a vibrator.");
    }
}
