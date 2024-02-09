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

use crate::protocols::media::block::{EFI_BLOCK_IO_MEDIA, EFI_LBA};
use crate::types::{BOOLEAN, EFI_EVENT, EFI_GUID, EFI_STATUS, UINT32, UINTN, VOID};

pub const EFI_BLOCK_IO2_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xA77B2472,
        0xE282,
        0x4E9F,
        [0xA2, 0x45, 0xC2, 0xC0, 0xE2, 0x7B, 0xBC, 0xC1],
    )
};

#[repr(C)]
pub struct EFI_BLOCK_IO2_PROTOCOL {
    pub Media: *mut EFI_BLOCK_IO_MEDIA,
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO2_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
    pub ReadBlocksEx: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO2_PROTOCOL,
        MediaId: UINT32,
        LBA: EFI_LBA,
        Token: *mut EFI_BLOCK_IO2_TOKEN,
        BufferSize: UINTN,
        Buffer: *mut VOID,
    ),
    pub WriteBlocksEx: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO2_PROTOCOL,
        MediaId: UINT32,
        LBA: EFI_LBA,
        Token: *mut EFI_BLOCK_IO2_TOKEN,
        BufferSize: UINTN,
        Buffer: *mut VOID,
    ),
    pub FlushBlocksEx: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO2_PROTOCOL,
        Token: *mut EFI_BLOCK_IO2_TOKEN,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_BLOCK_IO2_TOKEN {
    pub Event: EFI_EVENT,
    pub TransactionStatus: EFI_STATUS,
}
