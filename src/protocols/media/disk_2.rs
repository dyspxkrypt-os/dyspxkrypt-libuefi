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

use crate::types::{EFI_EVENT, EFI_GUID, EFI_STATUS, UINT32, UINT64, UINTN, VOID};

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
    pub Cancel: unsafe extern "efiapi" fn(This: *mut EFI_DISK_IO2_PROTOCOL) -> EFI_STATUS,
    /// Reads a specified number of bytes from a device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | Indicates a pointer to the calling context. |
    /// | **IN** `MediaId` | ID of the medium to be read. |
    /// | **IN** `Offset` | The starting byte offset on the logical block I/O device to read from. |
    /// | **IN OUT** `Token` | A pointer to the token associated with the transaction. If this is `NULL`, synchronous/blocking IO is performed. |
    /// | **IN** `BufferSize` | The size in bytes of `Buffer`. The number of bytes to read from the device. |
    /// | **OUT** `Buffer` | A pointer to the destination buffer for the data. The caller is responsible for either having implicit or explicit ownership of the buffer. |
    ///
    /// ## Description
    ///
    /// The `ReadDiskEx()` function reads the number of bytes specified by `BufferSize` from the device.
    /// All the bytes are read, or an error is returned. If there is no medium in the device, the
    /// function returns `EFI_NO_MEDIA`. If the `MediaId` is not the ID of the medium currently in
    /// the device, the function returns `EFI_MEDIA_CHANGED`.
    ///
    /// If an error is returned from the call to `ReadDiskEx()` and non-blocking I/O is being requested,
    /// the `Event` associated with this request will not be signaled. If the call to `ReadDiskEx()`
    /// succeeds then the `Event` will be signaled upon completion of the read or if an error occurs
    /// during the processing of the request. The status of the read request can be determined from
    /// the `Status` field of the `Token` once the event is signaled.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | If `Event` is `NULL` (blocking I/O): The data was read correctly from the device. If `Event` is not `NULL` (asynchronous I/O): The request was successfully queued for processing. `Event` will be signaled upon completion. Returned in the token after signaling `Event`. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error while performing the read operation. |
    /// | `EFI_NO_MEDIA` | There is no medium in the device. |
    /// | `EFI_MEDIA_CHANGED` | The `MediaId` is not for the current medium. |
    /// | `EFI_INVALID_PARAMETER` | The read request contains device addresses that are not valid for the device. |
    /// | `EFI_OUT_OF_RESOURCES` | The request could not be completed due to a lack of resources. |
    pub ReadDiskEx: unsafe extern "efiapi" fn(
        This: *mut EFI_DISK_IO2_PROTOCOL,
        MediaId: UINT32,
        Offset: UINT64,
        Token: *mut EFI_DISK_IO2_TOKEN,
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
    /// | **IN** `Offset` | The starting byte offset on the logical block I/O device to write to. |
    /// | **IN** `Token` | A pointer to the token associated with the transaction. If this is `NULL`, synchronous/blocking IO is performed. |
    /// | **IN** `BufferSize` | The size in bytes of `Buffer`. The number of bytes to write to the device. |
    /// | **IN** `Buffer` | A pointer to the destination buffer for the data. The caller is responsible for either having implicit or explicit ownership of the buffer. |
    ///
    /// ## Description
    ///
    /// The `WriteDiskEx()` function writes the number of bytes specified by `BufferSize` to the device. All bytes are
    /// written, or an error is returned. If there is no medium in the device, the function returns `EFI_NO_MEDIA`. If
    /// the `MediaId` is not the ID of the medium currently in the device, the function returns `EFI_MEDIA_CHANGED`.
    ///
    /// If an error is returned from the call to `WriteDiskEx()` and non-blocking I/O is being requested, the `Event`
    /// associated with this request will not be signaled. If the call to `WriteDiskEx()` succeeds then the `Event` will
    /// be signaled upon completion of the write or if an error occurs during the processing of the request. The status
    /// of the write request can be determined from the `Status` field of the `Token` once the event is signaled.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | If `Event` is `NULL` (blocking I/O): The data was written correctly to the device. If `Event` is not `NULL` (asynchronous I/O): The request was successfully queued for processing. `Event` will be signaled upon completion. Returned in the token after signaling `Event`. |
    /// | `EFI_WRITE_PROTECTED` | The device cannot be written to. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error while performing the write operation. |
    /// | `EFI_NO_MEDIA` | There is no medium in the device. |
    /// | `EFI_MEDIA_CHANGED` | The `MediaId` is not for the current medium. |
    /// | `EFI_INVALID_PARAMETER` | The write request contains device addresses that are not valid for the device. |
    /// | `EFI_OUT_OF_RESOURCES` | The request could not be completed due to a lack of resources. |
    pub WriteDiskEx: unsafe extern "efiapi" fn(
        This: *mut EFI_DISK_IO2_PROTOCOL,
        MediaId: UINT32,
        Offset: UINT64,
        Token: *mut EFI_DISK_IO2_TOKEN,
        BufferSize: UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    /// Flushes all modified data to the physical device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | Indicates a pointer to the calling context. |
    /// | **IN** `Token` | A pointer to the token associated with the transaction. If this is `NULL`, synchronous/blocking IO is performed. |
    ///
    /// ## Description
    ///
    /// The `FlushDiskEx()` function flushes all modified data to the physical device. If an error is returned from the
    /// call to `FlushDiskEx()` and non-blocking I/O is being requested, the `Event` associated with this request will
    /// not be signaled. If the call to `FlushDiskEx()` succeeds then the Event will be signaled upon completion of the
    /// flush or if an error occurs during the processing of the request. The status of the flush request can be
    /// determined from the `Status` field of the `Token` once the event is signaled.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | If `Event` is `NULL` (blocking I/O): The data was flushed successfully to the device. If `Event` is not `NULL` (asynchronous I/O): The request was successfully queued for processing. `Event` will be signaled upon completion. Returned in the token after signaling `Event`. |
    /// | `EFI_WRITE_PROTECTED` | The device cannot be written to. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error while performing the flush operation. |
    /// | `EFI_NO_MEDIA` | There is no medium in the device. |
    /// | `EFI_MEDIA_CHANGED` | The medium in the device has changed since the last access. |
    /// | `EFI_INVALID_PARAMETER` | `Token` is `NULL` |
    /// | `EFI_OUT_OF_RESOURCES` | The request could not be completed due to a lack of resources. |
    pub FlushDiskEx: unsafe extern "efiapi" fn(
        This: *mut EFI_DISK_IO2_PROTOCOL,
        Token: *mut EFI_DISK_IO2_TOKEN,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_DISK_IO2_TOKEN {
    /// If `Event` is `NULL`, then blocking I/O is performed. If `Event` is not `NULL` and non-blocking
    /// I/O is supported, then non-blocking I/O is performed, and Event will be signaled when the I/O
    /// request is completed. The caller must be prepared to handle the case where the callback
    /// associated with `Event` occurs before the original asynchronous I/O request call returns.
    pub Event: EFI_EVENT,
    /// Defines whether or not the signaled event encountered an error.
    pub TransactionStatus: EFI_STATUS,
}
