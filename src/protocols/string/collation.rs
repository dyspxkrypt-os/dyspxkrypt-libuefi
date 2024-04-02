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

pub const EFI_UNICODE_COLLATION_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xA4C751FC,
        0x23AE,
        0x4C3E,
        [0x92, 0xE9, 0x49, 0x64, 0xCF, 0x63, 0xF3, 0x49],
    )
};

#[repr(C)]
pub struct EFI_UNICODE_COLLATION_PROTOCOL {
    pub StriColl: unsafe extern "efiapi" fn(
        This: *mut EFI_UNICODE_COLLATION_PROTOCOL,
        s1: *mut CHAR16,
        s2: *mut CHAR16,
    ) -> INTN,
    pub MetaiMatch: unsafe extern "efiapi" fn(
        This: *mut EFI_UNICODE_COLLATION_PROTOCOL,
        String: *mut CHAR16,
        Pattern: *mut CHAR16,
    ) -> BOOLEAN,
    pub StrLwr: unsafe extern "efiapi" fn(
        This: *mut EFI_UNICODE_COLLATION_PROTOCOL,
        String: *mut CHAR16,
    ) -> EFI_STATUS,
    pub StrUpr: unsafe extern "efiapi" fn(
        This: *mut EFI_UNICODE_COLLATION_PROTOCOL,
        String: *mut CHAR16,
    ) -> EFI_STATUS,
    pub FatToStr: unsafe extern "efiapi" fn(
        This: *mut EFI_UNICODE_COLLATION_PROTOCOL,
        FatSize: UINTN,
        Fat: *mut CHAR8,
        String: *mut CHAR16,
    ) -> EFI_STATUS,
    pub StrToFat: unsafe extern "efiapi" fn(
        This: *mut EFI_UNICODE_COLLATION_PROTOCOL,
        String: *mut CHAR16,
        FatSize: UINTN,
        Fat: *mut CHAR8,
    ) -> EFI_STATUS,
}
