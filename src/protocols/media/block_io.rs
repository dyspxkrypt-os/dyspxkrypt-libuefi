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

use crate::types::{EFI_GUID, UINT64};

pub const EFI_BLOCK_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x964E5B21,0x6459,0x11D2,
        [0x8E,0x39,0x00,0xA0,0xC9,0x69,0x72,0x3B]
    )
};

pub const EFI_BLOCK_IO_PROTOCOL_REVISION: UINT64 = 0x00010000;
#[cfg(feature = "media-block-v2")]
#[cfg_attr(doc, doc(cfg(feature = "media-block-v2")))]
#[cfg_attr(docsrs, doc(cfg(feature = "media-block-v2")))]
pub const EFI_BLOCK_IO_PROTOCOL_REVISION2: UINT64 = 0x00020000;
#[cfg(feature = "media-block-v3")]
#[cfg_attr(doc, doc(cfg(feature = "media-block-v3")))]
#[cfg_attr(docsrs, doc(cfg(feature = "media-block-v3")))]
pub const EFI_BLOCK_IO_PROTOCOL_REVISION3: UINT64 = (2 << 16) | 31;

/// This protocol provides control over block devices.
///
/// This section defines the Block I/O protocol. This protocol is used to abstract mass storage
/// devices to allow code running in the EFI boot services environment to access them without
/// specific knowledge of the type of device or controller that manages the device. Functions are
/// defined to read and write data at a block level from mass storage devices as well as to manage
/// such devices in the EFI boot services environment.
#[repr(C)]
pub struct EFI_BLOCK_IO_PROTOCOL {
    /// The revision to which the block IO interface adheres.
    pub Revision: UINT64,
}
