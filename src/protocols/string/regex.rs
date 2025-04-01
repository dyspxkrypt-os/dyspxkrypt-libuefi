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

pub const EFI_UNICODE_COLLATION_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xB3F79D9A,
        0x436C,
        0xDC11,
        [0xB0, 0x52, 0xCD, 0x85, 0xDF, 0x52, 0x4C, 0xE6],
    )
};

#[repr(C)]
pub struct EFI_REGEX_CAPTURE {
    pub CapturePtr: *const CHAR16,
    pub Length: UINTN,
}

#[repr(C)]
pub struct EFI_REGULAR_EXPRESSION_PROTOCOL {
    pub MatchString: unsafe extern "efiapi" fn(
        This: *mut EFI_REGULAR_EXPRESSION_PROTOCOL,
        String: *mut CHAR16,
        Pattern: *mut CHAR16,
        SyntaxType: *mut EFI_REGEX_SYNTAX_TYPE,
        Result: *mut BOOLEAN,
        Captures: *mut *mut EFI_REGEX_CAPTURE,
        CapturesCount: *mut UINTN,
    ) -> EFI_STATUS,
    pub GetInfo: unsafe extern "efiapi" fn(
        This: *mut EFI_REGULAR_EXPRESSION_PROTOCOL,
        RegExSyntaxTypeListSize: *mut UINTN,
        RegExSyntaxTypeList: *mut EFI_REGEX_SYNTAX_TYPE,
    ) -> EFI_STATUS,
}

pub type EFI_REGEX_SYNTAX_TYPE = EFI_GUID;
