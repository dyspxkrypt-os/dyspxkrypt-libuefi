/*
 * Dyspxkrypt LibUEFI: Raw bindings of UEFI that conforms to the definitions of the UEFI Specification.
 * Copyright (C) 2023-2024 HTGAzureX1212.
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

use crate::protocols::media::block::EFI_BLOCK_IO_MEDIA;
use crate::types::{BOOLEAN, EFI_GUID, EFI_STATUS};

pub const EFI_BLOCK_IO2_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xA77B2472,
        0xE282,
        0x4E9F,
        [0xA2, 0x45, 0xC2, 0xC0, 0xE2, 0x7B, 0xBC, 0xC1],
    )
};

/// This protocol provides control over block devices.
///
/// The Block I/O 2 protocol defines an extension to the Block I/O protocol which enables the
/// ability to read and write data at a block level in a non-blocking manner.
#[repr(C)]
pub struct EFI_BLOCK_IO2_PROTOCOL {
    /// A pointer to the `EFI_BLOCK_IO_MEDIA` data for this device.
    pub Media: *mut EFI_BLOCK_IO_MEDIA,
    /// Resets the block device hardware.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | Indicates a pointer to the calling context. |
    /// | **IN** `ExtendedVerification` | Indicates that the driver may perform a more exhaustive verification operation of the device during reset. |
    ///
    /// ## Description
    ///
    /// The `Reset()` function resets the block device hardware.
    ///
    /// As part of the initialization process, the firmware/device will make a quick but reasonable
    /// attempt to verify that the device is functioning. If the `ExtendedVerification` flag is `TRUE`
    /// the firmware may take an extended amount of time to verify the device is operating on reset.
    /// Otherwise the reset operation is to occur as quickly as possible.
    ///
    /// The hardware verification process is not defined by this specification and is left up to the
    /// platform firmware or driver to implement.
    ///
    /// The `Reset()` function will terminate any in-flight non-blocking I/O requests by signaling
    /// an `EFI_ABORTED` in the `TransactionStatus` member of the `EFI_BLOCK_IO2_TOKEN` for the
    /// non-blocking I/O. After the `Reset()` function returns it is safe to free any `Token` or
    /// `Buffer` data structures that were allocated to initiate the non-blocking I/O requests that
    /// were in-flight for this device.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The block device was reset. |
    /// | `EFI_DEVICE_ERROR` | The block device is not functioning correctly and could not be reset. |
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO2_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
}
