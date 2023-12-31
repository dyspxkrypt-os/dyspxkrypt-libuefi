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

/// This protocol is used to communicate with any type of character-based I/O device.
///
/// The Serial I/O protocol is used to communicate with UART-style serial devices. These can be standard UART serial
/// ports in PC-AT systems, serial ports attached to a USB interface, or potentially any character-based I/O device.
///
/// The Serial I/O protocol can control byte I/O style devices from a generic device, to a device with features such as
/// a UART. As such many of the serial I/O features are optional to allow for the case of devices that do not have UART
/// controls. Each of these options is called out in the specific serial I/O functions.
///
/// The default attributes for all UART-style serial device interfaces are: 115,200 baud, a 1 byte receive FIFO, a 1,000,000
/// microsecond timeout per character, no parity, 8 data bits, and 1 stop bit. Flow control is the responsibility of the
/// software that uses the protocol. Hardware flow control can be implemented through the use of the
/// `EFI_SERIAL_IO_PROTOCOL.GetControl()` and `EFI_SERIAL_IO_PROTOCOL.SetControl()` functions to monitor and assert the
/// flow control signals. The XON/XOFF flow control algorithm can be implemented in software by inserting XON and XOFF
/// characters into the serial data stream as required.
///
/// Special care must be taken if a significant amount of data is going to be read from a serial device. Since UEFI
/// drivers are polled mode drivers, characters received on a serial device might be missed. It is the responsibility
/// of the software that uses the protocol to check for new data often enough to guarantee that no characters will be
/// missed. The required polling frequency depends on the baud rate of the connection and the depth of the receive FIFO.
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
    pub Reset: unsafe extern "efiapi" fn(This: *mut EFI_SERIAL_IO_PROTOCOL) -> EFI_STATUS,
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
    /// Sets the control bits on a serial device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_ABSOLUTE_POINTER_PROTOCOL` instance. |
    /// | **IN** `Control` | Sets the bits of `Control` that are settable. |
    ///
    /// ## Description
    ///
    /// The `SetControl()` function is used to assert or de-assert the control signals on a serial device. The following
    /// signals are set according their bit settings:
    ///
    /// - Request to Send
    ///
    /// - Data Terminal Ready
    ///
    /// Only the `REQUEST_TO_SEND`, `DATA_TERMINAL_READY`, `HARDWARE_LOOPBACK_ENABLE`, `SOFTWARE_LOOPBACK_ENABLE`, and
    /// `HARDWARE_FLOW_CONTROL_ENABLE` bits can be set with `SetControl()`. All the bits can be read with
    /// `EFI_SERIAL_IO_PROTOCOL.GetControl()`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The new control bits were set on the serial device. |
    /// | `EFI_UNSUPPORTED` | The serial device does not support this operation. |
    /// | `EFI_DEVICE_ERROR` | The serial device is not functioning correctly. |
    pub SetControl:
        unsafe extern "efiapi" fn(This: *mut EFI_SERIAL_IO_PROTOCOL, Control: UINT32) -> EFI_STATUS,
    /// Retrieves the status of the control bits on a serial device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_ABSOLUTE_POINTER_PROTOCOL` instance. |
    /// | **OUT** `Control` | A pointer to return the current control signals from the serial device. |
    ///
    /// ## Description
    ///
    /// The `GetControl()` function retrieves the status of the control bits on a serial device.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The control bits were read from the serial device. |
    /// | `EFI_DEVICE_ERROR` | The serial device is not functioning correctly. |
    pub GetControl: unsafe extern "efiapi" fn(
        This: *mut EFI_SERIAL_IO_PROTOCOL,
        Control: *mut UINT32,
    ) -> EFI_STATUS,
    /// Writes data to a serial device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_ABSOLUTE_POINTER_PROTOCOL` instance. |
    /// | **IN OUT** `BufferSize` | On input, the size of the `Buffer`. On output, the amount of data actually written. |
    /// | **IN** `Buffer` | The buffer of data to write. |
    ///
    /// ## Description
    ///
    /// The `Write()` function writes the specified number of bytes to a serial device. If a time out error occurs while
    /// data is being sent to the serial port, transmission of this buffer will terminate, and `EFI_TIMEOUT` will be
    /// returned. In all cases the number of bytes actually written to the serial device is returned in `BufferSize`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The data was written. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error. |
    /// | `EFI_TIMEOUT` | The data write was stopped due to a timeout. |
    pub Write: unsafe extern "efiapi" fn(
        This: *mut EFI_SERIAL_IO_PROTOCOL,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    /// Reads data from a serial device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_ABSOLUTE_POINTER_PROTOCOL` instance. |
    /// | **IN OUT** `BufferSize` | On input, the size of the `Buffer`. On output, the amount of data returned in `Buffer`. |
    /// | **IN** `Buffer` | The buffer to return the data into. |
    ///
    /// ## Description
    ///
    /// The `Read()` function reads a specified number of bytes from a serial device. If a time out error or an overrun
    /// error is detected while data is being read from the serial device, then no more characters will be read, and an
    /// error will be returned. In all cases the number of bytes actually read is returned in `BufferSize`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The data was read. |
    /// | `EFI_DEVICE_ERROR` | The device reported an error. |
    /// | `EFI_TIMEOUT` | The operation was stopped due to a timeout or overrun. |
    pub Read: unsafe extern "efiapi" fn(
        This: *mut EFI_SERIAL_IO_PROTOCOL,
        BufferSize: *mut UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    /// Pointer to `SERIAL_IO_MODE` data.
    pub Mode: *mut SERIAL_IO_MODE,
    /// Pointer to a GUID identifying the device connected to the serial port. This field is `NULL` when the protocol is
    /// installed by the serial port driver and may be populated by a platform driver for a serial port with a known device
    /// attached. The field will remain `NULL` if there is no platform serial device identification information available.
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
