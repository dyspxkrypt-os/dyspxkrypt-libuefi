/*
 * Dyspxkrypt LibUEFI: Raw bindings of UEFI that conforms to the definitions of the UEFI Specification.
 * Copyright (C) 2023-2025 HTGAzureX1212.
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

use crate::partition::{EFI_PARTITION_ENTRY, MBR_PARTITION_RECORD};

pub const EFI_PARTITION_INFO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x8CF2F62C,
        0xBC9B,
        0x4821,
        [0x80, 0x8D, 0xEC, 0x9E, 0xC4, 0x21, 0xA1, 0xA0],
    )
};

pub const EFI_PARTITION_INFO_PROTOCOL_REVISION: UINT32 = 0x0001000;

pub const PARTITION_TYPE_OTHER: UINT32 = 0x00;
pub const PARTITION_TYPE_MBR: UINT32 = 0x01;
pub const PARTITION_TYPE_GPT: UINT32 = 0x02;

#[repr(C)]
pub struct EFI_PARTITION_INFO_PROTOCOL {
    pub Revision: UINT32,
    pub Type: UINT32,
    pub System: UINT8,
    #[doc(hidden)]
    Reserved: [UINT8; 7],
    pub Info: EFI_PARTITION_INFO_PROTOCOL_INNER,
}

#[repr(C)]
pub union EFI_PARTITION_INFO_PROTOCOL_INNER {
    pub Mbr: MBR_PARTITION_RECORD,
    pub Gpt: EFI_PARTITION_ENTRY,
}
