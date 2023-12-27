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

use crate::types::{BOOLEAN, EFI_GUID, EFI_STATUS};

pub const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xDD9E7534,
        0x7762,
        0x4698,
        [0x8C, 0x14, 0xF5, 0x85, 0x17, 0xA6, 0x25, 0xAA],
    )
};

/// This protocol is used to obtain input from the `ConsoleIn` device. The EFI specification requires
/// that the `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` supports the same languages as the corresponding
/// `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL {
    /// Resets the input device hardware.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This`                 | A pointer to the `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL` instance.                                             |
    /// | **IN** `ExtendedVerification` | Indicates that the driver may perform a more exhaustive verification operation of the device during reset. |
    ///
    /// ## Description
    ///
    /// The `Reset()` function resets the input device hardware.
    ///
    /// The implementation of `Reset` is required to clear the contents of any input queues resident
    /// in memory used for buffering keystroke data and put the input stream in a known empty state.
    ///
    /// As part of initialization process, the firmware/device will make a quick but reasonable attempt
    /// to verify that the device is functioning. If the `ExtendedVerification` flag is `TRUE` the
    /// firmware may take an extended amount of time to verify the device is operating on reset.
    /// Otherwise the reset operation is to occur as quickly as possible.
    ///
    /// The hardware verification process is not defined by this specification and is left up to the
    /// platform firmware or driver to implement.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS`      | The device was reset.                                           |
    /// | `EFI_DEVICE_ERROR` | The device is not functioning correctly and could not be reset. |
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
}