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

pub const EFI_BTT_ABSTRACTION_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x18633BFC,
        0x1735,
        0x4217,
        [0x8A, 0xC9, 0x17, 0x23, 0x92, 0x82, 0xD3, 0xF8],
    )
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EFI_BTT_INFO_BLOCK {
    pub Sig: [CHAR8; 16],
    pub Uuid: EFI_GUID,
    pub ParentUuid: EFI_GUID,
    pub Flags: UINT32,
    pub Major: UINT16,
    pub Minor: UINT16,
    pub ExternalLbaSize: UINT32,
    pub ExternalNLba: UINT32,
    pub InternalLbaSize: UINT32,
    pub InternalNLba: UINT32,
    pub NFree: UINT32,
    pub InfoSize: UINT32,
    pub NextOff: UINT64,
    pub DataOff: UINT64,
    pub MapOff: UINT64,
    pub FlogOff: UINT64,
    pub InfoOff: UINT64,
    pub Unused: [CHAR8; 3968],
    pub Checksum: UINT64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EFI_BTT_FLOG {
    pub Lba0: UINT32,
    pub OldMap0: UINT32,
    pub NewMap0: UINT32,
    pub Seq0: UINT32,
    pub Lba1: UINT32,
    pub OldMap1: UINT32,
    pub NewMap1: UINT32,
    pub Seq1: UINT32,
}
