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

use crate::types::{UINT64};

pub const EFI_FILE_PROTOCOL_REVISION: UINT64 = 0x00010000;
pub const EFI_FILE_PROTOCOL_REVISION2: UINT64 = 0x00020000;
#[cfg(not(feature = "media-file-v2"))]
#[cfg_attr(doc, doc(not(feature = "media-file-v2")))]
#[cfg_attr(docsrs, doc(not(feature = "media-file-v2")))]
pub const EFI_FILE_PROTOCOL_LATEST_REVISION: UINT32 = EFI_FILE_PROTOCOL_REVISION;
#[cfg(feature = "media-file-v2")]
#[cfg_attr(doc, doc(cfg(feature = "media-file-v2")))]
#[cfg_attr(docsrs, doc(cfg(feature = "media-file-v2")))]
pub const EFI_FILE_PROTOCOL_LATEST_REVISION: UINT64 = EFI_FILE_PROTOCOL_REVISION2;

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
}
