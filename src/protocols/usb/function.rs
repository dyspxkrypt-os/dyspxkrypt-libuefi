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
    EFI_USB_CONFIG_DESCRIPTOR, EFI_USB_DEVICE_DESCRIPTOR, EFI_USB_DEVICE_REQUEST,
    EFI_USB_ENDPOINT_DESCRIPTOR, EFI_USB_INTERFACE_DESCRIPTOR,
};

pub const EFI_USBFN_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x32D2963A,
        0xFE5D,
        0x4F30,
        [0xB6, 0x33, 0x6E, 0x5D, 0xC5, 0x58, 0x03, 0xCC],
    )
};

pub const EFI_USBFN_IO_PROTOCOL_REVISION: UINT32 = 0x00010001;

#[repr(C)]
pub enum EFI_USB_BUS_SPEED {
    UsbBusSpeedUnknown,
    UsbBusSpeedLow,
    UsbBusSpeedFull,
    UsbBusSpeedHigh,
    UsbBusSpeedSuper,
    UsbBusSpeedMaximum,
}

#[repr(C)]
pub enum EFI_USB_ENDPOINT_TYPE {
    UsbEndpointControl,
    UsbEndpointIsochronous,
    UsbEndpointBulk,
    UsbEndpointInterrupt,
}

#[repr(C)]
pub enum EFI_USBFN_DEVICE_INFO_ID {
    EfiUsbDeviceInfoUnknown,
    EfiUsbDeviceInfoSerialNumber,
    EfiUsbDeviceInfoManufacturerName,
    EfiUsbDeviceInfoProductName,
}

#[repr(C)]
pub enum EFI_USBFN_ENDPOINT_DIRECTION {
    EfiUsbEndpointDirectionHostOut = 0,
    EfiUsbEndpointDirectionHostIn,
    EfiUsbEndpointDirectionDeviceTx,
    EfiUsbEndpointDirectionDeviceRx,
}

#[repr(C)]
pub enum EFI_USBFN_MESSAGE {
    EfiUsbMsgSetupPacket,
    EfiUsbMsgEndpointStatusChangedRx,
    EfiUsbMsgEndpointStatusChangedTx,
    EfiUsbMsgBusEventDetach,
    EfiUsbMsgBusEventAttach,
    EfiUsbMsgBusEventReset,
    EfiUsbMsgBusEventSuspend,
    EfiUsbMsgBusEventResume,
    EfiUsbMsgBusEventSpeed,
}

#[repr(C)]
pub enum EFI_USBFN_PORT_TYPE {
    EfiUsbUnknownPort,
    EfiUsbStandardDownstreamPort,
    EfiUsbChargingDownstreamPort,
    EfiUsbDedicatedChargingPort,
    EfiUsbInvalidDedicatedChargingPort,
}

#[repr(C)]
pub enum EFI_USBFN_TRANSFER_STATUS {
    UsbTransferStatusUnknown,
    UsbTransferStatusComplete,
    UsbTransferStatusAborted,
    UsbTransferStatusActive,
    UsbTransferStatusNone,
}

#[repr(C)]
pub struct EFI_USB_CONFIG_INFO {
    pub ConfigDescriptor: *mut EFI_USB_CONFIG_DESCRIPTOR,
    pub InterfaceInfoTable: *mut *mut EFI_USB_INTERFACE_INFO,
}

#[repr(C)]
pub struct EFI_USB_DEVICE_INFO {
    pub DeviceDescriptor: *mut EFI_USB_DEVICE_DESCRIPTOR,
    pub ConfigInfoTable: *mut *mut EFI_USB_CONFIG_INFO,
}

#[repr(C)]
pub struct EFI_USB_INTERFACE_INFO {
    pub InterfaceDescriptor: *mut EFI_USB_INTERFACE_DESCRIPTOR,
    pub EndpointDescriptorTable: *mut *mut EFI_USB_ENDPOINT_DESCRIPTOR,
}

#[repr(C)]
pub struct EFI_USBFN_IO_PROTOCOL {
    pub Revision: UINT32,
    pub DetectPort: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        PortType: *mut EFI_USBFN_PORT_TYPE,
    ) -> EFI_STATUS,
    pub ConfigureEnableEndpoints: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        DeviceInfo: *mut EFI_USB_DEVICE_INFO,
    ) -> EFI_STATUS,
    pub GetEndpointMaxPacketSize: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        EndpointType: EFI_USB_ENDPOINT_TYPE,
        BusSpeed: EFI_USB_BUS_SPEED,
        MaxPacketSize: *mut UINT16,
    ) -> EFI_STATUS,
    pub GetDeviceInfo: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        Id: EFI_USBFN_DEVICE_INFO_ID,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub GetVendorIdProductId: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        Vid: *mut UINT64,
        Pid: *mut UINT64,
    ) -> EFI_STATUS,
    pub AbortTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        EndpointIndex: UINT8,
        Direction: EFI_USBFN_ENDPOINT_DIRECTION,
    ) -> EFI_STATUS,
    pub GetEndpointStallState: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        EndpointIndex: UINT8,
        Direction: EFI_USBFN_ENDPOINT_DIRECTION,
        State: *mut BOOLEAN,
    ) -> EFI_STATUS,
    pub SetEndpointStallState: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        EndpointIndex: UINT8,
        Direction: EFI_USBFN_ENDPOINT_DIRECTION,
        State: BOOLEAN,
    ) -> EFI_STATUS,
    pub EventHandler: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        Message: *mut EFI_USBFN_MESSAGE,
        PayloadSize: *mut UINTN,
        Payload: *mut EFI_USBFN_MESSAGE_PAYLOAD,
    ) -> EFI_STATUS,
    pub Transfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        EndpointIndex: UINT8,
        Direction: EFI_USBFN_ENDPOINT_DIRECTION,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub GetMaxTransferSize: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        MaxTransferSize: *mut UINTN,
    ) -> EFI_STATUS,
    pub AllocateTransferBuffer: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        Size: UINTN,
        Buffer: *mut *mut VOID,
    ) -> EFI_STATUS,
    pub FreeTransferBuffer: unsafe extern "efiapi" fn(
        This: *mut EFI_USBFN_IO_PROTOCOL,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub StartController: unsafe extern "efiapi" fn(This: *mut EFI_USBFN_IO_PROTOCOL) -> EFI_STATUS,
    pub StopController: unsafe extern "efiapi" fn(This: *mut EFI_USBFN_IO_PROTOCOL) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_USBFN_TRANSFER_RESULT {
    pub BytesTransferred: UINTN,
    pub TransferStatus: EFI_USBFN_TRANSFER_STATUS,
    pub EndpointIndex: UINT8,
    pub Direction: EFI_USBFN_ENDPOINT_DIRECTION,
    pub Buffer: *mut VOID,
}

#[repr(C)]
pub union EFI_USBFN_MESSAGE_PAYLOAD {
    pub udr: EFI_USB_DEVICE_REQUEST,
    pub utr: EFI_USBFN_TRANSFER_RESULT,
    pub ubs: EFI_USB_BUS_SPEED,
}
