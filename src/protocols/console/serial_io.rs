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

use crate::types::{EFI_GUID, EFI_STATUS, UINT32, UINT64, UINT8, UINTN, VOID};

pub const EFI_SERIAL_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xBB25CF6F,
        0xF1D4,
        0x11D2,
        [0x9A, 0x0C, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0xFD],
    )
};

pub const EFI_SERIAL_IO_PROTOCOL_REVISION: UINT32 = 0x00010000;
#[cfg(feature = "console-serial-v1p1")]
#[cfg_attr(doc, doc(cfg(feature = "console-serial-v1p1")))]
#[cfg_attr(docsrs, doc(cfg(feature = "console-serial-v1p1")))]
pub const EFI_SERIAL_IO_PROTOCOL_REVISION1p1: UINT32 = 0x00010001;

pub const EFI_SERIAL_CLEAR_TO_SEND: UINT32 = 0x0010;
pub const EFI_SERIAL_DATA_SET_READY: UINT32 = 0x0020;
pub const EFI_SERIAL_RING_INDICATE: UINT32 = 0x0040;
pub const EFI_SERIAL_CARRIER_DETECT: UINT32 = 0x0080;
pub const EFI_SERIAL_REQUEST_TO_SEND: UINT32 = 0x0002;
pub const EFI_SERIAL_DATA_TERMINAL_READY: UINT32 = 0x0001;
pub const EFI_SERIAL_INPUT_BUFFER_EMPTY: UINT32 = 0x0100;
pub const EFI_SERIAL_OUTPUT_BUFFER_EMPTY: UINT32 = 0x0200;
pub const EFI_SERIAL_HARDWARE_LOOPBACK_ENABLE: UINT32 = 0x1000;
pub const EFI_SERIAL_SOFTWARE_LOOPBACK_ENABLE: UINT32 = 0x2000;
pub const EFI_SERIAL_HARDWARE_FLOW_CONTROL_ENABLE: UINT32 = 0x4000;

#[repr(C)]
pub enum EFI_PARITY_TYPE {
    DefaultParity,
    NoParity,
    EvenParity,
    OddParity,
    MarkParity,
    SpaceParity,
}

#[repr(C)]
pub enum EFI_STOP_BITS_TYPE {
    DefaultStopBits,
    OneStopBit,
    OneFiveStopBits,
    TwoStopBits,
}

#[repr(C)]
pub struct EFI_SERIAL_IO_PROTOCOL {
    pub Revision: UINT32,
    pub Reset: unsafe extern "efiapi" fn(This: *mut EFI_SERIAL_IO_PROTOCOL) -> EFI_STATUS,
    pub SetAttributes: unsafe extern "efiapi" fn(
        This: *mut EFI_SERIAL_IO_PROTOCOL,
        BaudRate: UINT64,
        ReceiveFifoDepth: UINT32,
        Timeout: UINT32,
        Parity: EFI_PARITY_TYPE,
        DataBits: UINT8,
        StopBits: EFI_STOP_BITS_TYPE,
    ) -> EFI_STATUS,
    pub SetControl:
        unsafe extern "efiapi" fn(This: *mut EFI_SERIAL_IO_PROTOCOL, Control: UINT32) -> EFI_STATUS,
    pub GetControl: unsafe extern "efiapi" fn(
        This: *mut EFI_SERIAL_IO_PROTOCOL,
        Control: *mut UINT32,
    ) -> EFI_STATUS,
    pub Write: unsafe extern "efiapi" fn(
        This: *mut EFI_SERIAL_IO_PROTOCOL,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub Read: unsafe extern "efiapi" fn(
        This: *mut EFI_SERIAL_IO_PROTOCOL,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub Mode: *mut SERIAL_IO_MODE,
    #[cfg(feature = "console-serial-v1p1")]
    #[cfg_attr(doc, doc(cfg(feature = "console-serial-v1p1")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "console-serial-v1p1")))]
    pub DeviceTypeGuid: *const EFI_GUID,
}

#[repr(C)]
pub struct SERIAL_IO_MODE {
    pub ControlMask: UINT32,
    pub Timeout: UINT32,
    pub BaudRate: UINT64,
    pub ReceiveFifoDepth: UINT32,
    pub DataBits: UINT32,
    pub Parity: UINT32,
    pub StopBits: UINT32,
}
