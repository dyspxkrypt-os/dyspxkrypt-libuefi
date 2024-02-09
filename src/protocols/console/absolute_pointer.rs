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

use crate::types::{BOOLEAN, EFI_EVENT, EFI_GUID, EFI_STATUS, UINT32, UINT64};

pub const EFI_ABSOLUTE_POINTER_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x8D59D32B,
        0xC655,
        0x4AE9,
        [0x9B, 0x15, 0xF2, 0x59, 0x04, 0x99, 0x2A, 0x43],
    )
};

pub const EFI_ABSP_SupportsAltActive: UINT32 = 0x00000001;
pub const EFI_ABSP_SupportsPressureAsZ: UINT32 = 0x00000002;

#[repr(C)]
pub struct EFI_ABSOLUTE_POINTER_PROTOCOL {
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_ABSOLUTE_POINTER_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
    pub GetState: unsafe extern "efiapi" fn(
        This: *mut EFI_ABSOLUTE_POINTER_PROTOCOL,
        State: *mut EFI_ABSOLUTE_POINTER_STATE,
    ) -> EFI_STATUS,
    pub WaitForInput: EFI_EVENT,
    pub Mode: *mut EFI_ABSOLUTE_POINTER_MODE,
}

#[repr(C)]
pub struct EFI_ABSOLUTE_POINTER_STATE {
    CurrentX: UINT64,
    CurrentY: UINT64,
    CurrentZ: UINT64,
    ActiveButtons: UINT32,
}

#[repr(C)]
pub struct EFI_ABSOLUTE_POINTER_MODE {
    AbsoluteMinX: UINT64,
    AbsoluteMinY: UINT64,
    AbsoluteMinZ: UINT64,
    AbsoluteMaxX: UINT64,
    AbsoluteMaxY: UINT64,
    AbsoluteMaxZ: UINT64,
    Attributes: UINT32,
}
