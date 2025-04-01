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

use crate::protocols::device_path::EFI_DEVICE_PATH_PROTOCOL;

pub const EFI_DEVICE_PATH_UTILITIES_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x0379BE4E,
        0xD706,
        0x437D,
        [0xB0, 0x37, 0xED, 0xB8, 0x2F, 0xB7, 0x72, 0xA4],
    )
};

#[repr(C)]
pub struct EFI_DEVICE_PATH_UTILITIES_PROTOCOL {
    pub GetDevicePathSize:
        unsafe extern "efiapi" fn(DevicePath: *const EFI_DEVICE_PATH_PROTOCOL) -> UINTN,
    pub DuplicateDevicePath: unsafe extern "efiapi" fn(
        DevicePath: *const EFI_DEVICE_PATH_UTILITIES_PROTOCOL,
    )
        -> *mut EFI_DEVICE_PATH_UTILITIES_PROTOCOL,
    pub AppendDevicePath: unsafe extern "efiapi" fn(
        Src1: *const EFI_DEVICE_PATH_PROTOCOL,
        Src2: *const EFI_DEVICE_PATH_PROTOCOL,
    ) -> *mut EFI_DEVICE_PATH_PROTOCOL,
    pub AppendDeviceNode: unsafe extern "efiapi" fn(
        DevicePath: *const EFI_DEVICE_PATH_PROTOCOL,
        DeviceNode: *const EFI_DEVICE_PATH_PROTOCOL,
    ) -> *mut EFI_DEVICE_PATH_PROTOCOL,
    pub AppendDevicePathInstance: unsafe extern "efiapi" fn(
        DevicePath: *const EFI_DEVICE_PATH_PROTOCOL,
        DevicePathInstance: *const EFI_DEVICE_PATH_PROTOCOL,
    ) -> *mut EFI_DEVICE_PATH_PROTOCOL,
    pub GetNextDevicePathInstance: unsafe extern "efiapi" fn(
        DevicePathInstance: *mut *mut EFI_DEVICE_PATH_PROTOCOL,
        DevicePathInstanceSize: *mut UINTN,
    ) -> *mut EFI_DEVICE_PATH_PROTOCOL,
    pub CreateDeviceNode: unsafe extern "efiapi" fn(
        NodeType: UINT8,
        NodeSubType: UINT8,
        NodeLength: UINT16,
    ) -> *mut EFI_DEVICE_PATH_PROTOCOL,
    pub IsDevicePathMultiInstance:
        unsafe extern "efiapi" fn(DevicePath: *const EFI_DEVICE_PATH_PROTOCOL) -> BOOLEAN,
}
