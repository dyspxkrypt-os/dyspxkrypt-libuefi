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

pub const EFI_SCSI_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x932F47E6,
        0x2362,
        0x4002,
        [0x80, 0x3E, 0x3c, 0xD5, 0x4B, 0x13, 0x8F, 0x85],
    )
};

pub const EFI_SCSI_IO_TYPE_DISK: UINT8 = 0x00;
pub const EFI_SCSI_IO_TYPE_TAPE: UINT8 = 0x01;
pub const EFI_SCSI_IO_TYPE_PRINTER: UINT8 = 0x02;
pub const EFI_SCSI_IO_TYPE_PROCESSOR: UINT8 = 0x03;
pub const EFI_SCSI_IO_TYPE_WORM: UINT8 = 0x04;
pub const EFI_SCSI_IO_TYPE_CDROM: UINT8 = 0x05;
pub const EFI_SCSI_IO_TYPE_SCANNER: UINT8 = 0x06;
pub const EFI_SCSI_IO_TYPE_OPTICAL: UINT8 = 0x07;
pub const EFI_SCSI_IO_TYPE_MEDIUMCHANGER: UINT8 = 0x08;
pub const EFI_SCSI_IO_TYPE_COMMUNICATION: UINT8 = 0x09;
pub const MFI_SCSI_IO_TYPE_A: UINT8 = 0x0A;
pub const MFI_SCSI_IO_TYPE_B: UINT8 = 0x0B;
pub const MFI_SCSI_IO_TYPE_RAID: UINT8 = 0x0C;
pub const MFI_SCSI_IO_TYPE_SES: UINT8 = 0x0D;
pub const MFI_SCSI_IO_TYPE_RBC: UINT8 = 0x0E;
pub const MFI_SCSI_IO_TYPE_OCRW: UINT8 = 0x0F;
pub const MFI_SCSI_IO_TYPE_BRIDGE: UINT8 = 0x10;
pub const MFI_SCSI_IO_TYPE_OSD: UINT8 = 0x11;
pub const EFI_SCSI_IO_TYPE_RESERVED_LOW: UINT8 = 0x12;
pub const EFI_SCSI_IO_TYPE_RESERVED_HIGH: UINT8 = 0x1E;
pub const EFI_SCSI_IO_TYPE_UNKNOWN: UINT8 = 0x1F;

pub const EFI_SCSI_IO_DATA_DIRECTION_READ: UINT8 = 0;
pub const EFI_SCSI_IO_DATA_DIRECTION_WRITE: UINT8 = 1;
pub const EFI_SCSI_IO_DATA_DIRECTION_BIDIRECTIONAL: UINT8 = 2;

pub const EFI_SCSI_IO_STATUS_HOST_ADAPTER_OK: UINT8 = 0x00;
pub const EFI_SCSI_IO_STATUS_HOST_ADAPTER_TIMEOUT_COMMAND: UINT8 = 0x09;
pub const EFI_SCSI_IO_STATUS_HOST_ADAPTER_TIMEOUT: UINT8 = 0x0b;
pub const EFI_SCSI_IO_STATUS_HOST_ADAPTER_MESSAGE_REJECT: UINT8 = 0x0d;
pub const EFI_SCSI_IO_STATUS_HOST_ADAPTER_BUS_RESET: UINT8 = 0x0e;
pub const EFI_SCSI_IO_STATUS_HOST_ADAPTER_PARITY_ERROR: UINT8 = 0x0f;
pub const EFI_SCSI_IO_STATUS_HOST_ADAPTER_REQUEST_SENSE_FAILED: UINT8 = 0x10;
pub const EFI_SCSI_IO_STATUS_HOST_ADAPTER_SELECTION_TIMEOUT: UINT8 = 0x11;
pub const EFI_SCSI_IO_STATUS_HOST_ADAPTER_DATA_OVERRUN_UNDERRUN: UINT8 = 0x12;
pub const EFI_SCSI_IO_STATUS_HOST_ADAPTER_BUS_FREE: UINT8 = 0x13;
pub const EFI_SCSI_IO_STATUS_HOST_ADAPTER_PHASE_ERROR: UINT8 = 0x14;
pub const EFI_SCSI_IO_STATUS_HOST_ADAPTER_OTHER: UINT8 = 0x7f;

pub const EFI_SCSI_IO_STATUS_TARGET_GOOD: UINT8 = 0x00;
pub const EFI_SCSI_IO_STATUS_TARGET_CHECK_CONDITION: UINT8 = 0x02;
pub const EFI_SCSI_IO_STATUS_TARGET_CONDITION_MET: UINT8 = 0x04;
pub const EFI_SCSI_IO_STATUS_TARGET_BUSY: UINT8 = 0x08;
pub const EFI_SCSI_IO_STATUS_TARGET_INTERMEDIATE: UINT8 = 0x10;
pub const EFI_SCSI_IO_STATUS_TARGET_INTERMEDIATE_CONDITION_METn: UINT8 = 0x14;
pub const EFI_SCSI_IO_STATUS_TARGET_RESERVATION_CONFLICT: UINT8 = 0x18;
pub const EFI_SCSI_IO_STATUS_TARGET_COMMAND_TERMINATED: UINT8 = 0x22;
pub const EFI_SCSI_IO_STATUS_TARGET_QUEUE_FULL: UINT8 = 0x28;

#[repr(C)]
pub struct EFI_SCSI_IO_PROTOCOL {
    pub GetDeviceType: unsafe extern "efiapi" fn(
        This: *mut EFI_SCSI_IO_PROTOCOL,
        DeviceType: *mut UINT8,
    ) -> EFI_STATUS,
    pub GetDeviceLocation: unsafe extern "efiapi" fn(
        This: *mut EFI_SCSI_IO_PROTOCOL,
        Target: *mut *mut UINT8,
        Lun: *mut UINT64,
    ) -> EFI_STATUS,
    pub ResetBus: unsafe extern "efiapi" fn(This: *mut EFI_SCSI_IO_PROTOCOL) -> EFI_STATUS,
    pub ResetDevice: unsafe extern "efiapi" fn(This: *mut EFI_SCSI_IO_PROTOCOL) -> EFI_STATUS,
    pub ExecuteScsiCommand: unsafe extern "efiapi" fn(
        This: *mut EFI_SCSI_IO_PROTOCOL,
        Packet: *mut EFI_SCSI_IO_SCSI_REQUEST_PACKET,
        Event: EFI_EVENT,
    ) -> EFI_STATUS,
    pub IoAlign: UINT32,
}

#[repr(C)]
pub struct EFI_SCSI_IO_SCSI_REQUEST_PACKET {
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
