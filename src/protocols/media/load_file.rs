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
use crate::types::{BOOLEAN, EFI_GUID, EFI_STATUS, UINTN, VOID};

pub const EFI_LOAD_FILE_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x56EC3091,
        0x954C,
        0x11D2,
        [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

/// Used to obtain files, that are primarily boot options, from arbitrary devices.
///
/// The `EFI_LOAD_FILE_PROTOCOL` is a simple protocol used to obtain files from arbitrary devices.
///
/// When the firmware is attempting to load a file, it first attempts to use the device’s Simple
/// File System protocol to read the file. If the file system protocol is found, the firmware
/// implements the policy of interpreting the File Path value of the file being loaded. If the
/// device does not support the file system protocol, the firmware then attempts to read the file
/// via the `EFI_LOAD_FILE_PROTOCOL` and the `LoadFile()` function. In this case the `LoadFile()`
/// function implements the policy of interpreting the File Path value.
#[repr(C)]
pub struct EFI_LOAD_FILE_PROTOCOL {
    /// Causes the driver to load a specified file.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | Indicates a pointer to the calling context. |
    /// | **IN** `FilePath` | The device specific path of the file to load. |
    /// | **IN** `BootPolicy` | If `TRUE`, indicates that the request originates from the boot manager, and that the boot manager is attempting to load `FilePath` as a boot selection. If `FALSE`, then `FilePath` must match an exact file to be loaded. |
    /// | **IN OUT** `BufferSize` | On input the size of `Buffer` in bytes. On output with a return code of `EFI_SUCCESS`, the amount of data transferred to `Buffer`. On output with a return code of `EFI_BUFFER_TOO_SMALL`, the size of `Buffer` required to retrieve the requested file. |
    /// | **OUT** `Buffer` **OPTIONAL** | The memory buffer to transfer the file to. If `Buffer` is `NULL`, then the size of the requested file is returned in `BufferSize`. |
    ///
    /// ## Description
    ///
    /// The `LoadFile()` function interprets the device-specific FilePath parameter, returns the
    /// entire file into `Buffer`, and sets BufferSize to the amount of data returned. If `Buffer` is
    /// `NULL`, then the size of the file is returned in `BufferSize`. If `Buffer` is not `NULL`, and
    /// `BufferSize` is not large enough to hold the entire file, then `EFI_BUFFER_TOO_SMALL` is
    /// returned, and `BufferSize` is updated to indicate the size of the buffer needed to obtain the
    /// file. In this case, no data is returned in `Buffer`.
    ///
    /// If `BootPolicy` is `FALSE` the `FilePath` must match an exact file to be loaded. If no such
    /// file exists, `EFI_NOT_FOUND` is returned. If `BootPolicy` is `FALSE`, and an attempt is being
    /// made to perform a network boot through the PXE Base Code protocol, `EFI_UNSUPPORTED` is returned.
    ///
    /// If `BootPolicy` is `TRUE` the firmware’s boot manager is attempting to load an EFI image that
    /// is a boot selection. In this case, `FilePath` contains the file path value in the boot selection
    /// option. Normally the firmware would implement the policy on how to handle an inexact boot file
    /// path; however, since in this case the firmware cannot interpret the file path, the `LoadFile()`
    /// function is responsible for implementing the policy. For example, in the case of a network boot
    /// through the PXE Base Code protocol, `FilePath` merely points to the root of the device, and
    /// the firmware interprets this as wanting to boot from the first valid loader. The following
    /// is a list of events that `LoadFile()` will implement for a PXE boot:
    ///
    /// - Perform DHCP.
    ///
    /// - Optionally prompt the user with a menu of boot selections.
    ///
    /// - Discover the boot server and the boot file.
    ///
    /// - Download the boot file into Buffer and update BufferSize with the size of the boot file.
    ///
    /// If the boot file downloaded from boot server is not an UEFI-formatted executable, but a binary
    /// image which contains a UEFI-compliant file system, then `EFI_WARN_FILE_SYSTEM` is returned, and
    /// a new RAM disk mapped on the returned `Buffer` is registered.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The file was loaded. |
    /// | `EFI_UNSUPPORTED` | The device does not support the provided `BootPolicy`. |
    /// | `EFI_INVALID_PARAMETER` | `FilePath` is not a valid device path, or `BufferSize` is `NULL`. |
    /// | `EFI_NO_MEDIA` | No medium was present to load the file. |
    /// | `EFI_DEVICE_ERROR` | The file was not loaded due to a device error. |
    /// | `EFI_NO_RESPONSE` | The remote system did not respond. |
    /// | `EFI_NOT_FOUND` | The file was not found. |
    /// | `EFI_ABORTED` | The file load process was manually cancelled. |
    /// | `EFI_BUFFER_TOO_SMALL` | The `BufferSize` is too small to read the current directory entry. `BufferSize` has been updated with the size needed to complete the request. |
    /// | `EFI_WARN_FILE_SYSTEM` | The resulting `Buffer` contains UEFI-compliant file system. |
    pub LoadFile: unsafe extern "efiapi" fn(
        This: *mut EFI_LOAD_FILE_PROTOCOL,
        FilePath: *mut EFI_DEVICE_PATH_PROTOCOL,
        BootPolicy: BOOLEAN,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
}
