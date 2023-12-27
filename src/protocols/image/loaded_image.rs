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
use crate::types::{EFI_GUID, EFI_HANDLE, EFI_STATUS, UINT32, UINTN, VOID};

pub const EFI_LOADED_IMAGE_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x5B1B31A1,
        0x9562,
        0x11D2,
        [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

pub const EFI_LOADED_IMAGE_PROTOCOL_REVISION: UINT32 = 0x1000;

/// Protocol for obtaining information about a loaded image.
#[repr(C)]
pub struct EFI_LOADED_IMAGE_PROTOCOL {
    /// Defines the revision of the `EFI_LOADED_IMAGE_PROTOCOL` structure. All future revisions will
    /// be backward compatible to the current revision.
    pub Revision: UINT32,
    /// Parent image’s image handle. `NULL` if the image is loaded directly from the firmware’s boot
    /// manager.
    pub ParentHandle: EFI_HANDLE,
    /// The image’s EFI system table pointer.
    pub SystemTable: *mut EFI_SYSTEM_TABLE,
    /// The device handle that the EFI Image was loaded from.
    pub DeviceHandle: EFI_HANDLE,
    /// A pointer to the file path portion specific to DeviceHandle that the EFI Image was loaded
    /// from.
    pub FilePath: *mut EFI_DEVICE_PATH_PROTOCOL,
    #[doc(hidden)]
    Reserved: *mut VOID,
    /// The size in bytes of `LoadOptions`.
    pub LoadOptionsSize: UINT32,
    /// A pointer to the image’s binary load options.
    pub LoadOptions: *mut VOID,
    /// The base address at which the image was loaded.
    pub ImageBase: *mut VOID,
    /// The size in bytes of the loaded image.
    pub ImageSize: UINTN,
    /// The memory type that the code sections were loaded as.
    pub ImageCodeType: EFI_MEMORY_TYPE,
    /// The memory type that the data sections were loaded as.
    pub ImageDataType: EFI_MEMORY_TYPE,
    /// Unloads an image from memory.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `ImageHandle`          | The handle to the image to unload.                                                                         |
    ///
    /// ## Description
    ///
    /// The `Unload()` function is a callback that a driver registers to do cleanup when the
    /// `UnloadImage()` boot service function is called.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS`      | The image was unloaded.                                          |
    /// | `EFI_INVALID_PARAMETER` | The `ImageHandle` was not valid. |
    pub Unload: unsafe extern "efiapi" fn(ImageHandle: EFI_HANDLE) -> EFI_STATUS,
}
