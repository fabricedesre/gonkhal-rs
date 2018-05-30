// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ffi::CString;
use std::os::raw::{c_char, c_int};

// Based on hardware/libhardware_legacy/include/hardware_legacy/wifi.h
#[link(name = "hardware_legacy")]
extern "C" {
    /**
     * Load the Wi-Fi driver.
     *
     * @return 0 on success, < 0 on failure.
     */
    pub fn wifi_load_driver() -> c_int;

    /**
     * Unload the Wi-Fi driver.
     *
     * @return 0 on success, < 0 on failure.
     */
    pub fn wifi_unload_driver() -> c_int;

    /**
     * Check if the Wi-Fi driver is loaded.
     *
     * @return 0 on success, < 0 on failure.
     */
    pub fn is_wifi_driver_loaded() -> c_int;

    /**
     * Start supplicant.
     *
     * @return 0 on success, < 0 on failure.
     */
    pub fn wifi_start_supplicant(p2pSupported: c_int) -> c_int;

    /**
     * Stop supplicant.
     *
     * @return 0 on success, < 0 on failure.
     */
    pub fn wifi_stop_supplicant(p2pSupported: c_int) -> c_int;

    /**
     * Open a connection to supplicant
     *
     * @return 0 on success, < 0 on failure.
     */
    pub fn wifi_connect_to_supplicant() -> c_int;

    /**
     * Close connection to supplicant
     *
     * @return 0 on success, < 0 on failure.
     */
    pub fn wifi_close_supplicant_connection();

    /**
     * wifi_wait_for_event() performs a blocking call to
     * get a Wi-Fi event and returns a string representing
     * a Wi-Fi event when it occurs.
     *
     * @param buf is the buffer that receives the event
     * @param len is the maximum length of the buffer
     *
     * @returns number of bytes in buffer, 0 if no
     * event (for instance, no connection), and less than 0
     * if there is an error.
     */
    pub fn wifi_wait_for_event(buf: *mut c_char, len: usize) -> c_int;

    /**
     * wifi_command() issues a command to the Wi-Fi driver.
     *
     * Android extends the standard commands listed at
     * http://hostap.epitest.fi/wpa_supplicant/devel/ctrl_iface_page.html
     * to include support for sending commands to the driver:
     *
     * See wifi/java/android/net/wifi/WifiNative.java for the details of
     * driver commands that are supported
     *
     * @param command is the string command (preallocated with 32 bytes)
     * @param commandlen is command buffer length
     * @param reply is a buffer to receive a reply string
     * @param reply_len on entry, this is the maximum length of
     *        the reply buffer. On exit, the number of
     *        bytes in the reply buffer.
     *
     * @return 0 if successful, < 0 if an error.
     */
    pub fn wifi_command(command: *const c_char, reply: *mut c_char, reply_len: *mut usize)
        -> c_int;
}

/// A stateless Wifi driver.
pub struct Wifi;

impl Wifi {
    /// Check if the Wifi driver is loaded.
    pub fn is_driver_loaded() -> bool {
        unsafe { is_wifi_driver_loaded() > 0 }
    }

    /// Load the Wifi driver.
    pub fn load_driver() -> Result<(), c_int> {
        match unsafe { wifi_load_driver() } {
            0 => Ok(()),
            err => Err(err),
        }
    }

    /// Unload the Wifi driver.
    pub fn unload_driver() -> Result<(), c_int> {
        match unsafe { wifi_unload_driver() } {
            0 => Ok(()),
            err => Err(err),
        }
    }

    /// Start the supplicant.
    pub fn start_supplicant(p2p_supported: bool) -> Result<(), c_int> {
        match unsafe { wifi_start_supplicant(p2p_supported as c_int) } {
            0 => Ok(()),
            err => Err(err),
        }
    }

    /// Stop the supplicant.
    pub fn stop_supplicant(p2p_supported: bool) -> Result<(), c_int> {
        match unsafe { wifi_stop_supplicant(p2p_supported as c_int) } {
            0 => Ok(()),
            err => Err(err),
        }
    }

    /// Open a connection to supplicant.
    pub fn connect_to_supplicant() -> Result<(), c_int> {
        match unsafe { wifi_connect_to_supplicant() } {
            0 => Ok(()),
            err => Err(err),
        }
    }

    /// Close connection to supplicant.
    pub fn close_supplicant_connection() {
        unsafe { wifi_close_supplicant_connection() };
    }

    /// Performs a blocking call to get a Wi-Fi event and returns a string
    /// representing a Wi-Fi event when it occurs.
    pub fn wait_for_event() -> Result<String, ()> {
        // Use a 4k buffer.
        let mut buffer: [c_char; 4096] = [0; 4096];
        let res = unsafe { wifi_wait_for_event(buffer.as_mut_ptr(), 4096) };

        // Some error occured...
        if res < 0 {
            return Err(());
        }

        // Either no event, or invalid buffer size.
        if res == 0 || res > 4096 {
            return Err(());
        }

        // Do a lossy conversion since we have no guarantee that the input
        // will be valid utf8.
        Ok(String::from_utf8_lossy(&buffer[..res as usize]).to_string())
    }

    ///  Issues a command to the Wi-Fi driver.
    ///
    ///  Android extends the standard commands listed at
    ///  /link http://hostap.epitest.fi/wpa_supplicant/devel/ctrl_iface_page.html
    ///  to include support for sending commands to the driver:
    ///
    ///  See wifi/java/android/net/wifi/WifiNative.java for the details of
    ///  driver commands that are supported
    pub fn command(command: &str) -> Result<String, c_int> {
        let mut buffer: [c_char; 4096] = [0; 4096];
        let mut buff_size: usize = 4096;

        // turn command into a C string suitable for ffi.
        let cmd = CString::new(command).expect(&format!("Malformed wifi command: {}", command));

        let res = unsafe { wifi_command(cmd.as_ptr(), buffer.as_mut_ptr(), &mut buff_size) };

        // Some error occured...
        if res < 0 {
            return Err(res);
        }

        // Either no response, or invalid buffer size.
        if buff_size == 0 || buff_size > 4096 {
            println!("buff_size error {}", buff_size);
            return Err(-10000); // TODO: check if that's actually safe to use a custom error value.
        }

        // Do a lossy conversion since we have no guarantee that the input
        // will be valid utf8.
        Ok(String::from_utf8_lossy(&buffer[..buff_size]).to_string())
    }
}
