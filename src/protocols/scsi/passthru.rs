/*
 * Dyspxkrypt LibUEFI: Raw bindings of UEFI that conforms to the definitions of the UEFI Specification.
 * Copyright (C) 2023 HTGAzureX1212.
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

pub const EFI_EXT_SCSI_PASS_THRU_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x143B7632,
        0xB81B,
        0x4CB7,
        [0xAB, 0xD3, 0xB6, 0x25, 0xA5, 0xB9, 0xBF, 0xFE],
    )
};

pub const TARGET_MAX_BYTES: UINT8 = 0x10;
pub const EFI_EXT_SCSI_PASS_THRU_ATTRIBUTES_PHYSICAL: UINT8 = 0x0001;
pub const EFI_EXT_SCSI_PASS_THRU_ATTRIBUTES_LOGICAL: UINT8 = 0x0002;
pub const EFI_EXT_SCSI_PASS_THRU_ATTRIBUTES_NONBLOCKIO: UINT8 = 0x0004;

pub const EFI_EXT_SCSI_DATA_DIRECTION_READ: UINT8 = 0;
pub const EFI_EXT_SCSI_DATA_DIRECTION_WRITE: UINT8 = 1;
pub const EFI_EXT_SCSI_DATA_DIRECTION_BIDIRECTIONAL: UINT8 = 2;

pub const EFI_EXT_SCSI_STATUS_HOST_ADAPTER_OK: UINT8 = 0x00;
pub const EFI_EXT_SCSI_STATUS_HOST_ADAPTER_TIMEOUT_COMMAND: UINT8 = 0x09;
pub const EFI_EXT_SCSI_STATUS_HOST_ADAPTER_TIMEOUT: UINT8 = 0x0b;
pub const EFI_EXT_SCSI_STATUS_HOST_ADAPTER_MESSAGE_REJECT: UINT8 = 0x0d;
pub const EFI_EXT_SCSI_STATUS_HOST_ADAPTER_BUS_RESET: UINT8 = 0x0e;
pub const EFI_EXT_SCSI_STATUS_HOST_ADAPTER_PARITY_ERROR: UINT8 = 0x0f;
pub const EFI_EXT_SCSI_STATUS_HOST_ADAPTER_REQUEST_SENSE_FAILED: UINT8 = 0x10;
pub const EFI_EXT_SCSI_STATUS_HOST_ADAPTER_SELECTION_TIMEOUT: UINT8 = 0x11;
pub const EFI_EXT_SCSI_STATUS_HOST_ADAPTER_DATA_OVERRUN_UNDERRUN: UINT8 = 0x12;
pub const EFI_EXT_SCSI_STATUS_HOST_ADAPTER_BUS_FREE: UINT8 = 0x13;
pub const EFI_EXT_SCSI_STATUS_HOST_ADAPTER_PHASE_ERROR: UINT8 = 0x14;
pub const EFI_EXT_SCSI_STATUS_HOST_ADAPTER_OTHER: UINT8 = 0x7f;

pub const EFI_EXT_SCSI_STATUS_TARGET_GOOD: UINT8 = 0x00;
pub const EFI_EXT_SCSI_STATUS_TARGET_CHECK_CONDITION: UINT8 = 0x02;
pub const EFI_EXT_SCSI_STATUS_TARGET_CONDITION_MET: UINT8 = 0x04;
pub const EFI_EXT_SCSI_STATUS_TARGET_BUSY: UINT8 = 0x08;
pub const EFI_EXT_SCSI_STATUS_TARGET_INTERMEDIATE: UINT8 = 0x10;
pub const EFI_EXT_SCSI_STATUS_TARGET_INTERMEDIATE_CONDITION_MET: UINT8 = 0x14;
pub const EFI_EXT_SCSI_STATUS_TARGET_RESERVATION_CONFLICT: UINT8 = 0x18;
pub const EFI_EXT_SCSI_STATUS_TARGET_TASK_SET_FULL: UINT8 = 0x28;
pub const EFI_EXT_SCSI_STATUS_TARGET_ACA_ACTIVE: UINT8 = 0x30;
pub const EFI_EXT_SCSI_STATUS_TARGET_TASK_ABORTED: UINT8 = 0x40;

#[repr(C)]
pub struct EFI_EXT_SCSI_PASS_THRU_PROTOCOL {
    pub Mode: *mut EFI_EXT_SCSI_PASS_THRU_MODE,
    pub PassThru: unsafe extern "efiapi" fn(
        This: *mut EFI_EXT_SCSI_PASS_THRU_PROTOCOL,
        Target: *mut UINT8,
        Lun: UINT64,
        Packet: *mut EFI_EXT_SCSI_PASS_THRU_SCSI_REQUEST_PACKET,
        Event: EFI_EVENT,
    ) -> EFI_STATUS,
    pub GetNextTargetLun: unsafe extern "efiapi" fn(
        This: *mut EFI_EXT_SCSI_PASS_THRU_PROTOCOL,
        Target: *mut *mut UINT8,
        Lun: *mut UINT64,
    ) -> EFI_STATUS,
    pub BuildDevicePath: unsafe extern "efiapi" fn(
        This: *mut EFI_EXT_SCSI_PASS_THRU_PROTOCOL,
        Target: *mut UINT8,
        Lun: UINT64,
        DevicePath: *mut *mut EFI_DEVICE_PATH_PROTOCOL,
    ) -> EFI_STATUS,
    pub GetTargetLun: unsafe extern "efiapi" fn(
        This: *mut EFI_EXT_SCSI_PASS_THRU_PROTOCOL,
        DevicePath: *mut EFI_DEVICE_PATH_PROTOCOL,
        Target: *mut *mut UINT8,
        Lun: *mut UINT64,
    ) -> EFI_STATUS,
    pub ResetChannel:
        unsafe extern "efiapi" fn(This: *mut EFI_EXT_SCSI_PASS_THRU_PROTOCOL) -> EFI_STATUS,
    pub ResetTargetLun: unsafe extern "efiapi" fn(
        This: *mut EFI_EXT_SCSI_PASS_THRU_PROTOCOL,
        Target: *mut UINT8,
        Lun: UINT64,
    ) -> EFI_STATUS,
    pub GetNextTarget: unsafe extern "efiapi" fn(
        This: *mut EFI_EXT_SCSI_PASS_THRU_PROTOCOL,
        Target: *mut *mut UINT8,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_EXT_SCSI_PASS_THRU_MODE {
    pub AdapterId: UINT32,
    pub Attributes: UINT32,
    pub IoAlign: UINT32,
}

#[repr(C)]
pub struct EFI_EXT_SCSI_PASS_THRU_SCSI_REQUEST_PACKET {
    pub Timeout: UINT64,
    pub InDataBuffer: *mut VOID,
    pub OutDataBuffer: *mut VOID,
    pub SenseData: *mut VOID,
    pub Cdb: *mut VOID,
    pub InTransferLength: UINT32,
    pub OutTransferLength: UINT32,
    pub CdbLength: UINT8,
    pub DataDirection: UINT8,
    pub HostAdapterStatus: UINT8,
    pub TargetStatus: UINT8,
    pub SenseDataLength: UINT8,
}
