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

use crate::protocols::device_path::EFI_DEVICE_PATH_PROTOCOL;

pub const EFI_ATA_PASS_THRU_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x1D3DE7F0,
        0x807,
        0x424F,
        [0xAA, 0x69, 0x11, 0xA5, 0x4E, 0x19, 0xA4, 0x6F],
    )
};

pub const EFI_ATA_PASS_THRU_ATTRIBUTES_PHYSICAL: UINT32 = 0x0001;
pub const EFI_ATA_PASS_THRU_ATTRIBUTES_LOGICAL: UINT32 = 0x0002;
pub const EFI_ATA_PASS_THRU_ATTRIBUTES_NONBLOCKIO: UINT32 = 0x0004;

pub const EFI_ATA_PASS_THRU_PROTOCOL_ATA_HARDWARE_RESET: UINT8 = 0x00;
pub const EFI_ATA_PASS_THRU_PROTOCOL_ATA_SOFTWARE_RESET: UINT8 = 0x01;
pub const EFI_ATA_PASS_THRU_PROTOCOL_ATA_NON_DATA: UINT8 = 0x02;
pub const EFI_ATA_PASS_THRU_PROTOCOL_PIO_DATA_IN: UINT8 = 0x04;
pub const EFI_ATA_PASS_THRU_PROTOCOL_PIO_DATA_OUT: UINT8 = 0x05;
pub const EFI_ATA_PASS_THRU_PROTOCOL_DMA: UINT8 = 0x06;
pub const EFI_ATA_PASS_THRU_PROTOCOL_DMA_QUEUED: UINT8 = 0x07;
pub const EFI_ATA_PASS_THRU_PROTOCOL_DEVICE_DIAGNOSTIC: UINT8 = 0x08;
pub const EFI_ATA_PASS_THRU_PROTOCOL_DEVICE_RESET: UINT8 = 0x09;
pub const EFI_ATA_PASS_THRU_PROTOCOL_UDMA_DATA_IN: UINT8 = 0x0A;
pub const EFI_ATA_PASS_THRU_PROTOCOL_UDMA_DATA_OUT: UINT8 = 0x0B;
pub const EFI_ATA_PASS_THRU_PROTOCOL_FPDMA: UINT8 = 0x0C;
pub const EFI_ATA_PASS_THRU_PROTOCOL_RETURN_RESPONSE: UINT8 = 0xFF;

pub const EFI_ATA_PASS_THRU_LENGTH_MASK: UINT8 = 0x70;
pub const EFI_ATA_PASS_THRU_LENGTH_BYTES: UINT8 = 0x80;

pub const EFI_ATA_PASS_THRU_LENGTH_NO_DATA_TRANSFER: UINT8 = 0x00;
pub const EFI_ATA_PASS_THRU_LENGTH_FEATURES: UINT8 = 0x10;
pub const EFI_ATA_PASS_THRU_LENGTH_SECTOR_COUNT: UINT8 = 0x20;
pub const EFI_ATA_PASS_THRU_LENGTH_TPSIU: UINT8 = 0x30;

pub const EFI_ATA_PASS_THRU_LENGTH_COUNT: UINT8 = 0x0F;

#[repr(C)]
pub struct EFI_ATA_PASS_THRU_PROTOCOL {
    pub Mode: *mut EFI_ATA_PASS_THRU_MODE,
    pub PassThru: unsafe extern "efiapi" fn(
        This: *mut EFI_ATA_PASS_THRU_PROTOCOL,
        Port: UINT16,
        PortMultiplierPort: UINT16,
        Packet: *mut EFI_ATA_PASS_THRU_COMMAND_PACKET,
    ) -> EFI_STATUS,
    pub GetNextPort: unsafe extern "efiapi" fn(
        This: *mut EFI_ATA_PASS_THRU_PROTOCOL,
        Port: *mut UINT16,
    ) -> EFI_STATUS,
    pub GetNextDevice: unsafe extern "efiapi" fn(
        This: *mut EFI_ATA_PASS_THRU_PROTOCOL,
        Port: UINT16,
        PortMultiplierPort: *mut UINT16,
    ) -> EFI_STATUS,
    pub BuildDevicePath: unsafe extern "efiapi" fn(
        This: *mut EFI_ATA_PASS_THRU_PROTOCOL,
        Port: UINT16,
        PortMultiplierPort: UINT16,
        DevicePath: *mut *mut EFI_DEVICE_PATH_PROTOCOL,
    ) -> EFI_STATUS,
    pub GetDevice: unsafe extern "efiapi" fn(
        This: *mut EFI_ATA_PASS_THRU_PROTOCOL,
        DevicePath: *mut *mut EFI_DEVICE_PATH_PROTOCOL,
        Port: *mut UINT16,
        PortMultiplierPort: *mut UINT16,
    ) -> EFI_STATUS,
    pub ResetPort: unsafe extern "efiapi" fn(
        This: *mut EFI_ATA_PASS_THRU_PROTOCOL,
        Port: *mut UINT16,
    ) -> EFI_STATUS,
    pub ResetDevice: unsafe extern "efiapi" fn(
        This: *mut EFI_ATA_PASS_THRU_PROTOCOL,
        Port: UINT16,
        PortMultiplierPort: UINT16,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_ATA_PASS_THRU_COMMAND_PACKET {
    pub Asb: *mut EFI_ATA_STATUS_BLOCK,
    pub Acb: *mut EFI_ATA_COMMAND_BLOCK,
    pub Timeout: UINT64,
    pub InDataBuffer: *mut VOID,
    pub OutDataBuffer: *mut VOID,
    pub InTransferLength: UINT32,
    pub OutTransferLength: UINT32,
    pub Protocol: EFI_ATA_PASS_THRU_CMD_PROTOCOL,
    pub Length: EFI_ATA_PASS_THRU_LENGTH,
}

#[repr(C)]
pub struct EFI_ATA_PASS_THRU_MODE {
    pub Attributes: UINT32,
    pub IoAlign: UINT32,
}

#[repr(C)]
pub struct EFI_ATA_COMMAND_BLOCK {
    #[doc(hidden)]
    Reserved1: [UINT8; 2],
    pub AtaCommand: UINT8,
    pub AtaFeatures: UINT8,
    pub AtaSectorNumber: UINT8,
    pub AtaCylinderLow: UINT8,
    pub AtaCylinderHigh: UINT8,
    pub AtaDeviceHead: UINT8,
    pub AtaSectorNumberExp: UINT8,
    pub AtaCylinderLowExp: UINT8,
    pub AtaCylinderHighExp: UINT8,
    pub AtaFeaturesExp: UINT8,
    pub AtaSectorCount: UINT8,
    pub AtaSectorCountExp: UINT8,
    #[doc(hidden)]
    Reserved2: [UINT8; 6],
}

#[repr(C)]
pub struct EFI_ATA_STATUS_BLOCK {
    #[doc(hidden)]
    Reserved1: [UINT8; 2],
    pub AtaStatus: UINT8,
    pub AtaError: UINT8,
    pub AtaSectorNumber: UINT8,
    pub AtaCylinderLow: UINT8,
    pub AtaCylinderHigh: UINT8,
    pub AtaDeviceHead: UINT8,
    pub AtaSectorNumberExp: UINT8,
    pub AtaCylinderLowExp: UINT8,
    pub AtaCylinderHighExp: UINT8,
    pub AtaFeaturesExp: UINT8,
    pub AtaSectorCount: UINT8,
    pub AtaSectorCountExp: UINT8,
    #[doc(hidden)]
    Reserved2: [UINT8; 6],
}

pub type EFI_ATA_PASS_THRU_CMD_PROTOCOL = UINT8;

pub type EFI_ATA_PASS_THRU_LENGTH = UINT8;
