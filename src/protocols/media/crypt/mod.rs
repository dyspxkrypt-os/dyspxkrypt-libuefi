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

use alloc::boxed::Box;

use crate::protocols::media::block::{EFI_BLOCK_IO_MEDIA, EFI_LBA};

pub mod capabilities;

pub const EFI_BLOCK_IO_CRYPTO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xA00490BA,
        0x3F1A,
        0x4B4C,
        [0xAB, 0x90, 0x4F, 0xA9, 0x97, 0x26, 0xA1, 0xE8],
    )
};

pub const EFI_BLOCK_IO_CRYPTO_INDEX_ANY: UINT64 = 0xFFFFFFFFFFFFFFFF;

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_PROTOCOL {
    pub Media: *mut EFI_BLOCK_IO_MEDIA,
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO_CRYPTO_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
    pub GetCapabilities: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO_CRYPTO_PROTOCOL,
        Capabilities: *mut EFI_BLOCK_IO_CRYPTO_CAPABILITIES,
    ) -> EFI_STATUS,
    pub SetConfiguration: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO_CRYPTO_PROTOCOL,
        ConfigurationCount: UINT64,
        ConfigurationTable: *mut EFI_BLOCK_IO_CRYPTO_CONFIGURATION_TABLE_ENTRY,
        ResultingTable: *mut EFI_BLOCK_IO_CRYPTO_CONFIGURATION_TABLE_ENTRY,
    ) -> EFI_STATUS,
    pub GetConfiguration: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO_CRYPTO_PROTOCOL,
        StartIndex: UINT64,
        ConfigurationCount: UINT64,
        KeyOwnerGuid: *mut EFI_GUID,
        ConfigurationTable: *mut EFI_BLOCK_IO_CRYPTO_CONFIGURATION_TABLE_ENTRY,
    ) -> EFI_STATUS,
    pub ReadExtended: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO_CRYPTO_PROTOCOL,
        MediaId: UINT32,
        LBA: EFI_LBA,
        Token: *mut EFI_BLOCK_IO_CRYPTO_TOKEN,
        BufferSize: UINT64,
        Buffer: *mut VOID,
        Index: *mut UINT64,
        CryptoIvInput: *mut VOID,
    ) -> EFI_STATUS,
    pub WriteExtended: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO_CRYPTO_PROTOCOL,
        MediaId: UINT32,
        LBA: EFI_LBA,
        Token: *mut EFI_BLOCK_IO_CRYPTO_TOKEN,
        BufferSize: UINT64,
        Buffer: *mut VOID,
        Index: *mut UINT64,
        CryptoIvInput: *mut VOID,
    ) -> EFI_STATUS,
    pub FlushBlocks: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO_CRYPTO_PROTOCOL,
        Token: *mut EFI_BLOCK_IO_CRYPTO_TOKEN,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_CAPABILITIES {
    pub Supported: BOOLEAN,
    pub KeyCount: UINT64,
    pub CapabilityCount: UINT64,
    pub Capabilities: Box<[EFI_BLOCK_IO_CRYPTO_CAPABILITY]>,
}

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_CAPABILITY {
    pub Algorithm: EFI_GUID,
    pub KeySize: UINT64,
    pub CryptoBlockSizeBitMask: UINT64,
}

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_CONFIGURATION_TABLE_ENTRY {
    pub Index: UINT64,
    pub KeyOwnerGuid: EFI_GUID,
    pub Capability: EFI_BLOCK_IO_CRYPTO_CAPABILITY,
    pub CryptoKey: *mut VOID,
}

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_IV_INPUT {
    pub InputSize: UINT64,
}

#[repr(C)]
pub struct EFI_BLOCK_IO_CRYPTO_TOKEN {
    pub Event: EFI_EVENT,
    pub TransactionStatus: EFI_STATUS,
}
