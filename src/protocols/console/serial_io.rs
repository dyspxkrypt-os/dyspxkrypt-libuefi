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

use crate::types::{EFI_GUID, EFI_STATUS, UINT32, UINT64, UINT8};

pub const EFI_SERIAL_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xBB25CF6F,
        0xF1D4,
        0x11D2,
        [0x9A, 0x0C, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0xFD],
    )
};

pub const EFI_SERIAL_IO_PROTOCOL_REVISION: UINT32 = 0x00010000;
pub const EFI_SERIAL_IO_PROTOCOL_REVISION1p1: UINT32 = 0x00010001;

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

/// This protocol is used to communicate with any type of character-based I/O device.
#[repr(C)]
pub struct EFI_SERIAL_IO_PROTOCOL {
    /// The revision to which the `EFI_SERIAL_IO_PROTOCOL` adheres. All future revisions must be
    /// backwards compatible. If a future version is not back wards compatible, it is not the same
    /// GUID.
    pub Revision: UINT32,
    /// Resets the serial device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_SERIAL_IO_PROTOCOL` instance. |
    ///
    /// ## Description
    ///
    /// The `Reset()` function resets the hardware of a serial device.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The device was reset. |
    /// | `EFI_DEVICE_ERROR` | The device is not functioning correctly and could not be reset. |
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_SERIAL_IO_PROTOCOL,
    ) -> EFI_STATUS,
    /// Sets the baud rate, receive FIFO depth, transmit/receive time out, parity, data bits, and
    /// stop bits on a serial device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_ABSOLUTE_POINTER_PROTOCOL` instance. |
    /// | **IN** `BaudRate` | The requested baud rate. A `BaudRate` value of `0` will use the device’s default interface speed. |
    /// | **IN** `ReceiveFifoDepth` | The requested depth of the FIFO on the receive side of the serial interface. A `ReceiveFifoDepth` value of `0` will use the device’s default FIFO depth. |
    /// | **IN** `Timeout` | The requested time out for a single character in microseconds. This timeout applies to both the transmit and receive side of the interface. A `Timeout` value of `0` will use the device’s default time out value. |
    /// | **IN** `Parity` | The type of parity to use on this serial device. A `Parity` value of `DefaultParity` will use the device’s default parity value. |
    /// | **IN** `DataBits` | The number of data bits to use on this serial device. A `DataBits` value of `0` will use the device’s default data bit setting. |
    /// | **IN** `StopBits` | The number of stop bits to use on this serial device. A `StopBits` value of `DefaultStopBits` will use the device’s default number of stop bits. |
    ///
    /// ## Description
    ///
    /// The `SetAttributes()` function sets the baud rate, receive-FIFO depth, transmit/receive timeout,
    /// parity, data bits, and stop bits on a serial device.
    ///
    /// The controller for a serial device is programmed with the specified attributes. If the `Parity`,
    /// `DataBits`, or `StopBits` values are not valid, then an error will be returned. If the specified
    /// `BaudRate` is below the minimum baud rate supported by the serial device, an error will be
    /// returned. The nearest baud rate supported by the serial device will be selected without exceeding
    /// the `BaudRate` parameter. If the specified `ReceiveFifoDepth` is below the smallest FIFO size
    /// supported by the serial device, an error will be returned. The nearest FIFO size supported by
    /// the serial device will be selected without exceeding the `ReceiveFifoDepth` parameter.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The new attributes were set on the serial device. |
    /// | `EFI_INVALID_PARAMETER` | One or more of the attributes has an unsupported value. |
    /// | `EFI_DEVICE_ERROR` | The serial device is not functioning correctly. |
    pub SetAttributes: unsafe extern "efiapi" fn(
        This: *mut EFI_SERIAL_IO_PROTOCOL,
        BaudRate: UINT64,
        ReceiveFifoDepth: UINT32,
        Timeout: UINT32,
        Parity: EFI_PARITY_TYPE,
        DataBits: UINT8,
        StopBits: EFI_STOP_BITS_TYPE,
    ) -> EFI_STATUS,
}
