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

use crate::types::{EFI_GUID, EFI_STATUS, UINTN, VOID};

pub const EFI_TAPE_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x1E93E633,
        0xD65A,
        0x459E,
        [0xAB, 0x84, 0x93, 0xD9, 0xEC, 0x26, 0x6D, 0x18],
    )
};

/// The EFI Tape IO protocol provides services to control and access a tape device.
#[repr(C)]
pub struct EFI_TAPE_IO_PROTOCOL {
    /// Reads from the tape.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_TAPE_IO_PROTOCOL` instance. |
    /// | **IN OUT** `BufferSize` | Size of the buffer in bytes pointed to by `Buffer`. |
    /// | **OUT** `Buffer` | Pointer to the buffer for data to be read into. |
    ///
    /// ## Description
    ///
    /// This function will read up to BufferSize bytes from media into the buffer pointed to by `Buffer` using an
    /// implementation-specific timeout. `BufferSize` will be updated with the number of bytes transferred.
    ///
    /// Each read operation for a device that operates in variable block size mode reads one media data block. Unread
    /// bytes which do not fit in the buffer will be skipped by the next read operation. The number of bytes transferred
    /// will be limited by the actual media block size. Best practice is for the buffer size to match the media data
    /// block size. When a file mark is encountered in variable block size mode the read operation will indicate that
    /// 0 bytes were transferred and the function will return an `EFI_END_OF_FILE` error condition.
    ///
    /// In fixed block mode the buffer is expected to be a multiple of the data block size. Each read operation for a
    /// device that operates in fixed block size mode may read multiple media data blocks. The number of bytes
    /// transferred will be limited to an integral number of complete media data blocks. BufferSize should be evenly
    /// divisible by the device’s fixed block size. When a file mark is encountered in fixed block size mode the read
    /// operation will indicate that the number of bytes transferred is less than the number of blocks that would fit
    /// in the provided buffer (possibly 0 bytes transferred) and the function will return an `EFI_END_OF_FILE` error
    /// condition.
    ///
    /// Two consecutive file marks are normally used to indicate the end of the last file on the media.
    ///
    /// The value specified for `BufferSize` should correspond to the actual block size used on the media. If necessary,
    /// the value for BufferSize may be larger than the actual media block size.
    ///
    /// Specifying a `BufferSize` of 0 is valid but requests the function to provide read-related status information
    /// instead of actual media data transfer. No data will be attempted to be read from the device however this
    /// operation is classified as an access for status handling. The status code returned may be used to determine if
    /// a file mark has been encountered by the last read request with a non-zero size, and to determine if media is
    /// loaded and the device is ready for reading. A `NULL` value for `Buffer` is valid when `BufferSize` is zero.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | Data was successfully transferred from the media. |
    /// | `EFI_END_OF_FILE` | A file mark was encountered which limited the data transferred by the read operation or the head is positioned just after a file mark. |
    /// | `EFI_NO_MEDIA` | No media is loaded in the device. |
    /// | `EFI_MEDIA_CHANGED` | The media in the device was changed since the last access. The transfer was aborted since the current position of the media may be incorrect. |
    /// | `EFI_DEVICE_ERROR` | A device error occurred while attempting to transfer data from the media. |
    /// | `EFI_INVALID_PARAMETER` | A `NULL` `Buffer` was specified with a non-zero `BufferSize` or the device is operating in fixed block size mode and the `BufferSize` was not a multiple of device’s fixed block size. |
    /// | `EFI_NOT_READY` | The transfer failed since the device was not ready (e.g. not online). The transfer may be retried at a later time. |
    /// | `EFI_UNSUPPORTED` | The device does not support this type of transfer. |
    /// | `EFI_TIMEOUT` | The transfer failed to complete within the timeout specified. |
    pub TapeRead: unsafe extern "efiapi" fn(
        This: *mut EFI_TAPE_IO_PROTOCOL,
        BufferSize: UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    /// Writes to the tape.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_TAPE_IO_PROTOCOL` instance. |
    /// | **IN** `BufferSize` | Size of the buffer in bytes pointed to by `Buffer`. |
    /// | **IN** `Buffer` | Pointer to the buffer for data to be written from. |
    pub TapeWrite: unsafe extern "efiapi" fn(
        This: *mut EFI_TAPE_IO_PROTOCOL,
        BufferSize: UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
}
