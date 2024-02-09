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

use crate::protocols::media::block::EFI_BLOCK_IO_MEDIA;
use crate::types::{BOOLEAN, EFI_GUID, EFI_STATUS, UINT64};

pub mod capabilities;

pub const EFI_BLOCK_IO_CRYPTO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xA00490BA,
        0x3F1A,
        0x4B4C,
        [0xAB, 0x90, 0x4F, 0xA9, 0x97, 0x26, 0xA1, 0xE8],
    )
};

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_PROTOCOL {
    pub Media: *mut EFI_BLOCK_IO_MEDIA,
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO_CRYPTO_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_CAPABILITY {
    pub Algorithm: EFI_GUID,
    pub KeySize: UINT64,
    pub CryptoBlockSizeBitMask: UINT64,
}

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_IV_INPUT {
    pub InputSize: UINT64,
}
