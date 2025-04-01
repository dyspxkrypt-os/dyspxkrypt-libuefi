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

pub const EFI_DECOMPRESS_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xD8117CFE,
        0x94A6,
        0x11D4,
        [0x9A, 0x3A, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
    )
};

#[repr(C)]
pub struct EFI_DECOMPRESS_PROTOCOL {
    pub GetInfo: unsafe extern "efiapi" fn(
        This: *mut EFI_DECOMPRESS_PROTOCOL,
        Source: *mut VOID,
        SourceSize: UINT32,
        DestinationSize: *mut UINT32,
        ScratchSize: *mut UINT32,
    ) -> EFI_STATUS,
    pub Decompress: unsafe extern "efiapi" fn(
        This: *mut EFI_DECOMPRESS_PROTOCOL,
        Source: *mut VOID,
        SourceSize: UINT32,
        Destination: *mut VOID,
        DestinationSize: UINT32,
        Scratch: *mut VOID,
        ScratchSize: UINT32,
    ) -> EFI_STATUS,
}
