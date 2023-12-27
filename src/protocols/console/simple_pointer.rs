/*
 * Dyspxkrypt LibUEFI: Raw bindings of UEFI that conforms to the definitions of the UEFI Specification.
 * Copyright (C) 2023 HTGAzureX1212.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::types::{BOOLEAN, EFI_GUID, EFI_STATUS, INT32};

pub const EFI_SIMPLE_POINTER_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x31878C87,
        0x0B75,
        0x11D5,
        [0x9A, 0x4F, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
    )
};

/// This protocol provides services that allow information about a pointer device to be retrieved.
#[repr(C)]
pub struct EFI_SIMPLE_POINTER_PROTOCOL {
    /// Resets the pointer device hardware.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_SIMPLE_POINTER_PROTOCOL` instance. |
    /// | **IN** `ExtendedVerification` | Indicates that the driver may perform a more exhaustive verification operation of the device during reset. |
    ///
    /// ## Description
    ///
    /// This `Reset()` function resets the pointer device hardware. As part of initialization process,
    /// the firmware/device will make a quick but reasonable attempt to verify that the device is functioning.
    /// If the `ExtendedVerification` flag is `TRUE` the firmware may take an extended amount of time
    /// to verify the device is operating on reset. Otherwise the reset operation is to occur as
    /// quickly as possible. The hardware verification process is not defined by this specification
    /// and is left up to the platform firmware or driver to implement.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The device was reset. |
    /// | `EFI_DEVICE_ERROR` | The device is not functioning correctly and could not be reset. |
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_POINTER_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
    /// Retrieves the current state of a pointer device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_SIMPLE_POINTER_PROTOCOL` instance. |
    /// | **IN** `State` | A pointer to the state information on the pointer device.  |
    ///
    /// ## Description
    ///
    /// The `GetState()` function retrieves the current state of a pointer device. This includes
    /// information on the buttons associated with the pointer device and the distance that each of
    /// the axes associated with the pointer device has been moved. If the state of the pointer device
    /// has not changed since the last call to `GetState()`, then `EFI_NOT_READY` is returned. If
    /// the state of the pointer device has changed since the last call to `GetState()`, then the
    /// state information is placed in State, and `EFI_SUCCESS` is returned. If a device error occurs
    /// while attempting to retrieve the state information, then `EFI_DEVICE_ERROR` is returned.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The state of the pointer device was returned in `State`. |
    /// | `EFI_NOT_READY` | The state of the pointer device has not changed since the last call to `GetState()`. |
    /// | `EFI_DEVICE_ERROR` | A device error occurred while attempting to retrieve the pointer device’s current state. |
    pub GetState: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_POINTER_PROTOCOL,
        State: *mut EFI_SIMPLE_POINTER_STATE,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_SIMPLE_POINTER_STATE {
    pub RelativeMovementX: INT32,
    pub RelativeMovementY: INT32,
    pub RelativeMovementZ: INT32,
    pub LeftButton: BOOLEAN,
    pub RightButton: BOOLEAN,
}
