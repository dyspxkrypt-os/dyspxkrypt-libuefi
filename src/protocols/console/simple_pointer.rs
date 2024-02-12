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

pub const EFI_SIMPLE_POINTER_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x31878C87,
        0x0B75,
        0x11D5,
        [0x9A, 0x4F, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
    )
};

#[repr(C)]
pub struct EFI_SIMPLE_POINTER_PROTOCOL {
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_POINTER_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
    pub GetState: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_POINTER_PROTOCOL,
        State: *mut EFI_SIMPLE_POINTER_STATE,
    ) -> EFI_STATUS,
    pub WaitForInput: EFI_EVENT,
    pub Mode: *mut EFI_SIMPLE_INPUT_MODE,
}

#[repr(C)]
pub struct EFI_SIMPLE_POINTER_STATE {
    pub RelativeMovementX: INT32,
    pub RelativeMovementY: INT32,
    pub RelativeMovementZ: INT32,
    pub LeftButton: BOOLEAN,
    pub RightButton: BOOLEAN,
}

#[repr(C)]
pub struct EFI_SIMPLE_INPUT_MODE {
    pub RelativeMovementX: INT32,
    pub RelativeMovementY: INT32,
    pub RelativeMovementZ: INT32,
    pub LeftButton: BOOLEAN,
    pub RightButton: BOOLEAN,
}
