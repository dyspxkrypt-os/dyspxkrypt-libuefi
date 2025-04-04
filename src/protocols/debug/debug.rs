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

pub const EFI_DEBUG_SUPPORT_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x2755590C,
        0x6F3C,
        0x42FA,
        [0x9E, 0xA4, 0xA3, 0xBA, 0x54, 0x3C, 0xDA, 0x25],
    )
};

pub const EXCEPT_AARCH64_SYNCHRONOUS_EXCEPTIONS: INTN = 0;
pub const EXCEPT_AARCH64_IRQ: INTN = 1;
pub const EXCEPT_AARCH64_FIQ: INTN = 2;
pub const EXCEPT_AARCH64_SERROR: INTN = 3;

pub const EXCEPT_ARM_RESET: INTN = 0;
pub const EXCEPT_ARM_UNDEFINED_INSTRUCTION: INTN = 1;
pub const EXCEPT_ARM_SOFTWARE_INTERRUPT: INTN = 2;
pub const EXCEPT_ARM_PREFETCH_ABORT: INTN = 3;
pub const EXCEPT_ARM_DATA_ABORT: INTN = 4;
pub const EXCEPT_ARM_RESERVED: INTN = 5;
pub const EXCEPT_ARM_IRQ: INTN = 6;
pub const EXCEPT_ARM_FIQ: INTN = 7;

pub const EXCEPT_EBC_UNDEFINED: INTN = 0;
pub const EXCEPT_EBC_DIVIDE_ERROR: INTN = 1;
pub const EXCEPT_EBC_DEBUG: INTN = 2;
pub const EXCEPT_EBC_BREAKPOINT: INTN = 3;
pub const EXCEPT_EBC_OVERFLOW: INTN = 4;
pub const EXCEPT_EBC_INVALID_OPCODE: INTN = 5;
pub const EXCEPT_EBC_STACK_FAULT: INTN = 6;
pub const EXCEPT_EBC_ALIGNMENT_CHECK: INTN = 7;
pub const EXCEPT_EBC_INSTRUCTION_ENCODING: INTN = 8;
pub const EXCEPT_EBC_BAD_BREAK: INTN = 9;
pub const EXCEPT_EBC_SINGLE_STEP: INTN = 10;

pub const EXCEPT_IA32_DIVIDE_ERROR: INTN = 0;
pub const EXCEPT_IA32_DEBUG: INTN = 1;
pub const EXCEPT_IA32_NMI: INTN = 2;
pub const EXCEPT_IA32_BREAKPOINT: INTN = 3;
pub const EXCEPT_IA32_OVERFLOW: INTN = 4;
pub const EXCEPT_IA32_BOUND: INTN = 5;
pub const EXCEPT_IA32_INVALID_OPCODE: INTN = 6;
pub const EXCEPT_IA32_DOUBLE_FAULT: INTN = 8;
pub const EXCEPT_IA32_INVALID_TSS: INTN = 10;
pub const EXCEPT_IA32_SEG_NOT_PRESENT: INTN = 11;
pub const EXCEPT_IA32_STACK_FAULT: INTN = 12;
pub const EXCEPT_IA32_GP_FAULT: INTN = 13;
pub const EXCEPT_IA32_PAGE_FAULT: INTN = 14;
pub const EXCEPT_IA32_FP_ERROR: INTN = 16;
pub const EXCEPT_IA32_ALIGNMENT_CHECK: INTN = 17;
pub const EXCEPT_IA32_MACHINE_CHECK: INTN = 18;
pub const EXCEPT_IA32_SIMD: INTN = 19;

pub const EXCEPT_IPF_VHTP_TRANSLATION: INTN = 0;
pub const EXCEPT_IPF_INSTRUCTION_TLB: INTN = 1;
pub const EXCEPT_IPF_DATA_TLB: INTN = 2;
pub const EXCEPT_IPF_ALT_INSTRUCTION_TLB: INTN = 3;
pub const EXCEPT_IPF_ALT_DATA_TLB: INTN = 4;
pub const EXCEPT_IPF_DATA_NESTED_TLB: INTN = 5;
pub const EXCEPT_IPF_INSTRUCTION_KEY_MISSED: INTN = 6;
pub const EXCEPT_IPF_DATA_KEY_MISSED: INTN = 7;
pub const EXCEPT_IPF_DIRTY_BIT: INTN = 8;
pub const EXCEPT_IPF_INSTRUCTION_ACCESS_BIT: INTN = 9;
pub const EXCEPT_IPF_DATA_ACCESS_BIT: INTN = 10;
pub const EXCEPT_IPF_BREAKPOINT: INTN = 11;
pub const EXCEPT_IPF_EXTERNAL_INTERRUPT: INTN = 12;
pub const EXCEPT_IPF_PAGE_NOT_PRESENT: INTN = 20;
pub const EXCEPT_IPF_KEY_PERMISSION: INTN = 21;
pub const EXCEPT_IPF_INSTRUCTION_ACCESS_RIGHTS: INTN = 22;
pub const EXCEPT_IPF_DATA_ACCESS_RIGHTS: INTN = 23;
pub const EXCEPT_IPF_GENERAL_EXCEPTION: INTN = 24;
pub const EXCEPT_IPF_DISABLED_FP_REGISTER: INTN = 25;
pub const EXCEPT_IPF_NAT_CONSUMPTION: INTN = 26;
pub const EXCEPT_IPF_SPECULATION: INTN = 27;
pub const EXCEPT_IPF_DEBUG: INTN = 29;
pub const EXCEPT_IPF_UNALIGNED_REFERENCE: INTN = 30;
pub const EXCEPT_IPF_UNSUPPORTED_DATA_REFERENCE: INTN = 31;
pub const EXCEPT_IPF_FP_FAULT: INTN = 32;
pub const EXCEPT_IPF_FP_TRAP: INTN = 33;
pub const EXCEPT_IPF_LOWER_PRIVILEGE_TRANSFER_TRAP: INTN = 34;
pub const EXCEPT_IPF_TAKEN_BRANCH: INTN = 35;
pub const EXCEPT_IPF_SINGLE_STEP: INTN = 36;
pub const EXCEPT_IPF_IA32_EXCEPTION: INTN = 45;
pub const EXCEPT_IPF_IA32_INTERCEPT: INTN = 46;
pub const EXCEPT_IPF_IA32_INTERRUPT: INTN = 47;

pub const EXCEPT_LOONGARCH_INT: INTN = 0;
pub const EXCEPT_LOONGARCH_PIL: INTN = 1;
pub const EXCEPT_LOONGARCH_PIS: INTN = 2;
pub const EXCEPT_LOONGARCH_PIF: INTN = 3;
pub const EXCEPT_LOONGARCH_PME: INTN = 4;
pub const EXCEPT_LOONGARCH_PNR: INTN = 5;
pub const EXCEPT_LOONGARCH_PNX: INTN = 6;
pub const EXCEPT_LOONGARCH_PPI: INTN = 7;
pub const EXCEPT_LOONGARCH_ADE: INTN = 8;
pub const EXCEPT_LOONGARCH_ALE: INTN = 9;
pub const EXCEPT_LOONGARCH_BCE: INTN = 10;
pub const EXCEPT_LOONGARCH_SYS: INTN = 11;
pub const EXCEPT_LOONGARCH_BRK: INTN = 12;
pub const EXCEPT_LOONGARCH_INE: INTN = 13;
pub const EXCEPT_LOONGARCH_IPE: INTN = 14;
pub const EXCEPT_LOONGARCH_FPD: INTN = 15;
pub const EXCEPT_LOONGARCH_SXD: INTN = 16;
pub const EXCEPT_LOONGARCH_ASXD: INTN = 17;
pub const EXCEPT_LOONGARCH_FPE: INTN = 18;

pub const EXCEPT_LOONGARCH_INT_SIP0: INTN = 0;
pub const EXCEPT_LOONGARCH_INT_SIP1: INTN = 1;
pub const EXCEPT_LOONGARCH_INT_IP0: INTN = 2;
pub const EXCEPT_LOONGARCH_INT_IP1: INTN = 3;
pub const EXCEPT_LOONGARCH_INT_IP2: INTN = 4;
pub const EXCEPT_LOONGARCH_INT_IP3: INTN = 5;
pub const EXCEPT_LOONGARCH_INT_IP4: INTN = 6;
pub const EXCEPT_LOONGARCH_INT_IP5: INTN = 7;
pub const EXCEPT_LOONGARCH_INT_IP6: INTN = 8;
pub const EXCEPT_LOONGARCH_INT_IP7: INTN = 9;
pub const EXCEPT_LOONGARCH_INT_PMC: INTN = 10;
pub const EXCEPT_LOONGARCH_INT_TIMER: INTN = 11;
pub const EXCEPT_LOONGARCH_INT_IPI: INTN = 12;

pub const EXCEPT_RISCV_INST_MISALIGNED: INTN = 0;
pub const EXCEPT_RISCV_INST_ACCESS_FAULT: INTN = 1;
pub const EXCEPT_RISCV_ILLEGAL_INST: INTN = 2;
pub const EXCEPT_RISCV_BREAKPOINT: INTN = 3;
pub const EXCEPT_RISCV_LOAD_ADDRESS_MISALIGNED: INTN = 4;
pub const EXCEPT_RISCV_LOAD_ACCESS_FAULT: INTN = 5;
pub const EXCEPT_RISCV_STORE_AMO_ADDRESS_MISALIGNED: INTN = 6;
pub const EXCEPT_RISCV_STORE_AMO_ACCESS_FAULT: INTN = 7;
pub const EXCEPT_RISCV_ENV_CALL_FROM_UMODE: INTN = 8;
pub const EXCEPT_RISCV_ENV_CALL_FROM_SMODE: INTN = 9;
pub const EXCEPT_RISCV_ENV_CALL_FROM_MMODE: INTN = 11;
pub const EXCEPT_RISCV_INST_PAGE_FAULT: INTN = 12;
pub const EXCEPT_RISCV_LOAD_PAGE_FAULT: INTN = 13;
pub const EXCEPT_RISCV_STORE_AMO_PAGE_FAULT: INTN = 15;

pub const EXCEPT_RISCV_SUPERVISOR_SOFTWARE_INT: INTN = 1;
pub const EXCEPT_RISCV_MACHINE_SOFTWARE_INT: INTN = 3;
pub const EXCEPT_RISCV_SUPERVISOR_TIMER_INT: INTN = 5;
pub const EXCEPT_RISCV_MACHINE_TIMER_INT: INTN = 7;
pub const EXCEPT_RISCV_SUPERVISOR_EXTERNAL_INT: INTN = 9;
pub const EXCEPT_RISCV_MACHINE_EXTERNAL_INT: INTN = 11;

pub const EXCEPT_X64_DIVIDE_ERROR: INTN = 0;
pub const EXCEPT_X64_DEBUG: INTN = 1;
pub const EXCEPT_X64_NMI: INTN = 2;
pub const EXCEPT_X64_BREAKPOINT: INTN = 3;
pub const EXCEPT_X64_OVERFLOW: INTN = 4;
pub const EXCEPT_X64_BOUND: INTN = 5;
pub const EXCEPT_X64_INVALID_OPCODE: INTN = 6;
pub const EXCEPT_X64_DOUBLE_FAULT: INTN = 8;
pub const EXCEPT_X64_INVALID_TSS: INTN = 10;
pub const EXCEPT_X64_SEG_NOT_PRESENT: INTN = 11;
pub const EXCEPT_X64_STACK_FAULT: INTN = 12;
pub const EXCEPT_X64_GP_FAULT: INTN = 13;
pub const EXCEPT_X64_PAGE_FAULT: INTN = 14;
pub const EXCEPT_X64_FP_ERROR: INTN = 16;
pub const EXCEPT_X64_ALIGNMENT_CHECK: INTN = 17;
pub const EXCEPT_X64_MACHINE_CHECK: INTN = 18;
pub const EXCEPT_X64_SIMD: INTN = 19;

pub const MAX_LOONGARCH_INTERRUPT: INTN = 14;

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
    pub RegisterExceptionCallback: unsafe extern "efiapi" fn(
        This: *mut EFI_DEBUG_SUPPORT_PROTOCOL,
        ProcessorIndex: UINTN,
        ExceptionCallback: EFI_EXCEPTION_CALLBACK,
        ExceptionType: EFI_EXCEPTION_TYPE,
    ) -> EFI_STATUS,
    pub InvalidateInstructionCache: unsafe extern "efiapi" fn(
        This: *mut EFI_DEBUG_SUPPORT_PROTOCOL,
        ProcessorIndex: UINTN,
        Start: *mut VOID,
        Length: UINT64,
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
pub struct EFI_FX_SAVE_STATE_X64 {
    pub Fcw: UINT16,
    pub Fsw: UINT16,
    pub Ftw: UINT16,
    pub Opcode: UINT16,
    pub Rip: UINT64,
    pub DataOffset: UINT64,
    Reserved1: [UINT8; 8],
    pub St0Mm0: [UINT8; 10],
    Reserved2: [UINT8; 6],
    pub St0Mm1: [UINT8; 10],
    Reserved3: [UINT8; 6],
    pub St0Mm2: [UINT8; 10],
    Reserved4: [UINT8; 6],
    pub St0Mm3: [UINT8; 10],
    Reserved5: [UINT8; 6],
    pub St0Mm4: [UINT8; 10],
    Reserved6: [UINT8; 6],
    pub St0Mm5: [UINT8; 10],
    Reserved7: [UINT8; 6],
    pub St0Mm6: [UINT8; 10],
    Reserved8: [UINT8; 6],
    pub St0Mm7: [UINT8; 10],
    Reserved9: [UINT8; 6],
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
pub struct EFI_SYSTEM_CONTEXT_AARCH64 {
    pub X0: UINT64,
    pub X1: UINT64,
    pub X2: UINT64,
    pub X3: UINT64,
    pub X4: UINT64,
    pub X5: UINT64,
    pub X6: UINT64,
    pub X7: UINT64,
    pub X8: UINT64,
    pub X9: UINT64,
    pub X10: UINT64,
    pub X11: UINT64,
    pub X12: UINT64,
    pub X13: UINT64,
    pub X14: UINT64,
    pub X15: UINT64,
    pub X16: UINT64,
    pub X17: UINT64,
    pub X18: UINT64,
    pub X19: UINT64,
    pub X20: UINT64,
    pub X21: UINT64,
    pub X22: UINT64,
    pub X23: UINT64,
    pub X24: UINT64,
    pub X25: UINT64,
    pub X26: UINT64,
    pub X27: UINT64,
    pub X28: UINT64,
    pub FP: UINT64,
    pub LR: UINT64,
    pub SP: UINT64,
    pub V0: [UINT64; 2],
    pub V1: [UINT64; 2],
    pub V2: [UINT64; 2],
    pub V3: [UINT64; 2],
    pub V4: [UINT64; 2],
    pub V5: [UINT64; 2],
    pub V6: [UINT64; 2],
    pub V7: [UINT64; 2],
    pub V8: [UINT64; 2],
    pub V9: [UINT64; 2],
    pub V10: [UINT64; 2],
    pub V11: [UINT64; 2],
    pub V12: [UINT64; 2],
    pub V13: [UINT64; 2],
    pub V14: [UINT64; 2],
    pub V15: [UINT64; 2],
    pub V16: [UINT64; 2],
    pub V17: [UINT64; 2],
    pub V18: [UINT64; 2],
    pub V19: [UINT64; 2],
    pub V20: [UINT64; 2],
    pub V21: [UINT64; 2],
    pub V22: [UINT64; 2],
    pub V23: [UINT64; 2],
    pub V24: [UINT64; 2],
    pub V25: [UINT64; 2],
    pub V26: [UINT64; 2],
    pub V27: [UINT64; 2],
    pub V28: [UINT64; 2],
    pub V29: [UINT64; 2],
    pub V30: [UINT64; 2],
    pub V31: [UINT64; 2],
    pub ELR: UINT64,
    pub SPSR: UINT64,
    pub FPSR: UINT64,
    pub ESR: UINT64,
    pub FAR: UINT64,
}

#[repr(C)]
pub struct EFI_SYSTEM_CONTEXT_ARM {
    pub R0: UINT32,
    pub R1: UINT32,
    pub R2: UINT32,
    pub R3: UINT32,
    pub R4: UINT32,
    pub R5: UINT32,
    pub R6: UINT32,
    pub R7: UINT32,
    pub R8: UINT32,
    pub R9: UINT32,
    pub R10: UINT32,
    pub R11: UINT32,
    pub R12: UINT32,
    pub SP: UINT32,
    pub LR: UINT32,
    pub PC: UINT32,
    pub CPSR: UINT32,
    pub DFSR: UINT32,
    pub DFAR: UINT32,
    pub IFSR: UINT32,
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

#[repr(C)]
pub struct EFI_SYSTEM_CONTEXT_IPF {
    Reserved: UINT64,
    pub R1: UINT64,
    pub R2: UINT64,
    pub R3: UINT64,
    pub R4: UINT64,
    pub R5: UINT64,
    pub R6: UINT64,
    pub R7: UINT64,
    pub R8: UINT64,
    pub R9: UINT64,
    pub R10: UINT64,
    pub R11: UINT64,
    pub R12: UINT64,
    pub R13: UINT64,
    pub R14: UINT64,
    pub R15: UINT64,
    pub R16: UINT64,
    pub R17: UINT64,
    pub R18: UINT64,
    pub R19: UINT64,
    pub R20: UINT64,
    pub R21: UINT64,
    pub R22: UINT64,
    pub R23: UINT64,
    pub R24: UINT64,
    pub R25: UINT64,
    pub R26: UINT64,
    pub R27: UINT64,
    pub R28: UINT64,
    pub R29: UINT64,
    pub R30: UINT64,
    pub F2: [UINT64; 2],
    pub F3: [UINT64; 2],
    pub F4: [UINT64; 2],
    pub F5: [UINT64; 2],
    pub F6: [UINT64; 2],
    pub F7: [UINT64; 2],
    pub F8: [UINT64; 2],
    pub F9: [UINT64; 2],
    pub F10: [UINT64; 2],
    pub F11: [UINT64; 2],
    pub F12: [UINT64; 2],
    pub F13: [UINT64; 2],
    pub F14: [UINT64; 2],
    pub F15: [UINT64; 2],
    pub F16: [UINT64; 2],
    pub F17: [UINT64; 2],
    pub F18: [UINT64; 2],
    pub F19: [UINT64; 2],
    pub F20: [UINT64; 2],
    pub F21: [UINT64; 2],
    pub F22: [UINT64; 2],
    pub F23: [UINT64; 2],
    pub F24: [UINT64; 2],
    pub F25: [UINT64; 2],
    pub F26: [UINT64; 2],
    pub F27: [UINT64; 2],
    pub F28: [UINT64; 2],
    pub F29: [UINT64; 2],
    pub F30: [UINT64; 2],
    pub F31: [UINT64; 2],
    pub Pr: UINT64,
    pub B0: UINT64,
    pub B1: UINT64,
    pub B2: UINT64,
    pub B3: UINT64,
    pub B4: UINT64,
    pub B5: UINT64,
    pub B6: UINT64,
    pub B7: UINT64,
    pub ArRsc: UINT64,
    pub ArBsp: UINT64,
    pub ArBspstore: UINT64,
    pub ArRnat: UINT64,
    pub ArFcr: UINT64,
    pub ArEflag: UINT64,
    pub ArCsd: UINT64,
    pub ArSsd: UINT64,
    pub ArCflg: UINT64,
    pub ArFsr: UINT64,
    pub ArFir: UINT64,
    pub ArFdr: UINT64,
    pub ArCcv: UINT64,
    pub ArUnat: UINT64,
    pub ArFpsr: UINT64,
    pub ArPfs: UINT64,
    pub ArLc: UINT64,
    pub ArEc: UINT64,
    pub CrDcr: UINT64,
    pub CrItm: UINT64,
    pub CrIva: UINT64,
    pub CrPta: UINT64,
    pub CrIpsr: UINT64,
    pub CrIsr: UINT64,
    pub CrIip: UINT64,
    pub CrIfa: UINT64,
    pub CrItir: UINT64,
    pub CrIipa: UINT64,
    pub CrIfs: UINT64,
    pub CrIim: UINT64,
    pub CrIha: UINT64,
    pub Dbr0: UINT64,
    pub Dbr1: UINT64,
    pub Dbr2: UINT64,
    pub Dbr3: UINT64,
    pub Dbr4: UINT64,
    pub Dbr5: UINT64,
    pub Dbr6: UINT64,
    pub Dbr7: UINT64,
    pub Ibr0: UINT64,
    pub Ibr1: UINT64,
    pub Ibr2: UINT64,
    pub Ibr3: UINT64,
    pub Ibr4: UINT64,
    pub Ibr5: UINT64,
    pub Ibr6: UINT64,
    pub Ibr7: UINT64,
    pub IntNat: UINT64,
}

#[repr(C)]
pub struct EFI_SYSTEM_CONTEXT_LOONGARCH64 {
    pub R0: UINT64,
    pub R1: UINT64,
    pub R2: UINT64,
    pub R3: UINT64,
    pub R4: UINT64,
    pub R5: UINT64,
    pub R6: UINT64,
    pub R7: UINT64,
    pub R8: UINT64,
    pub R9: UINT64,
    pub R10: UINT64,
    pub R11: UINT64,
    pub R12: UINT64,
    pub R13: UINT64,
    pub R14: UINT64,
    pub R15: UINT64,
    pub R16: UINT64,
    pub R17: UINT64,
    pub R18: UINT64,
    pub R19: UINT64,
    pub R20: UINT64,
    pub R21: UINT64,
    pub R22: UINT64,
    pub R23: UINT64,
    pub R24: UINT64,
    pub R25: UINT64,
    pub R26: UINT64,
    pub R27: UINT64,
    pub R28: UINT64,
    pub R29: UINT64,
    pub R30: UINT64,
    pub R31: UINT64,
    pub CRMD: UINT64,
    pub PRMD: UINT64,
    pub EUEN: UINT64,
    pub MISC: UINT64,
    pub ECFG: UINT64,
    pub ESTAT: UINT64,
    pub ERA: UINT64,
    pub BADV: UINT64,
    pub BADI: UINT64,
}

#[repr(C)]
pub struct EFI_SYSTEM_CONTEXT_RISCV32 {
    pub Zero: UINT32,
    pub Ra: UINT32,
    pub Sp: UINT32,
    pub Gp: UINT32,
    pub Tp: UINT32,
    pub T0: UINT32,
    pub T1: UINT32,
    pub T2: UINT32,
    pub S0FP: UINT32,
    pub S1: UINT32,
    pub A0: UINT32,
    pub A1: UINT32,
    pub A2: UINT32,
    pub A3: UINT32,
    pub A4: UINT32,
    pub A5: UINT32,
    pub A6: UINT32,
    pub A7: UINT32,
    pub S2: UINT32,
    pub S3: UINT32,
    pub S4: UINT32,
    pub S5: UINT32,
    pub S6: UINT32,
    pub S7: UINT32,
    pub S8: UINT32,
    pub S9: UINT32,
    pub S10: UINT32,
    pub S11: UINT32,
    pub T3: UINT32,
    pub T4: UINT32,
    pub T5: UINT32,
    pub T6: UINT32,
    pub Ft0: UINT128,
    pub Ft1: UINT128,
    pub Ft2: UINT128,
    pub Ft3: UINT128,
    pub Ft4: UINT128,
    pub Ft5: UINT128,
    pub Ft6: UINT128,
    pub Ft7: UINT128,
    pub Fs0: UINT128,
    pub Fs1: UINT128,
    pub Fa0: UINT128,
    pub Fa1: UINT128,
    pub Fa2: UINT128,
    pub Fa3: UINT128,
    pub Fa4: UINT128,
    pub Fa5: UINT128,
    pub Fa6: UINT128,
    pub Fa7: UINT128,
    pub Fs2: UINT128,
    pub Fs3: UINT128,
    pub Fs4: UINT128,
    pub Fs5: UINT128,
    pub Fs6: UINT128,
    pub Fs7: UINT128,
    pub Fs8: UINT128,
    pub Fs9: UINT128,
    pub Fs10: UINT128,
    pub Fs11: UINT128,
    pub Ft8: UINT128,
    pub Ft9: UINT128,
    pub Ft10: UINT128,
    pub Ft11: UINT128,
}

#[repr(C)]
pub struct EFI_SYSTEM_CONTEXT_RISCV64 {
    pub Zero: UINT64,
    pub Ra: UINT64,
    pub Sp: UINT64,
    pub Gp: UINT64,
    pub Tp: UINT64,
    pub T0: UINT64,
    pub T1: UINT64,
    pub T2: UINT64,
    pub S0FP: UINT64,
    pub S1: UINT64,
    pub A0: UINT64,
    pub A1: UINT64,
    pub A2: UINT64,
    pub A3: UINT64,
    pub A4: UINT64,
    pub A5: UINT64,
    pub A6: UINT64,
    pub A7: UINT64,
    pub S2: UINT64,
    pub S3: UINT64,
    pub S4: UINT64,
    pub S5: UINT64,
    pub S6: UINT64,
    pub S7: UINT64,
    pub S8: UINT64,
    pub S9: UINT64,
    pub S10: UINT64,
    pub S11: UINT64,
    pub T3: UINT64,
    pub T4: UINT64,
    pub T5: UINT64,
    pub T6: UINT64,
    pub Ft0: UINT128,
    pub Ft1: UINT128,
    pub Ft2: UINT128,
    pub Ft3: UINT128,
    pub Ft4: UINT128,
    pub Ft5: UINT128,
    pub Ft6: UINT128,
    pub Ft7: UINT128,
    pub Fs0: UINT128,
    pub Fs1: UINT128,
    pub Fa0: UINT128,
    pub Fa1: UINT128,
    pub Fa2: UINT128,
    pub Fa3: UINT128,
    pub Fa4: UINT128,
    pub Fa5: UINT128,
    pub Fa6: UINT128,
    pub Fa7: UINT128,
    pub Fs2: UINT128,
    pub Fs3: UINT128,
    pub Fs4: UINT128,
    pub Fs5: UINT128,
    pub Fs6: UINT128,
    pub Fs7: UINT128,
    pub Fs8: UINT128,
    pub Fs9: UINT128,
    pub Fs10: UINT128,
    pub Fs11: UINT128,
    pub Ft8: UINT128,
    pub Ft9: UINT128,
    pub Ft10: UINT128,
    pub Ft11: UINT128,
}

#[repr(C)]
pub struct EFI_SYSTEM_CONTEXT_RISCV128 {
    pub Zero: UINT128,
    pub Ra: UINT128,
    pub Sp: UINT128,
    pub Gp: UINT128,
    pub Tp: UINT128,
    pub T0: UINT128,
    pub T1: UINT128,
    pub T2: UINT128,
    pub S0FP: UINT128,
    pub S1: UINT128,
    pub A0: UINT128,
    pub A1: UINT128,
    pub A2: UINT128,
    pub A3: UINT128,
    pub A4: UINT128,
    pub A5: UINT128,
    pub A6: UINT128,
    pub A7: UINT128,
    pub S2: UINT128,
    pub S3: UINT128,
    pub S4: UINT128,
    pub S5: UINT128,
    pub S6: UINT128,
    pub S7: UINT128,
    pub S8: UINT128,
    pub S9: UINT128,
    pub S10: UINT128,
    pub S11: UINT128,
    pub T3: UINT128,
    pub T4: UINT128,
    pub T5: UINT128,
    pub T6: UINT128,
    pub Ft0: UINT128,
    pub Ft1: UINT128,
    pub Ft2: UINT128,
    pub Ft3: UINT128,
    pub Ft4: UINT128,
    pub Ft5: UINT128,
    pub Ft6: UINT128,
    pub Ft7: UINT128,
    pub Fs0: UINT128,
    pub Fs1: UINT128,
    pub Fa0: UINT128,
    pub Fa1: UINT128,
    pub Fa2: UINT128,
    pub Fa3: UINT128,
    pub Fa4: UINT128,
    pub Fa5: UINT128,
    pub Fa6: UINT128,
    pub Fa7: UINT128,
    pub Fs2: UINT128,
    pub Fs3: UINT128,
    pub Fs4: UINT128,
    pub Fs5: UINT128,
    pub Fs6: UINT128,
    pub Fs7: UINT128,
    pub Fs8: UINT128,
    pub Fs9: UINT128,
    pub Fs10: UINT128,
    pub Fs11: UINT128,
    pub Ft8: UINT128,
    pub Ft9: UINT128,
    pub Ft10: UINT128,
    pub Ft11: UINT128,
}

#[repr(C)]
pub struct EFI_SYSTEM_CONTEXT_X64 {
    pub ExceptionData: UINT64,
    pub FxSaveState: EFI_FX_SAVE_STATE_X64,
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
    pub Cr8: UINT32,
    pub Rflags: UINT32,
    pub Ldtr: UINT32,
    pub Tr: UINT32,
    pub Gdtr: [UINT32; 2],
    pub Idtr: [UINT32; 2],
    pub Rip: UINT32,
    pub Gs: UINT32,
    pub Fs: UINT32,
    pub Es: UINT32,
    pub Ds: UINT32,
    pub Cs: UINT32,
    pub Ss: UINT32,
    pub Rdi: UINT32,
    pub Rsi: UINT32,
    pub Rbp: UINT32,
    pub Rsp: UINT32,
    pub Rbx: UINT32,
    pub Rdx: UINT32,
    pub Rcx: UINT32,
    pub Rax: UINT32,
    pub R8: UINT32,
    pub R9: UINT32,
    pub R10: UINT32,
    pub R11: UINT32,
    pub R12: UINT32,
    pub R13: UINT32,
    pub R14: UINT32,
    pub R15: UINT32,
}

pub type EFI_EXCEPTION_CALLBACK =
    unsafe extern "efiapi" fn(ExceptionType: EFI_EXCEPTION_TYPE, SystemContext: EFI_SYSTEM_CONTEXT);

pub type EFI_EXCEPTION_TYPE = INTN;

pub type EFI_PERIODIC_CALLBACK = unsafe extern "efiapi" fn(SystemContext: EFI_SYSTEM_CONTEXT);

#[repr(C)]
pub union EFI_SYSTEM_CONTEXT {
    pub SystemContextEbc: *mut EFI_SYSTEM_CONTEXT_EBC,
    pub SystemContextIa32: *mut EFI_SYSTEM_CONTEXT_IA32,
    pub SystemContextX64: *mut EFI_SYSTEM_CONTEXT_X64,
    pub SystemContextIpf: *mut EFI_SYSTEM_CONTEXT_IPF,
    pub SystemContextArm: *mut EFI_SYSTEM_CONTEXT_ARM,
    pub SystemContextRiscV32: *mut EFI_SYSTEM_CONTEXT_RISCV32,
    pub SystemContextRiscV64: *mut EFI_SYSTEM_CONTEXT_RISCV64,
    pub SystemContextRiscV128: *mut EFI_SYSTEM_CONTEXT_RISCV128,
    pub SystemContextLongArch64: *mut EFI_SYSTEM_CONTEXT_LOONGARCH64,
}
