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

use crate::types::{EFI_GUID, EFI_STATUS, UINT64};

pub const EFI_LOAD_FILE2_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x4006C0C1,
        0xFCB3,
        0x403E,
        [0x99, 0x6D, 0x4A, 0x6C, 0x87, 0x24, 0xE0, 0x6D],
    )
};

pub const EFI_DISK_IO2_PROTOCOL_REVISION: UINT64 = 0x00020000;

/// This protocol is used to abstract Block I/O interfaces in a non-blocking manner.
///
/// The `EFI_DISK_IO2_PROTOCOL` is used to control block I/O interfaces.
///
/// The disk I/O functions allow I/O operations that need not be on the underlying device’s block
/// boundaries or alignment requirements. This is done by copying the data to/from internal buffers
/// as needed to provide the proper requests to the block I/O device. Outstanding write buffer data
/// is flushed by using the `FlushBlocksEx()` function of the `EFI_BLOCK_IO2_PROTOCOL` on the device
/// handle.
///
/// The firmware automatically adds an `EFI_DISK_IO2_PROTOCOL` interface to any `EFI_BLOCK_IO2_PROTOCOL`
/// interface that is produced. It also adds file system, or logical block I/O, interfaces to any
/// `EFI_DISK_IO2_PROTOCOL` interface that contains any recognized file system or logical block I/O
/// devices.
///
/// Implementations must account for cases where there is pending queued asynchronous I/O when a call
/// is received on a blocking protocol interface. In these cases the pending I/O will be processed
/// and completed before the blocking function is executed so that operation are carried out in the
/// order they were requested.
#[repr(C)]
pub struct EFI_DISK_IO2_PROTOCOL {
    /// The revision to which the disk I/O interface adheres.
    pub Revision: UINT64,
    /// Terminate outstanding asynchronous requests to a device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | Indicates a pointer to the calling context. |
    ///
    /// ## Description
    ///
    /// The `Cancel()` function will terminate any in-flight non-blocking I/O requests by signaling
    /// the `EFI_DISK_IO2_TOKEN` event and with `TransactionStatus` set to `EFI_ABORTED`. After the
    /// `Cancel()` function returns it is safe to free any Token or Buffer data structures that were
    /// allocated as part of the non-blocking I/O operation.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | All outstanding requests were successfully terminated. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error while performing the cancel operation. |
    pub Cancel: unsafe extern "efiapi" fn(
        This: *mut EFI_DISK_IO2_PROTOCOL,
    ) -> EFI_STATUS,
}
