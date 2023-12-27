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

use crate::types::{EFI_GUID, UINT32};

pub const EFI_SERIAL_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xBB25CF6F,
        0xF1D4,
        0x11D2,
        [0x9A, 0x0C, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0xFD],
    )
};

pub const EFI_SERIAL_IO_PROTOCOL_REVISION: UINT32 = 0x00010000;
pub const EFI_SERIAL_IO_PROTOCOL_REVISION1p1: UINT32 = 0x00010001;

/// This protocol is used to communicate with any type of character-based I/O device.
#[repr(C)]
pub struct EFI_SERIAL_IO_PROTOCOL {
    /// The revision to which the `EFI_SERIAL_IO_PROTOCOL` adheres. All future revisions must be
    /// backwards compatible. If a future version is not back wards compatible, it is not the same
    /// GUID.
    pub Revision: UINT32,
}
