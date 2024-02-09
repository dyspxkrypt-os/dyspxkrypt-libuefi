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

use crate::types::{EFI_GUID, EFI_STATUS, UINT32, UINT64, UINTN, VOID};

pub const EFI_DISK_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xCE345171,
        0xBA0B,
        0x11D2,
        [0x8E, 0x4F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

pub const EFI_DISK_IO_PROTOCOL_REVISION: UINT64 = 0x00010000;

#[repr(C)]
pub struct EFI_DISK_IO_PROTOCOL {
    pub Revision: UINT64,
    pub ReadDisk: unsafe extern "efiapi" fn(
        This: *mut EFI_DISK_IO_PROTOCOL,
        MediaId: UINT32,
        Offset: UINT64,
        BufferSize: UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub WriteDisk: unsafe extern "efiapi" fn(
        This: *mut EFI_DISK_IO_PROTOCOL,
        MediaId: UINT32,
        Offset: UINT64,
        BufferSize: UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
}
