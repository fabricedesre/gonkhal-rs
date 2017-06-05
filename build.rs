// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This build script set up the link time parameters needed to find
//! the android libraries.

use std::env;

fn get_env_var(name: &str, reason: &str) -> String {
    match env::var(name) {
        Ok(val) => val,
        Err(_) => {
            println!("warning=Error! Set the '{}' environment variable {}.",
                     name,
                     reason);
            panic!("kaboom");
        }
    }
}

fn main() {
    // Check that the GONK_HOME and GONK_DEVICE environment variable are set.
    let gonk_home = get_env_var("GONK_HOME", "to the path of your Gonk source directory");
    let gonk_device = get_env_var("GONK_DEVICE",
                                  "to the codename of your device (eg. aries for a z3c)");

    // Set the linker path.
    println!("cargo:rustc-flags=-L {}/out/target/product/{}/obj/lib/",
             gonk_home,
             gonk_device);

    // Update PATH to find the linker specified in .cargo/config
    let current_path = env::var("PATH").unwrap_or("".to_owned());
    env::set_var("PATH",
                 format!("{}/prebuilts/gcc/linux-x86/arm/arm-linux-androideabi-4.9/bin/:{}",
                         gonk_home,
                         current_path));
}
