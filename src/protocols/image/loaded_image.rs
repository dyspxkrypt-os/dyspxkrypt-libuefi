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

use crate::protocols::device_path::EFI_DEVICE_PATH_PROTOCOL;
use crate::tables::boot::EFI_MEMORY_TYPE;
use crate::tables::system::EFI_SYSTEM_TABLE;

pub const EFI_LOADED_IMAGE_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x5B1B31A1,
        0x9562,
        0x11D2,
        [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

pub const EFI_LOADED_IMAGE_PROTOCOL_REVISION: UINT32 = 0x1000;

#[repr(C)]
pub struct EFI_LOADED_IMAGE_PROTOCOL {
    pub Revision: UINT32,
    pub ParentHandle: EFI_HANDLE,
    pub SystemTable: *mut EFI_SYSTEM_TABLE,
    pub DeviceHandle: EFI_HANDLE,
    pub FilePath: *mut EFI_DEVICE_PATH_PROTOCOL,
    #[doc(hidden)]
    Reserved: *mut VOID,
    pub LoadOptionsSize: UINT32,
    pub LoadOptions: *mut VOID,
    pub ImageBase: *mut VOID,
    pub ImageSize: UINTN,
    pub ImageCodeType: EFI_MEMORY_TYPE,
    pub ImageDataType: EFI_MEMORY_TYPE,
    pub Unload: unsafe extern "efiapi" fn(ImageHandle: EFI_HANDLE) -> EFI_STATUS,
}
