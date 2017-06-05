// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::os::raw;
use std::ptr;

/**
 * Every hardware module must have a data structure named `HAL_MODULE_INFO_SYM`
 * and the fields of this data structure must begin with `hw_module_t`
 * followed by module specific information.
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hw_module_t {
    /** tag must be initialized to HARDWARE_MODULE_TAG */
    pub tag: u32,
    /**
     * The API version of the implemented module. The module owner is
     * responsible for updating the version when a module interface has
     * changed.
     *
     * The derived modules such as gralloc and audio own and manage this field.
     * The module user must interpret the version field to decide whether or
     * not to inter-operate with the supplied module implementation.
     * For example, SurfaceFlinger is responsible for making sure that
     * it knows how to manage different versions of the gralloc-module API,
     * and AudioFlinger must know how to do the same for audio-module API.
     *
     * The module API version should include a major and a minor component.
     * For example, version 1.0 could be represented as 0x0100. This format
     * implies that versions 0x0100-0x01ff are all API-compatible.
     *
     * In the future, libhardware will expose a hw_get_module_version()
     * (or equivalent) function that will take minimum/maximum supported
     * versions as arguments and would be able to reject modules with
     * versions outside of the supplied range.
     */
    pub module_api_version: u16,
    /**
     * The API version of the HAL module interface. This is meant to
     * version the hw_module_t, hw_module_methods_t, and hw_device_t
     * structures and definitions.
     *
     * The HAL interface owns this field. Module users/implementations
     * must NOT rely on this value for version information.
     *
     * Presently, 0 is the only valid value.
     */
    pub hal_api_version: u16,
    /** Identifier of module */
    pub id: *const raw::c_char,
    /** Name of this module */
    pub name: *const raw::c_char,
    /** Author/owner/implementor of the module */
    pub author: *const raw::c_char,
    /** Modules methods */
    pub methods: *mut hw_module_methods_t,
    /** module's dso */
    pub dso: *mut raw::c_void,
    /** padding to 128 bytes, reserved for future use */
    pub reserved: [u32; 25usize],
}

impl Default for hw_module_t {
    fn default() -> Self {
        hw_module_t {
            tag: 0,
            module_api_version: 0,
            hal_api_version: 0,
            id: ptr::null_mut(),
            name: ptr::null_mut(),
            author: ptr::null_mut(),
            methods: ptr::null_mut(),
            dso: ptr::null_mut(),
            reserved: [0; 25],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hw_module_methods_t {
    /** Open a specific device */
    pub open: ::std::option::Option<unsafe extern "C" fn(module: *const hw_module_t,
                                                             id: *const raw::c_char,
                                                             device: *mut *mut hw_device_t)
                                                             -> raw::c_int>,
}

/**
 * Every device data structure must begin with `hw_device_t`
 * followed by module specific public methods and attributes.
 */
#[repr(C)]
#[derive(Clone, Debug)]
pub struct hw_device_t {
    /** tag must be initialized to HARDWARE_DEVICE_TAG */
    pub tag: u32,
    /**
     * Version of the module-specific device API. This value is used by
     * the derived-module user to manage different device implementations.
     *
     * The module user is responsible for checking the module_api_version
     * and device version fields to ensure that the user is capable of
     * communicating with the specific module implementation.
     *
     * One module can support multiple devices with different versions. This
     * can be useful when a device interface changes in an incompatible way
     * but it is still necessary to support older implementations at the same
     * time. One such example is the Camera 2.0 API.
     *
     * This field is interpreted by the module user and is ignored by the
     * HAL interface itself.
     */
    pub version: u32,
    /** reference to the module this device belongs to */
    pub module: *mut hw_module_t,
    /** padding reserved for future use */
    pub reserved: [u32; 12usize],
    /** Close this device */
    pub close:
        ::std::option::Option<unsafe extern "C" fn(device: *mut hw_device_t) -> raw::c_int>,
}

impl Default for hw_device_t {
    fn default() -> Self {
        hw_device_t {
            tag: 0,
            version: 0,
            module: ptr::null_mut(),
            reserved: [0; 12usize],
            close: None,
        }
    }
}

impl Drop for hw_device_t {
    fn drop(&mut self) {
        if let Some(close) = self.close {
            unsafe {
                close(self);
            }
        }
    }
}

#[link(name = "hardware")]
extern "C" {
    /**
     * Get the module info associated with a module by id.
     *
     * @return: 0 == success, <0 == error and *module == NULL
     */
    pub fn hw_get_module(id: *const raw::c_char, module: *mut *mut hw_module_t) -> raw::c_int;

    /**
     * Get the module info associated with a module instance by class 'class_id'
     * and instance 'inst'.
     *
     * Some modules types necessitate multiple instances. For example audio supports
     * multiple concurrent interfaces and thus 'audio' is the module class
     * and 'primary' or 'a2dp' are module interfaces. This implies that the files
     * providing these modules would be named audio.primary.<variant>.so and
     * audio.a2dp.<variant>.so
     *
     * @return: 0 == success, <0 == error and *module == NULL
     */
    #[allow(dead_code)]
    pub fn hw_get_module_by_class(class_id: *const raw::c_char,
                                  inst: *const raw::c_char,
                                  module: *mut *const hw_module_t)
                                  -> raw::c_int;
}
