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

use crate::types::{BOOLEAN, CHAR16, EFI_EVENT, EFI_GUID, EFI_STATUS, UINT16};

pub const EFI_SIMPLE_TEXT_INPUT_PROTOCOL_GUID: EFI_GUID = EFI_GUID {
    Data1: 0x387477C1,
    Data2: 0x69C7,
    Data3: 0x11D2,
    Data4: [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
};

/// The Simple Text Input Protocol defines the minimum input required to support the console-in device.
///
/// This protocol is used to obtain input from the console-in device. The EFI specification requires
/// that the `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` supports the same languages as the corresponding
/// `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    /// Resets the input device hardware.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This`                 | A pointer to the `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` instance.                                                |
    /// | **IN** `ExtendedVerification` | Indicates that the driver may perform a more exhaustive verification operation of the device during reset. |
    ///
    /// ## Description
    ///
    /// The `Reset()` function resets the input device hardware.
    ///
    /// The implementation of `Reset` is required to clear the contents of any input queues resident
    /// in memory used for buffering keystroke data and put the input stream in a known empty state.
    ///
    /// As part of initialization process, the firmware or device will make a quick but reasonable attempt
    /// to verify that the device is functioning. If the `ExtendedVerification` flag is `TRUE` the firmware
    /// may take an extended amount of time to verify the device is operating on reset. Otherwise the
    /// reset operation is to occur as quickly as possible.
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
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
    /// Reads the next keystroke from the input device.
    ///
    /// ## Parameters
    ///
    /// | Parameter     | Description                                                                                          |
    /// | ------------- | ---------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` instance.                                          |
    /// | **OUT** `Key` | A pointer to a buffer that is filled in with the keystroke information for the key that was pressed. |
    ///
    /// ## Description
    ///
    /// The `ReadKeyStroke()` function reads the next keystroke from the input device. If there is
    /// no pending keystroke the function returns `EFI_NOT_READY`. If there is a pending keystroke,
    /// then `ScanCode` is the EFI scan code. The `UnicodeChar` is the actual printable character or
    /// zero if the key does not represent a printable character (like, control key, function key, etc.).
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                        |
    /// | ------------------ | ------------------------------------------------------------------ |
    /// | `EFI_SUCCESS`      | The keystroke information was returned.                            |
    /// | `EFI_NOT_READY`    | There was no keystroke data available.                             |
    /// | `EFI_DEVICE_ERROR` | The keystroke information was not returned due to hardware errors. |
    /// | `EFI_UNSUPPORTED`  | The device does not support the ability to read keystroke data.    |
    pub ReadKeyStroke: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        Key: *mut EFI_INPUT_KEY,
    ),
    /// An event to use with the `EFI_BOOT_SERVICES.WaitForEvent()` function to wait for a key to be
    /// available.
    pub WaitForKey: EFI_EVENT,
}

/// An input key returned from the `EFI_SIMPLE_TEXT_INPUT_PROTOCOL.ReadKeyStroke` function.
#[repr(C)]
pub struct EFI_INPUT_KEY {
    /// The EFI scan code.
    pub ScanCode: UINT16,
    /// The actual printable character or zero if the key does not represent a printable character.
    pub UnicodeChar: CHAR16,
}
