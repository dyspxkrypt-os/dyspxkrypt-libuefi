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

pub const EFI_DEBUG_SUPPORT_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x2755590C,
        0x6F3C,
        0x42FA,
        [0x9E, 0xA4, 0xA3, 0xBA, 0x54, 0x3C, 0xDA, 0x25],
    )
};

#[repr(C)]
pub enum EFI_INSTRUCTION_SET_ARCHITECTURE {
    IsaIa32 = 0x014C,
    IsaX64 = 0x8664,
    IsaIpf = 0x0200,
    IsaEbc = 0x0EBC,
    IsaArm = 0x01C2,
    IsaAArch64 = 0xAA64,
    IsaRISCV32 = 0x5032,
    IsaRISCV64 = 0x5064,
    IsaRISCV128 = 0x5128,
    IsaLoongArch32 = 0x6232,
    IsaLoongArch64 = 0x6264,
}

#[repr(C)]
pub struct EFI_DEBUG_SUPPORT_PROTOCOL {
    pub Isa: EFI_INSTRUCTION_SET_ARCHITECTURE,
    pub GetMaximumProcessorIndex: unsafe extern "efiapi" fn(
        This: *mut EFI_DEBUG_SUPPORT_PROTOCOL,
        MaxProcessorIndex: *mut UINTN,
    ) -> EFI_STATUS,
}
