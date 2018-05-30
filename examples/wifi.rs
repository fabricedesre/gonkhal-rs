// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate gonkhal;

use gonkhal::Wifi;

fn send_command(command: &str) {
    match Wifi::command(&format!("IFNAME=wlan0 {}", command)) {
        Err(err) => println!("Error sending `{}`: {}", command, err),
        Ok(response) => println!("Response: {}", response),
    }
}

fn main() {
    println!("GonkHal wifi demo...");

    send_command("LOGLEVEL DEBUG");

    if !Wifi::is_driver_loaded() {
        println!("Loading Wifi driver...");
        Wifi::load_driver().expect("Failed to load Wifi driver");
        if !Wifi::is_driver_loaded() {
            println!("Wifi driver is still not loaded, aborting :(");
            return;
        }
    } else {
        println!("Wifi driver already loaded.")
    }

    match Wifi::start_supplicant(false) {
        Ok(()) => println!("Supplicant started."),
        Err(code) => {
            println!("Failed to start supplicant: err={}", code);
            return;
        }
    }

    match Wifi::connect_to_supplicant() {
        Ok(()) => println!("Connected to supplicant."),
        Err(code) => {
            println!("Failed to connect to supplicant: err={}", code);
            return;
        }
    }

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
