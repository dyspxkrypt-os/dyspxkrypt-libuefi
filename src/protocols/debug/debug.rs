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
    pub RegisterPeriodicCallback: unsafe extern "efiapi" fn(
        This: *mut EFI_DEBUG_SUPPORT_PROTOCOL,
        ProcessorIndex: UINTN,
        PeriodicCallback: EFI_PERIODIC_CALLBACK,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_FX_SAVE_STATE_IA32 {
    pub Fcw: UINT16,
    pub Fsw: UINT16,
    pub Ftw: UINT16,
    pub Opcode: UINT16,
    pub Eip: UINT32,
    pub Cs: UINT16,
    Reserved1: UINT16,
    pub DataOffset: UINT32,
    pub Ds: UINT16,
    Reserved2: [UINT8; 10],
    pub St0Mm0: [UINT8; 10],
    Reserved3: [UINT8; 6],
    pub St0Mm1: [UINT8; 10],
    Reserved4: [UINT8; 6],
    pub St0Mm2: [UINT8; 10],
    Reserved5: [UINT8; 6],
    pub St0Mm3: [UINT8; 10],
    Reserved6: [UINT8; 6],
    pub St0Mm4: [UINT8; 10],
    Reserved7: [UINT8; 6],
    pub St0Mm5: [UINT8; 10],
    Reserved8: [UINT8; 6],
    pub St0Mm6: [UINT8; 10],
    Reserved9: [UINT8; 6],
    pub St0Mm7: [UINT8; 10],
    Reserved10: [UINT8; 6],
    pub Xmm0: [UINT8; 16],
    pub Xmm1: [UINT8; 16],
    pub Xmm2: [UINT8; 16],
    pub Xmm3: [UINT8; 16],
    pub Xmm4: [UINT8; 16],
    pub Xmm5: [UINT8; 16],
    pub Xmm6: [UINT8; 16],
    pub Xmm7: [UINT8; 16],
    Reserved11: [UINT8; 224],
}

#[repr(C)]
pub struct EFI_SYSTEM_CONTEXT_EBC {
    pub R0: UINT64,
    pub R1: UINT64,
    pub R2: UINT64,
    pub R3: UINT64,
    pub R4: UINT64,
    pub R5: UINT64,
    pub R6: UINT64,
    pub R7: UINT64,
    pub Flags: UINT64,
    pub ControlFlags: UINT64,
    pub Ip: UINT64,
}

#[repr(C)]
pub struct EFI_SYSTEM_CONTEXT_IA32 {
    pub ExceptionData: UINT32,
    pub FxSaveState: EFI_FX_SAVE_STATE_IA32,
    pub Dr0: UINT32,
    pub Dr1: UINT32,
    pub Dr2: UINT32,
    pub Dr3: UINT32,
    pub Dr6: UINT32,
    pub Dr7: UINT32,
    pub Cr0: UINT32,
    pub Cr1: UINT32,
    Reserved: UINT32,
    pub Cr2: UINT32,
    pub Cr3: UINT32,
    pub Cr4: UINT32,
    pub Eflags: UINT32,
    pub Ldtr: UINT32,
    pub Tr: UINT32,
    pub Gdtr: [UINT32; 2],
    pub Idtr: [UINT32; 2],
    pub Eip: UINT32,
    pub Gs: UINT32,
    pub Fs: UINT32,
    pub Es: UINT32,
    pub Ds: UINT32,
    pub Cs: UINT32,
    pub Ss: UINT32,
    pub Edi: UINT32,
    pub Esi: UINT32,
    pub Ebp: UINT32,
    pub Esp: UINT32,
    pub Ebx: UINT32,
    pub Edx: UINT32,
    pub Ecx: UINT32,
    pub Eax: UINT32,
}

pub type EFI_PERIODIC_CALLBACK =
    unsafe extern "efiapi" fn(SystemContext: EFI_SYSTEM_CONTEXT) -> EFI_STATUS;

#[repr(C)]
pub union EFI_SYSTEM_CONTEXT {
    pub SystemContextEbc: *mut EFI_SYSTEM_CONTEXT_EBC,
    pub SystemContextIa32: *mut EFI_SYSTEM_CONTEXT_IA32,
}
