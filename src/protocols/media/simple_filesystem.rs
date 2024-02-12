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

use crate::protocols::media::EFI_FILE_PROTOCOL;

pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x0964E5B22,
        0x6459,
        0x11D2,
        [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_REVISION: UINT64 = 0x00010000;

#[repr(C)]
pub struct EFI_SIMPLE_FILE_SYSTEM_PROTOCOL {
    pub Revision: UINT64,
    pub OpenVolume: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_FILE_SYSTEM_PROTOCOL,
        Root: *mut *mut EFI_FILE_PROTOCOL,
    ) -> EFI_STATUS,
}
