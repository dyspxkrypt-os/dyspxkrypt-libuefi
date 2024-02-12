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

use crate::protocols::media::crypt::EFI_BLOCK_IO_CRYPTO_IV_INPUT;

#[cfg(feature = "media-crypt-aes-cbc")]
pub const EFI_BLOCK_IO_CRYPTO_ALGO_GUID_AES_CBC_MICROSOFT_BITLOCKER: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x689E4C62,
        0x70BF,
        0x4CF3,
        [0x88, 0xBB, 0x33, 0xB3, 0x18, 0x26, 0x86, 0x70],
    )
};

#[cfg(feature = "media-crypt-aes-xts")]
pub const EFI_BLOCK_IO_CRYPTO_ALGO_GUID_AES_XTS: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x2F87BA6A,
        0x5C04,
        0x4385,
        [0xA7, 0x80, 0xF3, 0xBF, 0x78, 0xA9, 0x7B, 0xEC],
    )
};

#[cfg(feature = "media-crypt-aes-cbc")]
#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_IV_INPUT_AES_CBC_MICROSOFT_BITLOCKER {
    pub Header: EFI_BLOCK_IO_CRYPTO_IV_INPUT,
    pub CryptoBlockByteOffset: UINT64,
    pub CryptoBlockByteSize: UINT64,
}

#[cfg(feature = "media-crypt-aes-xts")]
#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_IV_INPUT_AES_XTS {
    pub Header: EFI_BLOCK_IO_CRYPTO_IV_INPUT,
    pub CryptoBlockNumber: UINT64,
    pub CryptoBlockByteSize: UINT64,
}
