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

use crate::types::{BOOLEAN, EFI_EVENT, EFI_GUID, EFI_STATUS, UINT32, UINT64};

pub const EFI_ABSOLUTE_POINTER_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x8D59D32B,
        0xC655,
        0x4AE9,
        [0x9B, 0x15, 0xF2, 0x59, 0x04, 0x99, 0x2A, 0x43],
    )
};

pub const EFI_ABSP_SupportsAltActive: UINT32 = 0x00000001;
pub const EFI_ABSP_SupportsPressureAsZ: UINT32 = 0x00000002;

#[repr(C)]
pub struct EFI_ABSOLUTE_POINTER_PROTOCOL {
    /// Resets the pointer device hardware.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_ABSOLUTE_POINTER_PROTOCOL` instance. |
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
        This: *mut EFI_ABSOLUTE_POINTER_PROTOCOL,
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
        This: *mut EFI_ABSOLUTE_POINTER_PROTOCOL,
        State: *mut EFI_ABSOLUTE_POINTER_STATE,
    ) -> EFI_STATUS,
    /// Event to use with `WaitForEvent()` to wait for input from the pointer device.
    pub WaitForInput: EFI_EVENT,
    /// Pointer to `EFI_ABSOLUTE_POINTER_MODE` data.
    pub Mode: *mut EFI_ABSOLUTE_POINTER_MODE,
}

#[repr(C)]
pub struct EFI_ABSOLUTE_POINTER_STATE {
    CurrentX: UINT64,
    CurrentY: UINT64,
    CurrentZ: UINT64,
    ActiveButtons: UINT32,
}

#[repr(C)]
pub struct EFI_ABSOLUTE_POINTER_MODE {
    AbsoluteMinX: UINT64,
    AbsoluteMinY: UINT64,
    AbsoluteMinZ: UINT64,
    AbsoluteMaxX: UINT64,
    AbsoluteMaxY: UINT64,
    AbsoluteMaxZ: UINT64,
    Attributes: UINT32,
}
