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

pub const EFI_ISCSI_INITIATOR_NAME_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x59324945,
        0xEC44,
        0x4C0D,
        [0xB1, 0xCD, 0x9D, 0xB1, 0x39, 0xDF, 0x07, 0x0C],
    )
};

#[repr(C)]
pub struct EFI_ISCSI_INITIATOR_NAME_PROTOCOL {
    pub Get: unsafe extern "efiapi" fn(
        This: *mut EFI_ISCSI_INITIATOR_NAME_PROTOCOL,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub Set: unsafe extern "efiapi" fn(
        This: *mut EFI_ISCSI_INITIATOR_NAME_PROTOCOL,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
}
