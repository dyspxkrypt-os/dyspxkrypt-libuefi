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

use crate::protocols::device_path::EFI_DEVICE_PATH_PROTOCOL;
use crate::types::{EFI_GUID, UINTN};

pub const EFI_DEVICE_PATH_UTILITIES_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x0379BE4E,
        0xD706,
        0x437D,
        [0xB0, 0x37, 0xED, 0xB8, 0x2F, 0xB7, 0x72, 0xA4],
    )
};

/// Creates and manipulates device paths and device nodes.
///
/// The `EFI_DEVICE_PATH_UTILITIES_PROTOCOL` provides common utilities for creating a manipulating
/// device paths and device nodes.
#[repr(C)]
pub struct EFI_DEVICE_PATH_UTILITIES_PROTOCOL {
    /// Returns the size of the device path, in bytes.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `DevicePath` | Points to the start of the EFI device path. |
    ///
    /// ## Description
    ///
    /// This function returns the size of the specified device path, in bytes, including the end-of-path
    /// tag. If `DevicePath` is `NULL` then zero is returned.
    pub GetDevicePathSize: unsafe extern "efiapi" fn(
        DevicePath: *const EFI_DEVICE_PATH_PROTOCOL,
    ) -> UINTN,
    /// Create a duplicate of the specified path.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `DevicePath` | Points to the start of the EFI device path. |
    ///
    /// ## Description
    ///
    /// This function creates a duplicate of the specified device path. The memory is allocated from
    /// EFI boot services memory. It is the responsibility of the caller to free the memory allocated.
    /// If `DevicePath` is `NULL` then `NULL` will be returned and no memory will be allocated.
    ///
    /// ## Returns
    ///
    /// This function returns a pointer to the duplicate device path `or` NULL if there was
    /// insufficient memory.
    pub DuplicateDevicePath: unsafe extern "efiapi" fn(
        DevicePath: *const EFI_DEVICE_PATH_UTILITIES_PROTOCOL,
    ) -> *mut EFI_DEVICE_PATH_UTILITIES_PROTOCOL,
}
