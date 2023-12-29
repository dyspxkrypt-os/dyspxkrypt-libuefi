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

use crate::tables::runtime::EFI_TIME;
use crate::types::{BOOLEAN, CHAR16, EFI_EVENT, EFI_GUID, EFI_STATUS, UINT32, UINT64, UINTN, VOID};

pub const EFI_FILE_PROTOCOL_REVISION: UINT64 = 0x00010000;
pub const EFI_FILE_PROTOCOL_REVISION2: UINT64 = 0x00020000;
#[cfg(not(feature = "media-file-v2"))]
#[cfg_attr(doc, doc(not(feature = "media-file-v2")))]
#[cfg_attr(docsrs, doc(not(feature = "media-file-v2")))]
pub const EFI_FILE_PROTOCOL_LATEST_REVISION: UINT64 = EFI_FILE_PROTOCOL_REVISION;
#[cfg(feature = "media-file-v2")]
#[cfg_attr(doc, doc(cfg(feature = "media-file-v2")))]
#[cfg_attr(docsrs, doc(cfg(feature = "media-file-v2")))]
pub const EFI_FILE_PROTOCOL_LATEST_REVISION: UINT64 = EFI_FILE_PROTOCOL_REVISION2;

pub const EFI_FILE_MODE_READ: UINT64 = 0x0000000000000001;
pub const EFI_FILE_MODE_WRITE: UINT64 = 0x0000000000000002;
pub const EFI_FILE_MODE_CREATE: UINT64 = 0x8000000000000000;

pub const EFI_FILE_READ_ONLY: UINT64 = 0x0000000000000001;
pub const EFI_FILE_HIDDEN: UINT64 = 0x0000000000000002;
pub const EFI_FILE_SYSTEM: UINT64 = 0x0000000000000004;
pub const EFI_FILE_RESERVED: UINT64 = 0x0000000000000008;
pub const EFI_FILE_DIRECTORY: UINT64 = 0x0000000000000010;
pub const EFI_FILE_ARCHIVE: UINT64 = 0x0000000000000020;
pub const EFI_FILE_VALID_ATTR: UINT64 = 0x0000000000000037;

pub const EFI_FILE_INFO_ID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x09576E92,
        0x6D3f,
        0x11D2,
        [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

pub const EFI_FILE_SYSTEM_INFO_ID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x09576E93,
        0x6d3f,
        0x11d2,
        [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

pub const EFI_FILE_SYSTEM_VOLUME_LABEL_ID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xDB47D7D3,
        0xFE81,
        0x11D3,
        [0x9A, 0x35, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
    )
};

/// This protocol provides file based access to supported file systems.
///
/// The `EFI_FILE_PROTOCOL` provides file IO access to supported file systems.
///
/// An `EFI_FILE_PROTOCOL` provides access to a file’s or directory’s contents, and is also a reference to a location
/// in the directory tree of the file system in which the file resides. With any given file handle, other files may be
/// opened relative to this file’s location, yielding new file handles.
///
/// On requesting the file system protocol on a device, the caller gets the `EFI_FILE_PROTOCOL` to the volume. This
/// interface is used to open the root directory of the file system when needed. The caller must `EFI_FILE_PROTOCOL.Close()`
/// the file handle to the root directory, and any other opened file handles before exiting. While there are open files
/// on the device, usage of underlying device protocol(s) that the file system is abstracting must be avoided. For example,
/// when a file system that is layered on a `EFI_DISK_IO_PROTOCOL` or`EFI_BLOCK_IO_PROTOCOL` , direct block access to
/// the device for the blocks that comprise the file system must be avoided while there are open file handles to the same
/// device.
///
/// A file system driver may cache data relating to an open file. A `Flush()` function is provided that flushes all dirty
/// data in the file system, relative to the requested file, to the physical medium. If the underlying device may cache data,
/// the file system must inform the device to flush as well.
///
/// Implementations must account for cases where there is pending queued asynchronous I/O when a call is received on a blocking
/// protocol interface. In these cases the pending I/O will be processed and completed before the blocking function is executed
/// so that operation are carried out in the order they were requested.
#[repr(C)]
pub struct EFI_FILE_PROTOCOL {
    pub Revision: UINT64,
    /// Opens a new file relative to the source file’s location.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle to the source location. This would typically be an open handle to a directory. |
    /// | **OUT** `NewHandle` | A pointer to the location to return the opened handle for the new file. |
    /// | **IN** `FileName` | The null-terminated string of the name of the file to be opened. The file name may contain the following path modifiers: "", ".", and "..". |
    /// | **IN** `OpenMode` | The mode to open the file. The only valid combinations that the file may be opened with are: Read, Read/Write, or Create/Read/Write. |
    /// | **IN** `Attributes` | Only valid for EFI_FILE_MODE_CREATE, in which case these are the attribute bits for the newly created file. |
    ///
    /// ## Description
    ///
    /// The `Open()` function opens the file or directory referred to by `FileName` relative to the location of `This`
    /// and returns a `NewHandle`. The `FileName` may include the following path modifiers:
    ///
    /// - "\"
    ///
    /// If the filename starts with a "\" the relative location is the root directory that `This` resides on; otherwise
    /// "" separates name components. Each name component is opened in turn, and the handle to the last file opened is
    /// returned.
    ///
    /// - "."
    ///
    /// Opens the current location.
    ///
    /// - ".."
    ///
    /// Opens the parent directory for the current location. If the location is the root directory the request will
    /// return an error, as there is no parent directory for the root directory.
    ///
    /// If `EFI_FILE_MODE_CREATE` is set, then the file is created in the directory. If the final location of `FileName`
    /// does not refer to a directory, then the operation fails. If the file does not exist in the directory, then a
    /// new file is created. If the file already exists in the directory, then the existing file is opened.
    ///
    /// If the medium of the device changes, all accesses (including the File handle) will result in `EFI_MEDIA_CHANGED`.
    /// To access the new medium, the volume must be reopened.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The file was opened. |
    /// | `EFI_NOT_FOUND` | The specified file could not be found on the device. |
    /// | `EFI_NO_MEDIA` | The device has no medium. |
    /// | `EFI_MEDIA_CHANGED` | The device has a different medium in it or the medium is no longer supported. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error. |
    /// | `EFI_VOLUME_CORRUPTED` | The file system structures are corrupted. |
    /// | `EFI_WRITE_PROTECTED` | An attempt was made to create a file, or open a file for write when the media is write-protected. |
    /// | `EFI_ACCESS_DENIED` | The service denied access to the file. |
    /// | `EFI_OUT_OF_RESOURCES` | Not enough resources were available to open the file. |
    /// | `EFI_VOLUME_FULL` | The volume is full. |
    pub Open: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        NewHandle: *mut *mut EFI_FILE_PROTOCOL,
        FileName: *mut CHAR16,
        OpenMode: UINT64,
        Attributes: UINT64,
    ) -> EFI_STATUS,
    /// Closes a specified file handle.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle to close. |
    ///
    /// ## Description
    ///
    /// The `Close()` function closes a specified file handle. All “dirty” cached file data is flushed to the device,
    /// and the file is closed. In all cases the handle is closed. The operation will wait for all pending asynchronous
    /// I/O requests to complete before completing.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The file was closed. |
    pub Close: unsafe extern "efiapi" fn(This: *mut EFI_FILE_PROTOCOL) -> EFI_STATUS,
    /// Closes and deletes a file.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the handle to the file to delete. |
    ///
    /// ## Description
    ///
    /// The `Delete()` function closes and deletes a file. In all cases the file handle is closed. If the file cannot be
    /// deleted, the warning code `EFI_WARN_DELETE_FAILURE` is returned, but the handle is still closed.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The file was closed and deleted, and the handle was closed. |
    /// | `EFI_WARN_DELETE_FAILURE` | The handle was closed, but the file was not deleted. |
    pub Delete: unsafe extern "efiapi" fn(This: *mut EFI_FILE_PROTOCOL) -> EFI_STATUS,
    /// Reads data from a file.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle to read data from. |
    /// | **IN OUT** `BufferSize` | On input, the size of the `Buffer`. On output, the amount of data returned in `Buffer`. In both cases, the size is measured in bytes. |
    /// | **OUT** `Buffer` | The buffer into which the data is read. |
    ///
    /// ## Description
    ///
    /// The `Read()` function reads data from a file.
    ///
    /// If `This` is not a directory, the function reads the requested number of bytes from the file at the file’s current
    /// position and returns them in `Buffer`. If the read goes beyond the end of the file, the read length is truncated
    /// to the end of the file. The file’s current position is increased by the number of bytes returned.
    ///
    /// If `This` is a directory, the function reads the directory entry at the file’s current position and returns the
    /// entry in `Buffer`. If the `Buffer` is not large enough to hold the current directory entry, then `EFI_BUFFER_TOO_SMALL`
    /// is returned and the current file position is not updated. `BufferSize` is set to be the size of the buffer needed
    /// to read the entry. On success, the current position is updated to the next directory entry. If there are no more
    /// directory entries, the read returns a zero-length buffer. `EFI_FILE_INFO` is the structure returned as the
    /// directory entry.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The data was read. |
    /// | `EFI_NO_MEDIA` | The device has no medium. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error. |
    /// | `EFI_DEVICE_ERROR` | An attempt was made to read from a deleted file. |
    /// | `EFI_DEVICE_ERROR` | On entry, the current file position is beyond the end of the file. |
    /// | `EFI_VOLUME_CORRUPTED` | The file system structures are corrupted. |
    /// | `EFI_BUFFER_TOO_SMALL` | The `BufferSize` is too small to read the current directory entry. `BufferSize` has been updated with the size needed to complete the request. |
    pub Read: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    /// Writes data to a file.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle to write data to. |
    /// | **IN OUT** `BufferSize` | On input, the size of the `Buffer`. On output, the amount of data actually written. In both cases, the size is measured in bytes. |
    /// | **OUT** `Buffer` | The buffer of data to write. |
    ///
    /// ## Description
    ///
    /// The `Write()` function writes the specified number of bytes to the file at the current file position. The current
    /// file position is advanced the actual number of bytes written, which is returned in `BufferSize`. Partial writes
    /// only occur when there has been a data error during the write attempt (such as "file space full"). The file is
    /// automatically grown to hold the data if required.
    ///
    /// Direct writes to opened directories are not supported.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The data was written. |
    /// | `EFI_UNSUPPORTED` | Writes to open directory files are not supported. |
    /// | `EFI_NO_MEDIA` | The device has no medium. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error. |
    /// | `EFI_DEVICE_ERROR` | An attempt was made to write to a deleted file. |
    /// | `EFI_VOLUME_CORRUPTED` | The file system structures are corrupted. |
    /// | `EFI_WRITE_PROTECTED` | The `BufferSize` is too small to read the current directory entry. `BufferSize` has been updated with the size needed to complete the request. |
    /// | `EFI_ACCESS_DENIED` | The file system structures are corrupted. |
    /// | `EFI_VOLUME_FULL` | The file system structures are corrupted. |
    pub Write: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    /// Returns a file’s current position.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle to get the current position on. |
    /// | **OUT** `Position` | The address to return the file’s current position value. |
    ///
    /// ## Description
    ///
    /// The `GetPosition()` function returns the current file position for the file handle. For directories, the current
    /// file position has no meaning outside of the file system driver and as such the operation is not supported. An
    /// error is returned if `This` is a directory.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The position was returned. |
    /// | `EFI_UNSUPPORTED` | The request is not valid on open directories. |
    /// | `EFI_DEVICE_ERROR` | An attempt was made to get the position from a deleted file. |
    pub GetPosition: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        Position: *mut UINT64,
    ) -> EFI_STATUS,
    /// Sets a file’s current position.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle to set the current position on. |
    /// | **IN** `Position` | The byte position from the start of the file to set. |
    ///
    /// ## Description
    ///
    /// The `SetPosition()` function sets the current file position for the handle to the position supplied. With the
    /// exception of seeking to position `0xFFFFFFFFFFFFFFFF`, only absolute positioning is supported, and seeking past
    /// the end of the file is allowed (a subsequent write would grow the file). Seeking to position `0xFFFFFFFFFFFFFFFF`
    /// causes the current position to be set to the end of the file.
    ///
    /// If `This` is a directory, the only position that may be set is zero. This has the effect of starting the read
    /// process of the directory entries over.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The position was set. |
    /// | `EFI_UNSUPPORTED` | The seek request for nonzero is not valid on open directories. |
    /// | `EFI_DEVICE_ERROR` | An attempt was made to set the position of a deleted file. |
    pub SetPosition:
        unsafe extern "efiapi" fn(This: *mut EFI_FILE_PROTOCOL, Position: UINT64) -> EFI_STATUS,
    /// Returns information about a file.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle the requested information is for. |
    /// | **IN** `InformationType` | The type identifier for the information being requested. |
    /// | **IN OUT** `BufferSize` | On input, the size of `Buffer`. On output, the amount of data returned in `Buffer`. In both cases, the size is measured in bytes. |
    /// | **OUT** `Buffer` | A pointer to the data buffer to return. The buffer’s type is indicated by `InformationType`. |
    ///
    /// ## Description
    ///
    /// The `GetInfo()` function returns information of type `InformationType` for the requested file. If the file does
    /// not support the requested information type, then `EFI_UNSUPPORTED` is returned. If the buffer is not large enough
    /// to fit the requested structure, `EFI_BUFFER_TOO_SMALL` is returned and the `BufferSize` is set to the size of
    /// buffer that is required to make the request.
    ///
    /// The information types defined by this specification are required information types that all file systems must
    /// support.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The information was retrieved. |
    /// | `EFI_UNSUPPORTED` | The `InformationType` is not known. |
    /// | `EFI_NO_MEDIA` | The device has no medium. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error. |
    /// | `EFI_VOLUME_CORRUPTED` | The file system structures are corrupted. |
    /// | `EFI_BUFFER_TOO_SMALL` | The `BufferSize` is too small to read the current directory entry. `BufferSize` has been updated with the size needed to complete the request. |
    pub GetInfo: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        InformationType: *mut EFI_GUID,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    /// Sets information about a file.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle the information is for. |
    /// | **IN** `InformationType` | The type identifier for the information being set. |
    /// | **IN** `BufferSize` | The size, in bytes, of `Buffer`. |
    /// | **IN** `Buffer` | A pointer to the data buffer to write. The buffer’s type is indicated by `InformationType`. |
    ///
    /// ## Description
    ///
    /// The `SetInfo()` function sets information of type `InformationType` on the requested file. Because a read-only file
    /// can be opened only in read-only mode, an `InformationType` of `EFI_FILE_INFO_ID` can be used with a read-only
    /// file because this method is the only one that can be used to convert a read-only file to a read-write file. In
    /// this circumstance, only the Attribute field of the `EFI_FILE_INFO` structure may be modified. One or more calls
    /// to `SetInfo()` to change the `Attribute` field are permitted before it is closed. The file attributes will be
    /// valid the next time the file is opened with `Open()`.
    ///
    /// An `InformationType` of `EFI_FILE_INFO_ID`, `EFI_FILE_SYSTEM_INFO_ID` or `EFI_FILE_SYSTEM_VOLUME_LABEL_ID` may
    /// not be used on read-only media.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The information was set. |
    /// | `EFI_UNSUPPORTED` | The `InformationType` is not known. |
    /// | `EFI_NO_MEDIA` | The device has no medium. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error. |
    /// | `EFI_VOLUME_CORRUPTED` | The file system structures are corrupted. |
    /// | `EFI_WRITE_PROTECTED` | `InformationType` is `EFI_FILE_INFO_ID` and the media is read-only. |
    /// | `EFI_WRITE_PROTECTED` | `InformationType` is `EFI_FILE_PROTOCOL_SYSTEM_INFO_ID` and the media is read only. |
    /// | `EFI_WRITE_PROTECTED` | InformationType is `EFI_FILE_SYSTEM_VOLUME_LABEL_ID` and the media is read-only. |
    /// | `EFI_ACCESS_DENIED` | The `device` has no medium. |
    /// | `EFI_ACCESS_DENIED` | The device reported an error. |
    /// | `EFI_ACCESS_DENIED` | The file system structures are corrupted. |
    /// | `EFI_ACCESS_DENIED` | The `BufferSize` is too small to read the current directory entry. `BufferSize` has been updated with the size needed to complete the request. |
    /// | `EFI_VOLUME_FULL` | The `BufferSize` is too small to read the current directory entry. `BufferSize` has been updated with the size needed to complete the request. |
    /// | `EFI_BAD_BUFFER_SIZE` | The `BufferSize` is too small to read the current directory entry. `BufferSize` has been updated with the size needed to complete the request. |
    pub SetInfo: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        InformationType: *mut EFI_GUID,
        BufferSize: UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    /// Flushes all modified data associated with a file to a device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle to flush. |
    ///
    /// ## Description
    ///
    /// The `Flush()` function flushes all modified data associated with a file to a device.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The data was flushed. |
    /// | `EFI_NO_MEDIA` | The device has no medium. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error. |
    /// | `EFI_VOLUME_CORRUPTED` | The file system structures are corrupted. |
    /// | `EFI_WRITE_PROTECTED` | The file or medium is write-protected. |
    /// | `EFI_ACCESS_DENIED` | The file was opened read-only. |
    /// | `EFI_VOLUME_FULL` | The volume is full. |
    pub Flush: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
    ) -> EFI_STATUS,
    /// Opens a new file relative to the source directory’s location.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle to the source location. This would typically be an open handle to a directory. |
    /// | **OUT** `NewHandle` | A pointer to the location to return the opened handle for the new file. |
    /// | **IN** `FileName` | The null-terminated string of the name of the file to be opened. The file name may contain the following path modifiers: "", ".", and "..". |
    /// | **IN** `OpenMode` | The mode to open the file. The only valid combinations that the file may be opened with are: Read, Read/Write, or Create/Read/Write. |
    /// | **IN** `Attributes` | Only valid for EFI_FILE_MODE_CREATE, in which case these are the attribute bits for the newly created file. |
    /// | **IN OUT** `Token` | A pointer to the token associated with the transaction. |
    ///
    /// ## Description
    ///
    /// The `OpenEx()` function opens the file or directory referred to by `FileName` relative to the location of `This`
    /// and returns a `NewHandle`. The `FileName` may include the path modifiers described previously in `Open()`.
    ///
    /// If `EFI_FILE_MODE_CREATE` is set, then the file is created in the directory. If the final location of `FileName`
    /// does not refer to a directory, then the operation fails. If the file does not exist in the directory, then a new
    /// file is created. If the file already exists in the directory, then the existing file is opened.
    ///
    /// If the medium of the device changes, all accesses (including the File handle) will result in `EFI_MEDIA_CHANGED`.
    /// To access the new medium, the volume must be reopened.
    ///
    /// If an error is returned from the call to `OpenEx()` and non-blocking I/O is being requested, the `Event` associated
    /// with this request will not be signaled. If the call to `OpenEx()` succeeds then the Event will be signaled upon
    /// completion of the open or if an error occurs during the processing of the request. The status of the read request
    /// can be determined from the `Status` field of the `Token` once the event is signaled.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | Returned from the call `OpenEx()`: If `Event` is `NULL` (blocking I/O): The file was opened successfully. If Event is not `NULL` (asynchronous I/O): The request was successfully queued for processing. `Event` will be signaled upon completion. Returned in the token after signaling `Event`. The file was opened successfully. |
    /// | `EFI_NOT_FOUND` | The specified file could not be found on the device. |
    /// | `EFI_NO_MEDIA` | The device has no medium. |
    /// | `EFI_MEDIA_CHANGED` | The device has a different medium in it or the medium is no longer supported. |
    /// | `EFI_VOLUME_CORRUPTED` | The file system structures are corrupted. |
    /// | `EFI_WRITE_PROTECTED` | An attempt was made to create a file, or open a file for write when the media is write-protected. |
    /// | `EFI_ACCESS_DENIED` | The service denied access to the file. |
    /// | `EFI_OUT_OF_RESOURCES` | Not enough resources were available to open the file. |
    /// | `EFI_VOLUME_FULL` | The volume is full. |
    #[cfg(feature = "media-file-v2")]
    #[cfg_attr(doc, doc(cfg(feature = "media-file-v2")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "media-file-v2")))]
    pub OpenEx: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        NewHandle: *mut *mut EFI_FILE_PROTOCOL,
        FileName: *mut CHAR16,
        OpenMode: UINT64,
        Attributes: UINT64,
        Token: *mut EFI_FILE_IO_TOKEN,
    ) -> EFI_STATUS,
    /// Reads data from a file.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle to read data from. |
    /// | **IN OUT** `Token` | A pointer to the token associated with the transaction. |
    ///
    /// ## Description
    ///
    /// The `ReadEx()` function reads data from a file.
    ///
    /// If `This` is not a directory, the function reads the requested number of bytes from the file at the file’s current
    /// position and returns them in `Buffer`. If the read goes beyond the end of the file, the read length is truncated
    /// to the end of the file. The file’s current position is increased by the number of bytes returned.
    ///
    /// If `This` is a directory, the function reads the directory entry at the file’s current position and returns the
    /// entry in `Buffer`. If the `Buffer` is not large enough to hold the current directory entry, then `EFI_BUFFER_TOO_SMALL`
    /// is returned and the current file position is not updated. `BufferSize` is set to be the size of the buffer needed
    /// to read the entry. On success, the current position is updated to the next directory entry. If there are no more
    /// directory entries, the read returns a zero-length buffer. `EFI_FILE_INFO` is the structure returned as the
    /// directory entry.
    ///
    /// If non-blocking I/O is used the file pointer will be advanced based on the order that read requests were submitted.
    ///
    /// If an error is returned from the call to `ReadEx()` and non-blocking I/O is being requested, the `Event`
    /// associated with this request will not be signaled. If the call to `ReadEx()` succeeds then the `Event` will be
    /// signaled upon completion of the read or if an error occurs during the processing of the request. The status of
    /// the read request can be determined from the `Status` field of the Token once the event is signaled.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | Returned from the call `ReadEx()`: If `Event` is `NULL` (blocking I/O): The data was read successfully. If `Event` is not `NULL` (asynchronous I/O): The request was successfully queued for processing. Event will be signaled upon completion. Returned in the token after signaling `Event`. The data was read successfully. |
    /// | `EFI_NO_MEDIA` | The device has no medium. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error. |
    /// | `EFI_DEVICE_ERROR` | An attempt was made to read from a deleted file. |
    /// | `EFI_DEVICE_ERROR` | On entry, the current file position is beyond the end of the file. |
    /// | `EFI_VOLUME_CORRUPTED` | The file system structures are corrupted. |
    /// | `EFI_BUFFER_TOO_SMALL` | The `BufferSize` is too small to read the current directory entry. `BufferSize` has been updated with the size needed to complete the request. |
    #[cfg(feature = "media-file-v2")]
    #[cfg_attr(doc, doc(cfg(feature = "media-file-v2")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "media-file-v2")))]
    pub ReadEx: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        Token: *mut EFI_FILE_IO_TOKEN,
    ) -> EFI_STATUS,
    /// Writes data to a file.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle to write data to. |
    /// | **IN OUT** `Token` | A pointer to the token associated with the transaction. |
    ///
    /// ## Description
    ///
    /// The `WriteEx()` function writes the specified number of bytes to the file at the current file position. The
    /// current file position is advanced the actual number of bytes written, which is returned in `BufferSize`. Partial
    /// writes only occur when there has been a data error during the write attempt (such as “file space full”). The
    /// file is automatically grown to hold the data if required.
    ///
    /// Direct writes to opened directories are not supported.
    ///
    /// If non-blocking I/O is used the file pointer will be advanced based on the order that write requests were
    /// submitted.
    ///
    /// If an error is returned from the call to `WriteEx()` and non-blocking I/O is being requested, the `Event`
    /// associated with this request will not be signaled. If the call to `WriteEx()` succeeds then the `Event` will be
    /// signaled upon completion of the write or if an error occurs during the processing of the request. The status of
    /// the write request can be determined from the Status field of the `Token` once the event is signaled.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | Returned from the call WriteEx(): If `Event` is `NULL` (blocking I/O): The data was written successfully. If `Event` is not `NULL` (asynchronous I/O): The request was successfully queued for processing. Event will be signaled upon completion. Returned in the token after signaling `Event`. The data was written successfully. |
    /// | `EFI_UNSUPPORTED` | Writes to open directory files are not supported. |
    /// | `EFI_NO_MEDIA` | The device has no medium. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error. |
    /// | `EFI_DEVICE_ERROR` | An attempt was made to write to a deleted file. |
    /// | `EFI_VOLUME_CORRUPTED` | The file system structures are corrupted. |
    /// | `EFI_WRITE_PROTECTED` | The `BufferSize` is too small to read the current directory entry. `BufferSize` has been updated with the size needed to complete the request. |
    /// | `EFI_ACCESS_DENIED` | The file system structures are corrupted. |
    /// | `EFI_VOLUME_FULL` | The file system structures are corrupted. |
    #[cfg(feature = "media-file-v2")]
    #[cfg_attr(doc, doc(cfg(feature = "media-file-v2")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "media-file-v2")))]
    pub WriteEx: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        Token: *mut EFI_FILE_IO_TOKEN,
    ) -> EFI_STATUS,
    /// Flushes all modified data associated with a file to a device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_FILE_PROTOCOL` instance that is the file handle to flush. |
    /// | **IN OUT** `Token` | A pointer to the token associated with the transaction. |
    ///
    /// ## Description
    ///
    /// The `FlushEx()` function flushes all modified data associated with a file to a device.
    ///
    /// For non-blocking I/O all writes submitted before the flush request will be flushed.
    ///
    /// If an error is returned from the call to `FlushEx()` and non-blocking I/O is being requested, the `Event`
    /// associated with this request will not be signaled.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | Returned from the call `FlushEx()`: If `Event` is `NULL` (blocking I/O): The data was flushed successfully. If `Event` is not `NULL` (asynchronous I/O): The request was successfully queued for processing. Event will be signaled upon completion. Returned in the token after signaling `Event`. The data was flushed successfully. |
    /// | `EFI_NO_MEDIA` | The device has no medium. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error. |
    /// | `EFI_VOLUME_CORRUPTED` | The file system structures are corrupted. |
    /// | `EFI_WRITE_PROTECTED` | The file or medium is write-protected. |
    /// | `EFI_ACCESS_DENIED` | The file was opened read-only. |
    /// | `EFI_VOLUME_FULL` | The volume is full. |
    #[cfg(feature = "media-file-v2")]
    #[cfg_attr(doc, doc(cfg(feature = "media-file-v2")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "media-file-v2")))]
    pub FlushEx: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        Token: *mut EFI_FILE_IO_TOKEN,
    ) -> EFI_STATUS,
}

/// The `EFI_FILE_INFO` data structure supports `EFI_FILE_PROTOCOL.GetInfo()` and `EFI_FILE_PROTOCOL.SetInfo()` requests.
/// In the case of `SetInfo()`, the following additional rules apply:
///
/// - On directories, the file size is determined by the contents of the directory and cannot be changed by setting
/// `FileSize`. On directories, `FileSize` is ignored during a `SetInfo()`.
///
/// - The `PhysicalSize` is determined by the `FileSize` and cannot be changed. This value is ignored during a `SetInfo()`
/// request.
///
/// - The `EFI_FILE_DIRECTORY` attribute bit cannot be changed. It must match the file’s actual type.
///
/// - A value of zero in `CreateTime`, `LastAccess`, or `ModificationTime` causes the fields to be ignored (and not
/// updated).
#[repr(C)]
pub struct EFI_FILE_INFO<const N: UINTN = 0> {
    /// Size of the `EFI_FILE_INFO` structure, including the null-terminated `FileName` string.
    pub Size: UINT64,
    /// The size of the file in bytes.
    pub FileSize: UINT64,
    /// The amount of physical space the file consumes on the file system volume.
    pub PhysicalSize: UINT64,
    /// The time the file was created.
    pub CreateTime: EFI_TIME,
    /// The time when the file was last accessed.
    pub LastAccessTime: EFI_TIME,
    /// The time when the file’s contents were last modified.
    pub ModificationTime: EFI_TIME,
    /// The attribute bits for the file.
    pub Attribute: UINT64,
    /// The null-terminated name of the file. For a root directory, the name is an empty string.
    pub FileName: [CHAR16; N],
}

/// The `EFI_FILE_SYSTEM_INFO` data structure is an information structure that can be obtained on the root directory file
/// handle. The root directory file handle is the file handle first obtained on the initial call to the `EFI_BOOT_SERVICES.HandleProtocol()`
/// function to open the file system interface. All of the fields are read-only except for `VolumeLabel`. The system volume’s
/// `VolumeLabel` can be created or modified by calling `EFI_FILE_PROTOCOL.SetInfo()` with an updated `VolumeLabel` field.
#[repr(C)]
pub struct EFI_FILE_SYSTEM_INFO<const N: UINTN = 0> {
    /// Size of the `EFI_FILE_SYSTEM_INFO` structure, including the null-terminated `VolumeLabel` string.
    pub Size: UINT64,
    /// `TRUE` if the volume only supports read access.
    pub ReadOnly: BOOLEAN,
    /// The number of bytes managed by the file system.
    pub VolumeSize: UINT64,
    /// The number of available bytes for use by the file system.
    pub FreeSpace: UINT64,
    /// The nominal block size by which files are typically grown.
    pub BlockSize: UINT32,
    /// The null-terminated string that is the volume’s label.
    pub VolumeLabel: [CHAR16; N],
}

/// The `EFI_FILE_SYSTEM_VOLUME_LABEL` data structure is an information structure that can be obtained on the root directory
/// file handle. The root directory file handle is the file handle first obtained on the initial call to the `EFI_BOOT_SERVICES.HandleProtocol()`
/// function to open the file system interface. The system volume’s `VolumeLabel` can be created or modified by calling
/// `EFI_FILE_PROTOCOL.SetInfo()` with an updated `VolumeLabel` field.
#[repr(C)]
pub struct EFI_FILE_SYSTEM_VOLUME_LABEL<const N: UINTN = 0> {
    /// The null-terminated string that is the volume’s label.
    pub VolumeLabel: [CHAR16; N],
}

#[repr(C)]
pub struct EFI_FILE_IO_TOKEN {
    /// If `Event` is `NULL`, then blocking I/O is performed. If `Event` is not `NULL` and non-blocking I/O is supported,
    /// then non-blocking I/O is performed, and `Event` will be signaled when the read request is completed. The caller
    /// must be prepared to handle the case where the callback associated with Event occurs before the original asynchronous
    /// I/O request call returns.
    pub Event: EFI_EVENT,
    /// Defines whether or not the signaled event encountered an error.
    pub Status: EFI_STATUS,
    /// For `OpenEx()`: Not used, ignored.
    /// For `ReadEx()`:On input, the size of the `Buffer`. On output, the amount of data returned in `Buffer`. In both cases, the size is measured in bytes.
    /// For `WriteEx()`: On input, the size of the `Buffer`. On output, the amount of data actually written. In both cases, the size is measured in bytes.
    /// For `FlushEx()`: Not used, ignored.
    pub BufferSize: UINTN,
    /// For `OpenEx()`: Not used, ignored.
    /// For `ReadEx()`: The buffer into which the data is read.
    /// For `WriteEx()`: The buffer of data to write.
    /// For `FlushEx()`: Not Used, ignored.
    pub Buffer: *mut VOID,
}
