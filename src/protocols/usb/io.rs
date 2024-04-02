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

pub const EFI_USB_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x2B2F68D6,
        0x0CD2,
        0x44CF,
        [0x8E, 0x8B, 0xBB, 0xA2, 0x0B, 0x1B, 0x5B, 0x75],
    )
};

pub const EFI_USB_NOERROR: UINT16 = 0x0000;
pub const EFI_USB_ERR_NOTEXECUTE: UINT16 = 0x0001;
pub const EFI_USB_ERR_STALL: UINT16 = 0x0002;
pub const EFI_USB_ERR_BUFFER: UINT16 = 0x0004;
pub const EFI_USB_ERR_BABBLE: UINT16 = 0x0008;
pub const EFI_USB_ERR_NAK: UINT16 = 0x0010;
pub const EFI_USB_ERR_CRC: UINT16 = 0x0020;
pub const EFI_USB_ERR_TIMEOUT: UINT16 = 0x0040;
pub const EFI_USB_ERR_BITSTUFF: UINT16 = 0x0080;
pub const EFI_USB_ERR_SYSTEM: UINT16 = 0x0100;

#[repr(C)]
pub enum EFI_USB_DATA_DIRECTION {
    EfiUsbDataIn,
    EfiUsbDataOut,
    EfiUsbNoData,
}

#[repr(C)]
pub struct EFI_USB_CONFIG_DESCRIPTOR {
    pub Length: UINT8,
    pub DescriptorType: UINT8,
    pub TotalLength: UINT16,
    pub NumInterfaces: UINT8,
    pub ConfigurationValue: UINT8,
    pub Configuration: UINT8,
    pub Attributes: UINT8,
    pub MaxPower: UINT8,
}

#[repr(C)]
pub struct EFI_USB_DEVICE_DESCRIPTOR {
    pub Length: UINT8,
    pub DescriptorType: UINT8,
    pub BcdUSB: UINT16,
    pub DeviceClass: UINT8,
    pub DeviceSubClass: UINT8,
    pub DeviceProtocol: UINT8,
    pub MaxPacketSize0: UINT8,
    pub IdVendor: UINT16,
    pub IdProduct: UINT16,
    pub BcdDevice: UINT16,
    pub StrManufacturer: UINT8,
    pub StrProduct: UINT8,
    pub StrSerialNumber: UINT8,
    pub NumConfigurations: UINT8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EFI_USB_DEVICE_REQUEST {
    pub RequestType: UINT8,
    pub Request: UINT8,
    pub Value: UINT16,
    pub Index: UINT16,
    pub Length: UINT16,
}

#[repr(C)]
pub struct EFI_USB_ENDPOINT_DESCRIPTOR {
    pub Length: UINT8,
    pub DescriptorType: UINT8,
    pub EndpointAddress: UINT8,
    pub Attributes: UINT8,
    pub MaxPacketSize: UINT16,
    pub Interval: UINT8,
}

#[repr(C)]
pub struct EFI_USB_INTERFACE_DESCRIPTOR {
    pub Length: UINT8,
    pub DescriptorType: UINT8,
    pub InterfaceNumber: UINT8,
    pub AlternateSetting: UINT8,
    pub NumEndpoints: UINT8,
    pub InterfaceClass: UINT8,
    pub InterfaceSubClass: UINT8,
    pub InterfaceProtocol: UINT8,
    pub Interface: UINT8,
}

#[repr(C)]
pub struct EFI_USB_IO_PROTOCOL {
    pub UsbControlTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USB_IO_PROTOCOL,
        Request: *mut EFI_USB_DEVICE_REQUEST,
        Direction: EFI_USB_DATA_DIRECTION,
        Timeout: UINT32,
        Data: *mut VOID,
        DataLength: UINTN,
        Status: *mut UINT32,
    ) -> EFI_STATUS,
    pub UsbBulkTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USB_IO_PROTOCOL,
        DeviceEndpoint: UINT8,
        Data: *mut VOID,
        DataLength: *mut UINTN,
        Timeout: UINTN,
        Status: *mut UINT32,
    ) -> EFI_STATUS,
    pub UsbAsyncInterruptTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USB_IO_PROTOCOL,
        DeviceEndpoint: UINT8,
        IsNewTransfer: BOOLEAN,
        PollingInterval: UINTN,
        DataLength: UINTN,
        InterruptCallBack: EFI_ASYNC_USB_TRANSFER_CALLBACK,
        Context: *mut VOID,
    ) -> EFI_STATUS,
    pub UsbSyncInterruptTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USB_IO_PROTOCOL,
        DeviceEndpoint: UINT8,
        Data: *mut VOID,
        DataLength: *mut UINTN,
        Timeout: UINTN,
        Status: *mut UINT32,
    ) -> EFI_STATUS,
    pub UsbIsochronousTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USB_IO_PROTOCOL,
        DeviceEndpoint: UINT8,
        Data: *mut VOID,
        DataLength: UINTN,
        Status: *mut UINT32,
    ) -> EFI_STATUS,
    pub UsbAsyncIsochronousTransfer: unsafe extern "efiapi" fn(
        This: *mut EFI_USB_IO_PROTOCOL,
        DeviceEndpoint: UINT8,
        Data: *mut VOID,
        DataLength: UINTN,
        IsochronousCallBack: EFI_ASYNC_USB_TRANSFER_CALLBACK,
        Context: *mut VOID,
    ) -> EFI_STATUS,
    pub UsbGetDeviceDescriptor: unsafe extern "efiapi" fn(
        This: *mut EFI_USB_IO_PROTOCOL,
        DeviceDescriptor: *mut EFI_USB_DEVICE_DESCRIPTOR,
    ) -> EFI_STATUS,
    pub UsbGetConfigDescriptor: unsafe extern "efiapi" fn(
        This: *mut EFI_USB_IO_PROTOCOL,
        ConfigurationDescriptor: *mut EFI_USB_CONFIG_DESCRIPTOR,
    ) -> EFI_STATUS,
    pub UsbGetInterfaceDescriptor: unsafe extern "efiapi" fn(
        This: *mut EFI_USB_IO_PROTOCOL,
        InterfaceDescriptor: *mut EFI_USB_INTERFACE_DESCRIPTOR,
    ) -> EFI_STATUS,
    pub UsbGetEndpointDescriptor: unsafe extern "efiapi" fn(
        This: *mut EFI_USB_IO_PROTOCOL,
        EndpointIndex: UINT8,
        EndpointDescriptor: *mut EFI_USB_ENDPOINT_DESCRIPTOR,
    ) -> EFI_STATUS,
    pub UsbGetStringDescriptor: unsafe extern "efiapi" fn(
        This: *mut EFI_USB_IO_PROTOCOL,
        LangID: UINT16,
        StringID: UINT8,
        String: *mut *mut CHAR16,
    ) -> EFI_STATUS,
    pub UsbGetSupportedLanguages: unsafe extern "efiapi" fn(
        This: *mut EFI_USB_IO_PROTOCOL,
        LangIDTable: *mut *mut UINT16,
        TableSize: *mut UINT16,
    ) -> EFI_STATUS,
    pub UsbPortReset: unsafe extern "efiapi" fn(This: *mut EFI_USB_IO_PROTOCOL) -> EFI_STATUS,
}

pub type EFI_ASYNC_USB_TRANSFER_CALLBACK = unsafe extern "efiapi" fn(
    Data: *mut VOID,
    DataLength: UINTN,
    Context: *mut VOID,
    Status: UINT32,
) -> EFI_STATUS;
