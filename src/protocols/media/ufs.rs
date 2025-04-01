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

pub const EFI_UFS_DEVICE_CONFIG_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xB81BFAB0,
        0x0EB3,
        0x4CF9,
        [0x84, 0x65, 0x7F, 0xA9, 0x86, 0x36, 0x16, 0x64],
    )
};

#[repr(C)]
pub struct EFI_UFS_DEVICE_CONFIG_PROTOCOL {
    pub RwUfsDescriptor: unsafe extern "efiapi" fn(
        This: *mut EFI_UFS_DEVICE_CONFIG_PROTOCOL,
        Read: BOOLEAN,
        DescId: UINT8,
        Index: UINT8,
        Selector: UINT8,
        Descriptor: UINT8,
        DescSize: *mut UINT32,
    ) -> EFI_STATUS,
    pub RwUfsFlag: unsafe extern "efiapi" fn(
        This: *mut EFI_UFS_DEVICE_CONFIG_PROTOCOL,
        Read: BOOLEAN,
        FlagId: UINT8,
        Flag: *mut UINT8,
    ) -> EFI_STATUS,
    pub RwUfsAttribute: unsafe extern "efiapi" fn(
        This: *mut EFI_UFS_DEVICE_CONFIG_PROTOCOL,
        Read: BOOLEAN,
        AttrId: UINT8,
        Index: UINT8,
        Selector: UINT8,
        Attribute: *mut UINT8,
        AttrSize: *mut UINT32,
    ) -> EFI_STATUS,
}
