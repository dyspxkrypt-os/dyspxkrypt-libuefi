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

use crate::types::{CHAR16, EFI_STATUS, UINT64, UINTN, VOID};

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
    pub Close: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
    ) -> EFI_STATUS,
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
    pub Delete: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
    ) -> EFI_STATUS,
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
}
