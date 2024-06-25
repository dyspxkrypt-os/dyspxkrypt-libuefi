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

pub const EFI_SIMPLE_NETWORK_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xA19832B9,
        0xAC25,
        0x11D3,
        [0x9A, 0x2D, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
    )
};

pub const EFI_SIMPLE_NETWORK_PROTOCOL_REVISION: UINT64 = 0x00010000;

#[repr(C)]
pub struct EFI_SIMPLE_NETWORK_PROTOCOL {
    pub Revision: UINT64,
    pub Start: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_NETWORK_PROTOCOL,
    ) -> EFI_STATUS,
    pub Stop: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_NETWORK_PROTOCOL,
    ) -> EFI_STATUS,
    pub Initialize: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_NETWORK_PROTOCOL,
        ExtraRxBufferSize: UINTN,
        ExtraTxBufferSize: UINTN,
    ) -> EFI_STATUS,
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_NETWORK_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
    pub Shutdown: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_NETWORK_PROTOCOL,
    ) -> EFI_STATUS,
    pub ReceiveFilters: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_NETWORK_PROTOCOL,
        Enable: UINT32,
        Disable: UINT32,
        ResetMCastFilter: BOOLEAN,
        MCastFilterCnt: UINTN,
        MCastFilter: EFI_MAC_ADDRESS,
    ) -> EFI_STATUS,
    pub StationAddress: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_NETWORK_PROTOCOL,
        Reset: BOOLEAN,
        New: *mut EFI_MAC_ADDRESS,
    ) -> EFI_STATUS,
}
