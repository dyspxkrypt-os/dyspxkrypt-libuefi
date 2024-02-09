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

use alloc::boxed::Box;

use crate::tables::runtime::EFI_TIME;
use crate::types::{BOOLEAN, CHAR16, EFI_EVENT, EFI_GUID, EFI_STATUS, UINT32, UINT64, UINTN, VOID};

pub const EFI_FILE_PROTOCOL_REVISION: UINT64 = 0x00010000;
pub const EFI_FILE_PROTOCOL_REVISION2: UINT64 = 0x00020000;
#[cfg(not(feature = "media-file-v2"))]
#[cfg_attr(doc, doc(not(feature = "media-file-v2")))]
#[cfg_attr(docsrs, doc(not(feature = "media-file-v2")))]
pub const EFI_FILE_PROTOCOL_LATEST_REVISION: UINT64 = EFI_FILE_PROTOCOL_REVISION;
#[cfg(feature = "media-file-v2")]
#[cfg_attr(doc, doc(cfg(feature = "media-file-v2")))]
#[cfg_attr(docsrs, doc(cfg(feature = "media-file-v2")))]
pub const EFI_FILE_PROTOCOL_LATEST_REVISION: UINT64 = EFI_FILE_PROTOCOL_REVISION2;

pub const EFI_FILE_MODE_READ: UINT64 = 0x0000000000000001;
pub const EFI_FILE_MODE_WRITE: UINT64 = 0x0000000000000002;
pub const EFI_FILE_MODE_CREATE: UINT64 = 0x8000000000000000;

pub const EFI_FILE_READ_ONLY: UINT64 = 0x0000000000000001;
pub const EFI_FILE_HIDDEN: UINT64 = 0x0000000000000002;
pub const EFI_FILE_SYSTEM: UINT64 = 0x0000000000000004;
pub const EFI_FILE_RESERVED: UINT64 = 0x0000000000000008;
pub const EFI_FILE_DIRECTORY: UINT64 = 0x0000000000000010;
pub const EFI_FILE_ARCHIVE: UINT64 = 0x0000000000000020;
pub const EFI_FILE_VALID_ATTR: UINT64 = 0x0000000000000037;

pub const EFI_FILE_INFO_ID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x09576E92,
        0x6D3f,
        0x11D2,
        [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

pub const EFI_FILE_SYSTEM_INFO_ID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x09576E93,
        0x6d3f,
        0x11d2,
        [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

pub const EFI_FILE_SYSTEM_VOLUME_LABEL_ID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xDB47D7D3,
        0xFE81,
        0x11D3,
        [0x9A, 0x35, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
    )
};

#[repr(C)]
pub struct EFI_FILE_PROTOCOL {
    pub Revision: UINT64,
    pub Open: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        NewHandle: *mut *mut EFI_FILE_PROTOCOL,
        FileName: *mut CHAR16,
        OpenMode: UINT64,
        Attributes: UINT64,
    ) -> EFI_STATUS,
    pub Close: unsafe extern "efiapi" fn(This: *mut EFI_FILE_PROTOCOL) -> EFI_STATUS,
    pub Delete: unsafe extern "efiapi" fn(This: *mut EFI_FILE_PROTOCOL) -> EFI_STATUS,
    pub Read: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub Write: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub GetPosition: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        Position: *mut UINT64,
    ) -> EFI_STATUS,
    pub SetPosition:
        unsafe extern "efiapi" fn(This: *mut EFI_FILE_PROTOCOL, Position: UINT64) -> EFI_STATUS,
    pub GetInfo: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        InformationType: *mut EFI_GUID,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub SetInfo: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        InformationType: *mut EFI_GUID,
        BufferSize: UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub Flush: unsafe extern "efiapi" fn(This: *mut EFI_FILE_PROTOCOL) -> EFI_STATUS,
    #[cfg(feature = "media-file-v2")]
    #[cfg_attr(doc, doc(cfg(feature = "media-file-v2")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "media-file-v2")))]
    pub OpenEx: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        NewHandle: *mut *mut EFI_FILE_PROTOCOL,
        FileName: *mut CHAR16,
        OpenMode: UINT64,
        Attributes: UINT64,
        Token: *mut EFI_FILE_IO_TOKEN,
    ) -> EFI_STATUS,
    #[cfg(feature = "media-file-v2")]
    #[cfg_attr(doc, doc(cfg(feature = "media-file-v2")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "media-file-v2")))]
    pub ReadEx: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        Token: *mut EFI_FILE_IO_TOKEN,
    ) -> EFI_STATUS,
    #[cfg(feature = "media-file-v2")]
    #[cfg_attr(doc, doc(cfg(feature = "media-file-v2")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "media-file-v2")))]
    pub WriteEx: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        Token: *mut EFI_FILE_IO_TOKEN,
    ) -> EFI_STATUS,
    #[cfg(feature = "media-file-v2")]
    #[cfg_attr(doc, doc(cfg(feature = "media-file-v2")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "media-file-v2")))]
    pub FlushEx: unsafe extern "efiapi" fn(
        This: *mut EFI_FILE_PROTOCOL,
        Token: *mut EFI_FILE_IO_TOKEN,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_FILE_INFO {
    pub Size: UINT64,
    pub FileSize: UINT64,
    pub PhysicalSize: UINT64,
    pub CreateTime: EFI_TIME,
    pub LastAccessTime: EFI_TIME,
    pub ModificationTime: EFI_TIME,
    pub Attribute: UINT64,
    pub FileName: Box<[CHAR16]>,
}

#[repr(C)]
pub struct EFI_FILE_SYSTEM_INFO {
    pub Size: UINT64,
    pub ReadOnly: BOOLEAN,
    pub VolumeSize: UINT64,
    pub FreeSpace: UINT64,
    pub BlockSize: UINT32,
    pub VolumeLabel: Box<[CHAR16]>,
}

#[repr(C)]
pub struct EFI_FILE_SYSTEM_VOLUME_LABEL {
    pub VolumeLabel: Box<[CHAR16]>,
}

#[repr(C)]
pub struct EFI_FILE_IO_TOKEN {
    pub Event: EFI_EVENT,
    pub Status: EFI_STATUS,
    pub BufferSize: UINTN,
    pub Buffer: *mut VOID,
}
