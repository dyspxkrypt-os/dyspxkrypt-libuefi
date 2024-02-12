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

pub const EFI_SD_MMC_PASS_THRU_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x716EF0D9,
        0xFF83,
        0x4F69,
        [0x81, 0xE9, 0x51, 0x8B, 0xD3, 0x9A, 0x8E, 0x70],
    )
};

#[repr(C)]
pub enum EFI_SD_MMC_COMMAND_TYPE {
    SdMmcCommandTypeBc,
    SdMmcCommandTypeBcr,
    SdMmcCommandTypeAc,
    SdMmcCommandTypeAdtc,
}

#[repr(C)]
pub enum EFI_SD_MMC_RESPONSE_TYPE {
    SdMmcResponseTypeR1,
    SdMmcResponseTypeR1b,
    SdMmcResponseTypeR2,
    SdMmcResponseTypeR3,
    SdMmcResponseTypeR4,
    SdMmcResponseTypeR5,
    SdMmcResponseTypeR5b,
    SdMmcResponseTypeR6,
    SdMmcResponseTypeR7,
}

#[repr(C)]
pub struct EFI_SD_MMC_PASS_THRU_PROTOCOL {
    pub IoAlign: UINTN,
    pub PassThru: unsafe extern "efiapi" fn(
        This: *mut EFI_SD_MMC_PASS_THRU_PROTOCOL,
        Slot: UINT8,
        Packet: *mut EFI_SD_MMC_PASS_THRU_COMMAND_PACKET,
        Event: EFI_EVENT,
    ) -> EFI_STATUS,
    pub GetNextSlot: unsafe extern "efiapi" fn(
        This: *mut EFI_SD_MMC_PASS_THRU_PROTOCOL,
        Slot: *mut UINT8,
    ) -> EFI_STATUS,
    pub BuildDevicePath: unsafe extern "efiapi" fn(
        This: *mut EFI_SD_MMC_PASS_THRU_PROTOCOL,
        Slot: UINT8,
        DevicePath: *mut *mut EFI_DEVICE_PATH_PROTOCOL,
    ) -> EFI_STATUS,
    pub GetSlotNumber: unsafe extern "efiapi" fn(
        This: *mut EFI_SD_MMC_PASS_THRU_PROTOCOL,
        DevicePath: *mut EFI_DEVICE_PATH_PROTOCOL,
        Slot: *mut UINT8,
    ) -> EFI_STATUS,
    pub ResetDevice: unsafe extern "efiapi" fn(
        This: *mut EFI_SD_MMC_PASS_THRU_PROTOCOL,
        Slot: UINT8,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_SD_MMC_COMMAND_BLOCK {
    pub CommandIndex: UINT16,
    pub CommandArgument: UINT32,
    pub CommandType: UINT32,
    pub ResponseType: UINT32,
}

#[repr(C)]
pub struct EFI_SD_MMC_STATUS_BLOCK {
    pub Resp0: UINT32,
    pub Resp1: UINT32,
    pub Resp2: UINT32,
    pub Resp3: UINT32,
}

#[repr(C)]
pub struct EFI_SD_MMC_PASS_THRU_COMMAND_PACKET {
    pub SdMmcCmdBlk: *mut EFI_SD_MMC_COMMAND_BLOCK,
    pub SdMmcStatusBlk: *mut EFI_SD_MMC_STATUS_BLOCK,
    pub Timeout: UINT64,
    pub InDataBuffer: *mut VOID,
    pub OutDataBuffer: *mut VOID,
    pub InTransferLength: UINT32,
    pub OutTransferLength: UINT32,
    pub TransactionStatus: EFI_STATUS,
}
