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

use crate::types::{EFI_GUID, EFI_STATUS, UINT32, UINT64, UINTN, VOID};

pub const EFI_DISK_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xCE345171,
        0xBA0B,
        0x11D2,
        [0x8E, 0x4F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

pub const EFI_DISK_IO_PROTOCOL_REVISION: UINT64 = 0x00010000;

/// This protocol is used to abstract Block I/O interfaces.
///
/// The `EFI_DISK_IO_PROTOCOL` is used to control block I/O interfaces.
///
/// The disk I/O functions allow I/O operations that need not be on the underlying device’s block boundaries or alignment requirements. This is done by copying the data to/from internal buffers as needed to provide the proper requests to the block I/O device. Outstanding write buffer data is flushed by using the EFI_BLOCK_IO_PROTOCOL.FlushBlocks() function of the EFI_BLOCK_IO_PROTOCOL on the device handle.
///
/// The firmware automatically adds an EFI_DISK_IO_PROTOCOL interface to any EFI_BLOCK_IO_PROTOCOL interface that is produced. It also adds file system, or logical block I/O, interfaces to any EFI_DISK_IO_PROTOCOL interface that contains any recognized file system or logical block I/O devices. The firmware must automatically support the following required formats:
///
/// - The EFI FAT12, FAT16, and FAT32 file system types.
///
/// - The legacy master boot record partition block. (The presence of this on any block I/O device is optional, but if
/// it is present the firmware is responsible for allocating a logical device for each partition).
///
/// - The extended partition record partition block.
///
/// - The El Torito logical block devices.
#[repr(C)]
pub struct EFI_DISK_IO_PROTOCOL {
    /// The revision to which the disk I/O interface adheres. All future revisions must be backwards compatible. If a
    /// future version is not backwards compatible, it is not the same GUID.
    pub Revision: UINT64,
    /// Reads a specified number of bytes from a device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | Indicates a pointer to the calling context. |
    /// | **IN** `MediaId` | ID of the medium to be read. |
    /// | **IN** `Offset` | The starting byte offset on the logical block I/O device to read from. |
    /// | **IN** `BufferSize` | The size in bytes of `Buffer`. The number of bytes to read from the device. |
    /// | **OUT** `Buffer` | A pointer to the destination buffer for the data. The caller is responsible for either having implicit or explicit ownership of the buffer. |
    ///
    /// ## Description
    ///
    /// The `ReadDisk()` function reads the number of bytes specified by `BufferSize` from the device. All the bytes are
    /// read, or an error is returned. If there is no medium in the device, the function returns `EFI_NO_MEDIA`. If the
    /// `MediaId` is not the ID of the medium currently in the device, the function returns `EFI_MEDIA_CHANGED`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The data was read correctly from the device. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error while performing the read operation. |
    /// | `EFI_NO_MEDIA` | There is no medium in the device. |
    /// | `EFI_MEDIA_CHANGED` | The `MediaId` is not for the current medium. |
    /// | `EFI_INVALID_PARAMETER` | The read request contains device addresses that are not valid for the device. |
    pub ReadDisk: unsafe extern "efiapi" fn(
        This: *mut EFI_DISK_IO_PROTOCOL,
        MediaId: UINT32,
        Offset: UINT64,
        BufferSize: UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    /// Writes a specified number of bytes to a device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | Indicates a pointer to the calling context. |
    /// | **IN** `MediaId` | ID of the medium to be written. |
    /// | **IN** `Offset` | The starting byte offset on the logical block I/O device to write. |
    /// | **IN** `BufferSize` | The size in bytes of Buffer. The number of bytes to write to the device. |
    /// | **IN** `Buffer` | A pointer to the buffer containing the data to be written. |
    ///
    /// ## Description
    ///
    /// The `WriteDisk()` function writes the number of bytes specified by BufferSize to the device. All bytes are
    /// written, or an error is returned. If there is no medium in the device, the function returns `EFI_NO_MEDIA`. If
    /// the `MediaId` is not the ID of the medium currently in the device, the function returns `EFI_MEDIA_CHANGED`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The data was written correctly to the device. |
    /// | `EFI_WRITE_PROTECTED` | The device cannot be written to. |
    /// | `EFI_NO_MEDIA` | There is no medium in the device. |
    /// | `EFI_MEDIA_CHANGED` | The `MediaId` is not for the current medium. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error while performing the write operation. |
    /// | `EFI_INVALID_PARAMETER` | The write request contains device addresses that are not valid for the device. |
    pub WriteDisk: unsafe extern "efiapi" fn(
        This: *mut EFI_DISK_IO_PROTOCOL,
        MediaId: UINT32,
        Offset: UINT64,
        BufferSize: UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
}
