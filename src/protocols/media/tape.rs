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

use crate::types::{EFI_GUID, EFI_STATUS, UINTN, VOID};

pub const EFI_TAPE_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x1E93E633,
        0xD65A,
        0x459E,
        [0xAB, 0x84, 0x93, 0xD9, 0xEC, 0x26, 0x6D, 0x18],
    )
};

/// The EFI Tape IO protocol provides services to control and access a tape device.
#[repr(C)]
pub struct EFI_TAPE_IO_PROTOCOL {
    pub TapeRead: unsafe extern "efiapi" fn(
        This: *mut EFI_TAPE_IO_PROTOCOL,
        BufferSize: UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
}
