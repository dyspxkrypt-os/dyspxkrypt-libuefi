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

use crate::protocols::device_path::EFI_DEVICE_PATH_PROTOCOL;
use crate::types::{EFI_EVENT, EFI_GUID, EFI_STATUS, UINT32, UINT64, UINT8, VOID};

pub const EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x52C78312,
        0x8EDC,
        0x4233,
        [0x98, 0xF2, 0x1A, 0x1A, 0xA5, 0xE3, 0x88, 0xA5],
    )
};

pub const EFI_NVM_EXPRESS_PASS_THRU_ATTRIBUTES_PHYSICAL: UINT32 = 0x0001;
pub const EFI_NVM_EXPRESS_PASS_THRU_ATTRIBUTES_LOGICAL: UINT32 = 0x0002;
pub const EFI_NVM_EXPRESS_PASS_THRU_ATTRIBUTES_NONBLOCKIO: UINT32 = 0x0004;
pub const EFI_NVM_EXPRESS_PASS_THRU_ATTRIBUTES_CMD_SET_NVM: UINT32 = 0x0008;

pub const NORMAL_CMD: UINT32 = 0x00;
pub const FUSED_FIRST_CMD: UINT32 = 0x01;
pub const FUSED_SECOND_CMD: UINT32 = 0x02;

pub const CDW2_VALID: UINT32 = 0x01;
pub const CDW3_VALID: UINT32 = 0x02;
pub const CDW10_VALID: UINT32 = 0x04;
pub const CDW11_VALID: UINT32 = 0x08;
pub const CDW12_VALID: UINT32 = 0x10;
pub const CDW13_VALID: UINT32 = 0x20;
pub const CDW14_VALID: UINT32 = 0x40;
pub const CDW15_VALID: UINT32 = 0x80;

#[repr(C)]
pub struct EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL {
    pub Mode: *mut EFI_NVM_EXPRESS_PASS_THRU_MODE,
    pub PassThru: unsafe extern "efiapi" fn(
        This: *mut EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL,
        NamespaceId: UINT32,
        Packet: *mut EFI_NVM_EXPRESS_PASS_THRU_COMMAND_PACKET,
        Event: EFI_EVENT,
    ) -> EFI_STATUS,
    pub GetNextNamespace: unsafe extern "efiapi" fn(
        This: *mut EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL,
        NamespaceId: *mut UINT32,
    ) -> EFI_STATUS,
    pub BuildDevicePath: unsafe extern "efiapi" fn(
        This: *mut EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL,
        NamespaceId: UINT32,
        DevicePath: *mut *mut EFI_DEVICE_PATH_PROTOCOL,
    ) -> EFI_STATUS,
    pub GetNamespace: unsafe extern "efiapi" fn(
        This: *mut EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL,
        DevicePath: *mut EFI_DEVICE_PATH_PROTOCOL,
        NamespaceId: *mut UINT32,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_NVM_EXPRESS_COMMAND {
    pub Cdw0: NVME_CDW0,
    pub Flags: UINT8,
    pub Nsid: UINT32,
    pub Cdw2: UINT32,
    pub Cdw3: UINT32,
    pub Cdw10: UINT32,
    pub Cdw11: UINT32,
    pub Cdw12: UINT32,
    pub Cdw13: UINT32,
    pub Cdw14: UINT32,
    pub Cdw15: UINT32,
}

#[repr(C)]
pub struct EFI_NVM_EXPRESS_COMPLETION {
    pub DW0: UINT32,
    pub DW1: UINT32,
    pub DW2: UINT32,
    pub DW3: UINT32,
}

#[repr(C)]
pub struct EFI_NVM_EXPRESS_PASS_THRU_COMMAND_PACKET {
    pub CommandTimeout: UINT64,
    pub TransferBuffer: *mut VOID,
    pub TransferLength: UINT32,
    pub MetaDataBuffer: *mut VOID,
    pub MetaDataLength: UINT32,
    pub QueueType: UINT8,
    pub NvmeCmd: *mut EFI_NVM_EXPRESS_COMMAND,
    pub NvmeCompletion: *mut EFI_NVM_EXPRESS_COMPLETION,
}

#[repr(C)]
pub struct EFI_NVM_EXPRESS_PASS_THRU_MODE {
    pub Attributes: UINT32,
    pub IoAlign: UINT32,
    pub NvmeVersion: UINT32,
}

// FIXME: bitfields
pub type NVME_CDW0 = UINT32;
