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

use crate::protocols::device_path::EFI_DEVICE_PATH_PROTOCOL;
use crate::types::{EFI_GUID, EFI_STATUS, UINT64};

pub const EFI_RAM_DISK_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xAB38A0DF,
        0x6873,
        0x44A9,
        [0x87, 0xE6, 0xD4, 0xEB, 0x56, 0x14, 0x84, 0x49],
    )
};

#[repr(C)]
pub struct EFI_RAM_DISK_PROTOCOL {
    pub Register: unsafe extern "efiapi" fn(
        RamDiskBase: UINT64,
        RamDiskSize: UINT64,
        RamDiskType: *mut EFI_GUID,
        ParentDevicePath: *mut EFI_DEVICE_PATH_PROTOCOL,
        DevicePath: *mut *mut EFI_DEVICE_PATH_PROTOCOL,
    ) -> EFI_STATUS,
    pub Unregister:
        unsafe extern "efiapi" fn(DevicePath: *mut EFI_DEVICE_PATH_PROTOCOL) -> EFI_STATUS,
}
