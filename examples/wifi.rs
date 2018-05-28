// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate gonkhal;

use gonkhal::Wifi;

fn send_command(command: &str) {
    match Wifi::command(command) {
        Err(_) => println!("Error sending `{}`", command),
        Ok(response) => println!("Response: {}", response),
    }
}

fn main() {
    println!("GonkHal wifi demo...");

    if !Wifi::is_driver_loaded() {
        println!("Loading Wifi driver...");
        Wifi::load_driver();
        if !Wifi::is_driver_loaded() {
            println!("Failed to load driver, aborting :(");
            return;
        }
    } else {
        println!("Wifi driver already loaded.")
    }

    let started = Wifi::start_supplicant(false);
    println!("Supplicant starting status: {}", started);

    let connected = Wifi::connect_to_supplicant();
    println!("Connecting to supplicant status: {}", connected);

    fn get_event() {
        match Wifi::wait_for_event() {
            Err(_) => println!("Error getting event!"),
            Ok(event) => {
                if !event.ends_with("CTRL-EVENT-TERMINATING  - connection closed") {
                    println!("Event: {}", event);
                }
            }
        }
    }

    let commands = ["STATUS", "SCAN TYPE=ONLY", "GET_NETWORKS"];

    for command in commands.iter() {
        println!("-> Send {}", command);
        send_command(command);
    }

    loop {
        get_event();
    }
}
