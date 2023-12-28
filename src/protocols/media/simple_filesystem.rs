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

use crate::protocols::media::EFI_FILE_PROTOCOL;
use crate::types::{EFI_GUID, EFI_STATUS, UINT64};

pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x0964E5B22,
        0x6459,
        0x11D2,
        [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_REVISION: UINT64 = 0x00010000;

/// This protocol provides a minimal interface for file-type access to a device.
///
/// The `EFI_SIMPLE_FILE_SYSTEM_PROTOCOL` provides a minimal interface for file-type access to a device.
/// This protocol is only supported on some devices.
///
/// Devices that support the Simple File System protocol return an `EFI_FILE_PROTOCOL`. The only
/// function of this interface is to open a handle to the root directory of the file system on the
/// volume. Once opened, all accesses to the volume are performed through the volume’s file handles,
/// using the `EFI_FILE_PROTOCOL` protocol. The volume is closed by closing all the open file handles.
///
/// The firmware automatically creates handles for any block device that supports the following file
/// system formats:
///
/// - FAT12
///
/// - FAT16
///
/// - FAT32
#[repr(C)]
pub struct EFI_SIMPLE_FILE_SYSTEM_PROTOCOL {
    /// The version of the `EFI_SIMPLE_FILE_SYSTEM_PROTOCOL`. The version specified by
    /// specification is `0x00010000`. All future revisions must be backwards compatible. If a future
    /// version is not backwards compatible, it is not the same GUID.
    pub Revision: UINT64,
    /// Opens the root directory on a volume.
    pub OpenVolume: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_FILE_SYSTEM_PROTOCOL,
        Root: *mut *mut EFI_FILE_PROTOCOL,
    ) -> EFI_STATUS,
}
