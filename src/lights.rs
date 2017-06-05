// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use hw_module::{hw_device_t, hw_get_module, hw_module_t};
use std::mem;
use std::os::raw;

pub const LIGHTS_HARDWARE_MODULE_ID: &'static [u8; 7usize] = b"lights\x00";

/**
 * The parameters that can be set for a given light.
 *
 * Not all lights must support all parameters.  If you
 * can do something backward-compatible, you should.
 */
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct light_state_t {
    /**
     * The color of the LED in ARGB.
     *
     * Do your best here.
     *   - If your light can only do red or green, if they ask for blue,
     *     you should do green.
     *   - If you can only do a brightness ramp, then use this formula:
     *      unsigned char brightness = ((77*((color>>16)&0x00ff))
     *              + (150*((color>>8)&0x00ff)) + (29*(color&0x00ff))) >> 8;
     *   - If you can only do on or off, 0 is off, anything else is on.
     *
     * The high byte should be ignored.  Callers will set it to 0xff (which
     * would correspond to 255 alpha).
     */
    pub color: raw::c_uint,
    /**
     * See the LIGHT_FLASH_* constants
     */
    pub flash_mode: raw::c_int,
    pub flash_on_ms: raw::c_int,
    pub flash_off_ms: raw::c_int,
    /**
     * Policy used by the framework to manage the light's brightness.
     * Currently the values are BRIGHTNESS_MODE_USER and BRIGHTNESS_MODE_SENSOR.
     */
    pub brightness_mode: raw::c_int,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct light_device_t {
    pub common: hw_device_t,
    /**
     * Set the provided lights to the provided values.
     *
     * Returns: 0 on succes, error code on failure.
     */
    pub set_light: ::std::option::Option<unsafe extern "C" fn(dev: *mut light_device_t,
                                                                  state: *const light_state_t)
                                                                  -> raw::c_int>,
}

impl Default for light_device_t {
    fn default() -> Self {
        light_device_t {
            common: hw_device_t::default(),
            set_light: None,
        }
    }
}

/// This enum represents the different possible lights.
pub enum LightKind {
    Backlight,
    Keyboard,
    Buttons,
    Battery,
    Notifications,
    Attention,
    Bluetooth,
    Wifi,
}

impl LightKind {
    /// Returns an ascii representation of this light kind suitable
    /// to use when opening a light device.
    fn to_cstr(&self) -> &[u8] {
        match *self {
            LightKind::Backlight => b"backlight\x00",
            LightKind::Keyboard => b"keyboard\x00",
            LightKind::Buttons => b"buttons\x00",
            LightKind::Battery => b"battery\x00",
            LightKind::Notifications => b"notifications\x00",
            LightKind::Attention => b"attention\x00",
            LightKind::Bluetooth => b"bluetooth\x00",
            LightKind::Wifi => b"wifi\x00",
        }
    }
}

#[derive(Clone)]
#[repr(isize)]
pub enum FlashMode {
    NoFlash = 0,
    Timed = 1,
    Hardware = 2,
}

#[derive(Clone)]
#[repr(isize)]
pub enum BrightnessMode {
    User = 0,
    Sensor = 1,
}

/// Bundle of parameters used to set a light value and pattern.
#[derive(Clone)]
pub struct LightState {
    pub color: (u8, u8, u8), // RGB
    pub flash_mode: FlashMode,
    pub flash_on_ms: isize,
    pub flash_off_ms: isize,
    pub brightness_mode: BrightnessMode,
}

impl Default for LightState {
    /// Default constructor, a black non-blinking value.
    fn default() -> Self {
        LightState {
            color: (0, 0, 0),
            flash_mode: FlashMode::NoFlash,
            flash_on_ms: 0,
            flash_off_ms: 0,
            brightness_mode: BrightnessMode::User,
        }
    }
}

impl LightState {
    fn as_native(&self) -> light_state_t {
        light_state_t {
            color: ((0xff as u32) << 24) | ((self.color.0 as u32) << 16) |
                   ((self.color.1 as u32) << 8) | self.color.2 as u32,
            flash_mode: self.flash_mode.clone() as i32,
            flash_on_ms: self.flash_on_ms as i32,
            flash_off_ms: self.flash_off_ms as i32,
            brightness_mode: self.brightness_mode.clone() as i32,
        }
    }
}

/// A device attached to one light.
pub struct LightsDevice {
    device: Box<light_device_t>,
}

impl LightsDevice {
    /// Setup a display color and blinking pattern for this light.
    /// Returns true if successful.
    pub fn set(&self, state: LightState) -> bool {
        if let Some(set_light) = self.device.set_light {
            return unsafe { set_light(Box::into_raw(self.device.clone()), &state.as_native()) } ==
                   0;
        }
        false
    }

    /// Turn this light off.
    /// Returns true if successful.
    pub fn off(&self) -> bool {
        self.set(LightState::default())
    }
}

/// The lights module provides access to the Lights devices.
#[derive(Clone)]
pub struct LightsModule {
    module: Box<hw_module_t>,
}

impl LightsModule {
    /// Instanciates a lights module, or return None if the device
    /// doesn't support lights at all.
    pub fn new() -> Option<Self> {
        unsafe {
            let mut module: Box<hw_module_t> = mem::uninitialized();
            if hw_get_module(LIGHTS_HARDWARE_MODULE_ID.as_ptr(),
                             &mut module as *mut _ as *mut _) == 0 {
                Some(LightsModule { module: module })
            } else {
                None
            }
        }
    }

    /// Returns the specified light device, or None if this particular light
    /// is not supported.
    pub fn get_device(&self, light: LightKind) -> Option<LightsDevice> {
        unsafe {
            let mut device: Box<light_device_t> = mem::uninitialized();

            if let Some(open) = (*self.module.methods).open {
                if open(Box::into_raw(self.clone().module),
                        light.to_cstr().as_ptr(),
                        &mut device as *mut _ as *mut _) == 0 {
                    Some(LightsDevice { device: device })
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
