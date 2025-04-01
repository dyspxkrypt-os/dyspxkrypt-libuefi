/*
 * Dyspxkrypt LibUEFI: Raw bindings of UEFI that conforms to the definitions of the UEFI Specification.
 * Copyright (C) 2023-2024 HTGAzureX1212.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * at your option) any later version.
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

pub const EFI_DRIVER_BINDING_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x18A031AB,
        0xB443,
        0x4D1A,
        [0xA5, 0xC0, 0x0C, 0x09, 0x26, 0x1E, 0x9F, 0x71],
    )
};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EFI_DRIVER_BINDING_PROTOCOL {
    pub Supported: unsafe extern "efiapi" fn(
        This: *mut EFI_DRIVER_BINDING_PROTOCOL,
        ControllerHandle: EFI_HANDLE,
        RemainingDevicePath: *mut EFI_DEVICE_PATH_PROTOCOL,
    ) -> EFI_STATUS,
    pub Start: unsafe extern "efiapi" fn(
        This: *mut EFI_DRIVER_BINDING_PROTOCOL,
        ControllerHandle: EFI_HANDLE,
        RemainingDevicePath: *mut EFI_DEVICE_PATH_PROTOCOL,
    ) -> EFI_STATUS,
    pub Stop: unsafe extern "efiapi" fn(
        This : *mut EFI_DRIVER_BINDING_PROTOCOL,
        ControllerHandle: EFI_HANDLE,
        NumberOfChildren: UINTN,
        ChildHandleBuffer: *mut EFI_HANDLE,
    ) -> EFI_STATUS,
    pub Version: UINT32,
    pub ImageHandle: EFI_HANDLE,
    pub DriverBindingHandle: EFI_HANDLE,
}
