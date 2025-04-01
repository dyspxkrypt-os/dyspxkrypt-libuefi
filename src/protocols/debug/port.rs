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

pub const EFI_DEBUGPORT_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xEBA4E8D2,
        0x3858,
        0x41EC,
        [0xA2, 0x81, 0x26, 0x47, 0xBA, 0x96, 0x60, 0xD0],
    )
};

#[repr(C)]
pub struct EFI_DEBUGPORT_PROTOCOL {
    pub Reset: unsafe extern "efiapi" fn(This: *mut EFI_DEBUGPORT_PROTOCOL) -> EFI_STATUS,
    pub Write: unsafe extern "efiapi" fn(
        This: *mut EFI_DEBUGPORT_PROTOCOL,
        Timeout: UINT32,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub Read: unsafe extern "efiapi" fn(
        This: *mut EFI_DEBUGPORT_PROTOCOL,
        Timeout: UINT32,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub Poll: unsafe extern "efiapi" fn(This: *mut EFI_DEBUGPORT_PROTOCOL) -> EFI_STATUS,
}
