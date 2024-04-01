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

use crate::protocols::usb::io::{
    EFI_ASYNC_USB_TRANSFER_CALLBACK, EFI_USB_DATA_DIRECTION, EFI_USB_DEVICE_REQUEST,
};

pub const EFI_USB2_HC_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x3E745226,
        0x9818,
        0x45B6,
        [0xA2, 0xAC, 0xD7, 0xCD, 0x0E, 0x8B, 0xA2, 0xBC],
    )
};

pub const EFI_USB_SPEED_FULL: UINT8 = 0x00;
pub const EFI_USB_SPEED_LOW: UINT8 = 0x01;
pub const EFI_USB_SPEED_HIGH: UINT8 = 0x02;
pub const EFI_USB_SPEED_SUPER: UINT8 = 0x03;

pub const EFI_USB_HC_RESET_GLOBAL: UINT16 = 0x0001;
pub const EFI_USB_HC_RESET_HOST_CONTROLLER: UINT16 = 0x0002;
pub const EFI_USB_HC_RESET_GLOBAL_WITH_DEBUG: UINT16 = 0x0004;
pub const EFI_USB_HC_RESET_HOST_WITH_DEBUG: UINT16 = 0x0008;

pub const EFI_USB_MAX_BULK_BUFFER_NUM: UINTN = 10;

pub const EFI_USB_MAX_ISO_BUFFER_NUM: UINTN = 7;

pub const USB_PORT_STAT_CONNECTION: UINT16 = 0x0001;
pub const USB_PORT_STAT_ENABLE: UINT16 = 0x0002;
pub const USB_PORT_STAT_SUSPEND: UINT16 = 0x0004;
pub const USB_PORT_STAT_OVERCURRENT: UINT16 = 0x0008;
pub const USB_PORT_STAT_RESET: UINT16 = 0x0010;
pub const USB_PORT_STAT_POWER: UINT16 = 0x0100;
pub const USB_PORT_STAT_LOW_SPEED: UINT16 = 0x0200;
pub const USB_PORT_STAT_HIGH_SPEED: UINT16 = 0x0400;
pub const USB_PORT_STAT_SUPER_SPEED: UINT16 = 0x0800;
pub const USB_PORT_STAT_OWNER: UINT16 = 0x2000;

pub const USB_PORT_STAT_C_CONNECTION: UINT16 = 0x0001;
pub const USB_PORT_STAT_C_ENABLE: UINT16 = 0x0002;
pub const USB_PORT_STAT_C_SUSPEND: UINT16 = 0x0004;
pub const USB_PORT_STAT_C_OVERCURRENT: UINT16 = 0x0008;
pub const USB_PORT_STAT_C_RESET: UINT16 = 0x0010;

#[repr(C)]
pub enum EFI_USB_HC_STATE {
    EfiUsbHcStateHalt,
    EfiUsbHcStateOperational,
    EfiUsbHcStateSuspend,
    EfiUsbHcStateMaximum,
}

pub enum EFI_USB_PORT_FEATURE {
    EfiUsbPortEnable = 1,
    EfiUsbPortSuspend = 2,
    EfiUsbPortReset = 4,
    EfiUsbPortPower = 8,
    EfiUsbPortOwner = 13,
    EfiUsbPortConnectChange = 16,
    EfiUsbPortEnableChange = 17,
    EfiUsbPortSuspendChange = 18,
    EfiUsbPortOverCurrentChange = 19,
    EfiUsbPortResetChange = 20,
}

#[repr(C)]
pub struct EFI_USB_PORT_STATUS {
    pub PortStatus: UINT16,
    pub PortChangeStatus: UINT16,
}

#[repr(C)]
pub struct EFI_USB2_HC_PROTOCOL {
    pub GetCapability: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        MaxSpeed: *mut UINT8,
        PortNumber: *mut UINT8,
        Is64BitCapable: *mut UINT8,
    ) -> EFI_STATUS,
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        Attributes: *mut UINT16,
    ) -> EFI_STATUS,
    pub GetState: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        State: *mut EFI_USB_HC_STATE,
    ) -> EFI_STATUS,
    pub SetState: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        State: EFI_USB_HC_STATE,
    ) -> EFI_STATUS,
    pub ControlTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        DeviceAddress: UINT8,
        DeviceSpeed: UINT8,
        MaximumPacketLength: UINTN,
        Request: *mut EFI_USB_DEVICE_REQUEST,
        TransferDirection: EFI_USB_DATA_DIRECTION,
        Data: *mut VOID,
        DataLength: *mut UINTN,
        TimeOut: UINTN,
        Translator: EFI_USB2_HC_TRANSACTION_TRANSLATOR,
        TransferResult: *mut UINT32,
    ) -> EFI_STATUS,
    pub BulkTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        DeviceAddress: UINT8,
        DeviceSpeed: UINT8,
        MaximumPacketLength: UINTN,
        DataBuffersNumber: UINT8,
        Data: [*mut VOID; EFI_USB_MAX_BULK_BUFFER_NUM],
        DataLength: *mut UINTN,
        DataToggle: *mut UINT8,
        TimeOut: UINTN,
        Translator: EFI_USB2_HC_TRANSACTION_TRANSLATOR,
        TransferResult: *mut UINT32,
    ) -> EFI_STATUS,
    pub AsyncInterruptTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        DeviceAddress: UINT8,
        EndPointAddress: UINT8,
        DeviceSpeed: UINT8,
        MaximumPacketLength: UINTN,
        IsNewTransfer: BOOLEAN,
        DataToggle: *mut UINT8,
        PollingInterval: UINTN,
        DataLength: UINTN,
        Translator: EFI_USB2_HC_TRANSACTION_TRANSLATOR,
        CallbackFunction: EFI_ASYNC_USB_TRANSFER_CALLBACK,
        Context: *mut VOID,
    ) -> EFI_STATUS,
    pub SyncInterruptTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        DeviceAddress: UINT8,
        EndPointAddress: UINT8,
        DeviceSpeed: UINT8,
        MaximumPacketLength: UINTN,
        Data: *mut VOID,
        DataLength: *mut UINTN,
        DataToggle: *mut UINT8,
        Translator: EFI_USB2_HC_TRANSACTION_TRANSLATOR,
        Context: *mut VOID,
    ) -> EFI_STATUS,
    pub IsochronousTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        DeviceAddress: UINT8,
        EndPointAddress: UINT8,
        DeviceSpeed: UINT8,
        MaximumPacketLength: UINTN,
        DataBuffersNumber: UINT8,
        Data: [*mut VOID; EFI_USB_MAX_ISO_BUFFER_NUM],
        DataLength: UINTN,
        Translator: EFI_USB2_HC_TRANSACTION_TRANSLATOR,
        TransferResult: *mut UINT32,
    ) -> EFI_STATUS,
    pub AsyncIsochronousTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        DeviceAddress: UINT8,
        EndPointAddress: UINT8,
        DeviceSpeed: UINT8,
        MaximumPacketLength: UINTN,
        DataBuffersNumber: UINT8,
        Data: [*mut VOID; EFI_USB_MAX_ISO_BUFFER_NUM],
        DataLength: UINTN,
        Translator: EFI_USB2_HC_TRANSACTION_TRANSLATOR,
        CallbackFunction: EFI_ASYNC_USB_TRANSFER_CALLBACK,
        Context: *mut VOID,
    ) -> EFI_STATUS,
    pub GetRootHubPortStatus: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        PortNumber: UINT8,
        PortStatus: *mut EFI_USB_PORT_STATUS,
    ) -> EFI_STATUS,
    pub SetRootHubPortFeature: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        PortNumber: UINT8,
        PortFeature: EFI_USB_PORT_FEATURE,
    ) -> EFI_STATUS,
    pub ClearRootHubPortFeature: unsafe extern "efiapi" fn(
        This: *mut EFI_USB2_HC_PROTOCOL,
        PortNumber: UINT8,
        PortFeature: EFI_USB_PORT_FEATURE,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_USB2_HC_TRANSACTION_TRANSLATOR {
    pub TranslatorHubAddress: UINT8,
    pub TranslatorPortNumber: UINT8,
}
