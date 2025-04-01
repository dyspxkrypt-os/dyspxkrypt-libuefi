/*
 * Dyspxkrypt LibUEFI: Raw bindings of UEFI that conforms to the definitions of the UEFI Specification.
 * Copyright (C) 2023-2025 HTGAzureX1212.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub const EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x1ACED566,
        0x76ED,
        0x4218,
        [0xBC, 0x81, 0x76, 0x7F, 0x1F, 0x97, 0x7A, 0x89],
    )
};

pub const EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL_REVISION: UINT64 = 0x00020000;

#[repr(C)]
pub struct EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL {
    pub Revision: UINT64,
    pub Id: UINT64,
    pub ImageAddr: UINT64,
    pub ImageSize: UINT32,
    pub StringId: [CHAR8; 4],
    pub Type: UINT8,
    pub MajorVer: UINT8,
    pub MinorVer: UINT8,
    pub Ipv6Supported: BOOLEAN,
    pub IfNum: UINT16,
}
