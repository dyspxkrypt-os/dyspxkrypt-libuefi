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

pub const EFI_STORAGE_SECURITY_COMMAND_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xc88B0B6D,
        0x0DFC,
        0x49A7,
        [0x9C, 0xB4, 0x49, 0x07, 0x4B, 0x4C, 0x3A, 0x78],
    )
};

#[repr(C)]
pub struct EFI_STORAGE_SECURITY_COMMAND_PROTOCOL {
    pub ReceiveData: unsafe extern "efiapi" fn(
        This: *mut EFI_STORAGE_SECURITY_COMMAND_PROTOCOL,
        MediaId: UINT32,
        Timeout: UINT64,
        SecurityProtocolId: UINT8,
        SecurityProtocolSpecificData: UINT16,
        PayloadBufferSize: UINTN,
        PayloadBuffer: *mut VOID,
        PayloadTransferSize: *mut UINTN,
    ) -> EFI_STATUS,
    pub SendData: unsafe extern "efiapi" fn(
        This: *mut EFI_STORAGE_SECURITY_COMMAND_PROTOCOL,
        Timeout: UINT64,
        MediaId: UINT32,
        SecurityProtocolId: UINT8,
        SecurityProtocolSpecificData: UINT16,
        PayloadBufferSize: UINTN,
        PayloadBuffer: *mut VOID,
    ) -> EFI_STATUS,
}
