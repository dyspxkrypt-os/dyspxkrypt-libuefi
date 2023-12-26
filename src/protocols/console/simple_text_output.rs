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

use crate::protocols::console::EFI_SIMPLE_TEXT_INPUT_PROTOCOL;
use crate::types::{BOOLEAN, CHAR16, EFI_GUID, EFI_STATUS, INT32, UINTN};

pub const EFI_SIMPLE_TEXT_INPUT_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x387477C2,
        0x69C7,
        0x11D2,
        [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

pub const BOXDRAW_HORIZONTAL: CHAR16 = 0x2500;
pub const BOXDRAW_VERTICAL: CHAR16 = 0x2502;
pub const BOXDRAW_DOWN_RIGHT: CHAR16 = 0x250c;
pub const BOXDRAW_DOWN_LEFT: CHAR16 = 0x2510;
pub const BOXDRAW_UP_RIGHT: CHAR16 = 0x2514;
pub const BOXDRAW_UP_LEFT: CHAR16 = 0x2518;
pub const BOXDRAW_VERTICAL_RIGHT: CHAR16 = 0x251c;
pub const BOXDRAW_VERTICAL_LEFT: CHAR16 = 0x2524;
pub const BOXDRAW_DOWN_HORIZONTAL: CHAR16 = 0x252c;
pub const BOXDRAW_UP_HORIZONTAL: CHAR16 = 0x2534;
pub const BOXDRAW_VERTICAL_HORIZONTAL: CHAR16 = 0x253c;

pub const BOXDRAW_DOUBLE_HORIZONTAL: CHAR16 = 0x2550;
pub const BOXDRAW_DOUBLE_VERTICAL: CHAR16 = 0x2551;
pub const BOXDRAW_DOWN_RIGHT_DOUBLE: CHAR16 = 0x2552;
pub const BOXDRAW_DOWN_DOUBLE_RIGHT: CHAR16 = 0x2553;
pub const BOXDRAW_DOUBLE_DOWN_RIGHT: CHAR16 = 0x2554;
pub const BOXDRAW_DOWN_LEFT_DOUBLE: CHAR16 = 0x2555;
pub const BOXDRAW_DOWN_DOUBLE_LEFT: CHAR16 = 0x2556;
pub const BOXDRAW_DOUBLE_DOWN_LEFT: CHAR16 = 0x2557;

pub const BOXDRAW_UP_RIGHT_DOUBLE: CHAR16 = 0x2558;
pub const BOXDRAW_UP_DOUBLE_RIGHT: CHAR16 = 0x2559;
pub const BOXDRAW_DOUBLE_UP_RIGHT: CHAR16 = 0x255a;
pub const BOXDRAW_UP_LEFT_DOUBLE: CHAR16 = 0x255b;
pub const BOXDRAW_UP_DOUBLE_LEFT: CHAR16 = 0x255c;
pub const BOXDRAW_DOUBLE_UP_LEFT: CHAR16 = 0x255d;

pub const BOXDRAW_VERTICAL_RIGHT_DOUBLE: CHAR16 = 0x255e;
pub const BOXDRAW_VERTICAL_DOUBLE_RIGHT: CHAR16 = 0x255f;
pub const BOXDRAW_DOUBLE_VERTICAL_RIGHT: CHAR16 = 0x2560;

pub const BOXDRAW_VERTICAL_LEFT_DOUBLE: CHAR16 = 0x2561;
pub const BOXDRAW_VERTICAL_DOUBLE_LEFT: CHAR16 = 0x2562;
pub const BOXDRAW_DOUBLE_VERTICAL_LEFT: CHAR16 = 0x2563;

pub const BOXDRAW_DOWN_HORIZONTAL_DOUBLE: CHAR16 = 0x2564;
pub const BOXDRAW_DOWN_DOUBLE_HORIZONTAL: CHAR16 = 0x2565;
pub const BOXDRAW_DOUBLE_DOWN_HORIZONTAL: CHAR16 = 0x2566;

pub const BOXDRAW_UP_HORIZONTAL_DOUBLE: CHAR16 = 0x2567;
pub const BOXDRAW_UP_DOUBLE_HORIZONTAL: CHAR16 = 0x2568;
pub const BOXDRAW_DOUBLE_UP_HORIZONTAL: CHAR16 = 0x2569;

pub const BOXDRAW_VERTICAL_HORIZONTAL_DOUBLE: CHAR16 = 0x256a;
pub const BOXDRAW_VERTICAL_DOUBLE_HORIZONTAL: CHAR16 = 0x256b;
pub const BOXDRAW_DOUBLE_VERTICAL_HORIZONTAL: CHAR16 = 0x256c;

pub const BLOCKELEMENT_FULL_BLOCK: CHAR16 = 0x2588;
pub const BLOCKELEMENT_LIGHT_SHADE: CHAR16 = 0x2591;

pub const GEOMETRICSHAPE_UP_TRIANGLE: CHAR16 = 0x25b2;
pub const GEOMETRICSHAPE_RIGHT_TRIANGLE: CHAR16 = 0x25ba;
pub const GEOMETRICSHAPE_DOWN_TRIANGLE: CHAR16 = 0x25bc;
pub const GEOMETRICSHAPE_LEFT_TRIANGLE: CHAR16 = 0x25c4;

pub const ARROW_UP: CHAR16 = 0x2191;
pub const ARROW_DOWN: CHAR16 = 0x2193;

pub const EFI_BLACK: UINTN = 0x00;
pub const EFI_BLUE: UINTN = 0x01;
pub const EFI_GREEN: UINTN = 0x02;
pub const EFI_CYAN: UINTN = 0x03;
pub const EFI_RED: UINTN = 0x04;
pub const EFI_MAGENTA: UINTN = 0x05;
pub const EFI_BROWN: UINTN = 0x06;
pub const EFI_LIGHTGRAY: UINTN = 0x07;
pub const EFI_BRIGHT: UINTN = 0x08;
pub const EFI_DARKGRAY: UINTN = 0x08;
pub const EFI_LIGHTBLUE: UINTN = 0x09;
pub const EFI_LIGHTGREEN: UINTN = 0x0A;
pub const EFI_LIGHTCYAN: UINTN = 0x0B;
pub const EFI_LIGHTRED: UINTN = 0x0C;
pub const EFI_LIGHTMAGENTA: UINTN = 0x0D;
pub const EFI_YELLOW: UINTN = 0x0E;
pub const EFI_WHITE: UINTN = 0x0F;

pub const EFI_BACKGROUND_BLACK: UINTN = 0x00;
pub const EFI_BACKGROUND_BLUE: UINTN = 0x10;
pub const EFI_BACKGROUND_GREEN: UINTN = 0x20;
pub const EFI_BACKGROUND_CYAN: UINTN = 0x30;
pub const EFI_BACKGROUND_RED: UINTN = 0x40;
pub const EFI_BACKGROUND_MAGENTA: UINTN = 0x50;
pub const EFI_BACKGROUND_BROWN: UINTN = 0x60;
pub const EFI_BACKGROUND_LIGHTGRAY: UINTN = 0x70;

/// This protocol is used to control text-based output devices.
///
/// The `SIMPLE_TEXT_OUTPUT` protocol is used to control text-based output devices. It is the minimum
/// required protocol for any handle supplied as the `ConsoleOut` or `StandardError` device. In addition,
/// the minimum supported text mode of such devices is at least `80x25` characters.
///
/// A video device that only supports graphics mode is required to emulate text mode functionality.
/// Output strings themselves are not allowed to contain any control codes other than those defined
/// in **Supported Unicode Control Characters**. Positional cursor placement is done only via the
/// `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.SetCursorPosition()` function. It is highly recommended that
/// text output to the `StandardError` device be limited to sequential string outputs. (That is, it is
/// not recommended to use the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.ClearScreen()` or `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.SetCursorPosition()`
/// functions on output messages to `StandardError`.)
///
/// If the output device is not in a valid text mode at the time of the `EFI_BOOT_SERVICES.HandleProtocol()`
/// call, the device is to indicate that its `CurrentMode` is `-1`. On connecting to the output device
/// the caller is required to verify the mode of the output device, and whether it is acceptable to set
/// it to something it can use.
#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    /// Resets the text output device hardware.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This`                 | A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` instance.                                               |
    /// | **IN** `ExtendedVerification` | Indicates that the driver may perform a more exhaustive verification operation of the device during reset. |
    ///
    /// ## Description
    ///
    /// The `Reset()` function resets the text output device hardware. The cursor position is set to
    /// `(0, 0)`, and the screen is cleared to the default background color for the output device.
    ///
    /// As part of initialization process, the firmware/device will make a quick but reasonable attempt
    /// to verify that the device is functioning. If the `ExtendedVerification` flag is `TRUE` the firmware
    /// may take an extended amount of time to verify the device is operating on reset. Otherwise the
    /// reset operation is to occur as quickly as possible.
    ///
    /// The hardware verification process is not defined by this specification and is left up to the
    /// platform firmware or driver to implement.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                                 |
    /// | ------------------ | --------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`      | The text output device was reset.                                           |
    /// | `EFI_DEVICE_ERROR` | The text output device is not functioning correctly and could not be reset. |
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
    /// Writes a string to the output device.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                         |
    /// | --------------- | ------------------------------------------------------------------- |
    /// | **IN** `This`   | A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` instance.        |
    /// | **IN** `String` | The null-terminated string to be displayed on the output device(s). |
    ///
    /// ## Description
    ///
    /// The `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.OutputString()` function writes a string to the output
    /// device. This is the most basic output mechanism on an output device. The `String` is displayed
    /// at the current cursor location on the output device(s) and the cursor is advanced according
    /// to the rules listed below.
    ///
    /// | Mnemonic | Unicode  | Description                                                                                                                                                                                                                                                                                                                                                                                     |
    /// | -------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `NULL`   | `U+0000` | Ignore the character, and do not move the cursor.                                                                                                                                                                                                                                                                                                                                               |
    /// | `BS`     | `U+0008` | If the cursor is not at the left edge of the display, then move the cursor left one column.                                                                                                                                                                                                                                                                                                     |
    /// | `LF`     | `U+000A` | If the cursor is at the bottom of the display, then scroll the display one row, and do not update the cursor position. Otherwise, move the cursor down one row.                                                                                                                                                                                                                                 |
    /// | `CR`     | `U+000D` | Move the cursor to the beginning of the current row.                                                                                                                                                                                                                                                                                                                                            |
    /// | Other    | `U+XXXX` | Print the character at the current cursor position and move the cursor right one column. If this moves the cursor past the right edge of the display, then the line should wrap to the beginning of the next line. This is equivalent to inserting a `CR` and an `LF`. Note that if the cursor is at the bottom of the display, and the line wraps, then the display will be scrolled one line. |
    ///
    /// ### Note
    ///
    /// If desired, the system’s NVRAM environment variables may be used at install time to determine
    /// the configured locale of the system or the installation procedure can query the user for the
    /// proper language support. This is then used to either install the proper EFI image or loader or
    /// to configure the installed image’s strings to use the proper text for the selected locale.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code              | Description                                                                                                   |
    /// | ------------------------ | ------------------------------------------------------------------------------------------------------------  |
    /// | `EFI_SUCCESS`            | The string was output to the device.                                                                          |
    /// | `EFI_DEVICE_ERROR`       | The device reported an error while attempting to output the text.                                             |
    /// | `EFI_UNSUPPORTED`        | The output device’s mode is not currently in a defined text mode.                                             |
    /// | `EFI_WARN_UNKNOWN_GLYPH` | This warning code indicates that some of the characters in the string could not be rendered and were skipped. |
    pub OutputString: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        String: *mut CHAR16,
    ) -> EFI_STATUS,
    /// Verifies that all characters in a string can be output to the target device.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                         |
    /// | --------------- | ------------------------------------------------------------------- |
    /// | **IN** `This`   | A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` instance.        |
    /// | **IN** `String` | The null-terminated string to be examined for the output device(s). |
    ///
    /// ## Description
    ///
    /// The `TestString()` function verifies that all characters in a string can be output to the target
    /// device.
    ///
    /// This function provides a way to know if the desired character codes are supported for rendering
    /// on the output device(s). This allows the installation procedure (or EFI image) to at least select
    /// character codes that the output devices are capable of displaying. Since the output device(s)
    /// may be changed between boots, if the loader cannot adapt to such changes it is recommended that
    /// the loader call `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.OutputString()` with the text it has and
    /// ignore any "unsupported" error codes. Devices that are capable of displaying the Unicode character
    /// codes will do so.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code       | Description                                                                                                            |
    /// | ----------------- | ---------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`     | The device(s) are capable of rendering the output string.                                                              |
    /// | `EFI_UNSUPPORTED` | Some of the characters in the string cannot be rendered by one or more of the output devices mapped by the EFI handle. |
    pub TestString: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        String: *mut CHAR16,
    ) -> EFI_STATUS,
    /// Returns information for an available text mode that the output device(s) supports.
    ///
    /// ## Parameters
    ///
    /// | Parameter                 | Description                                                                    |
    /// | ------------------------- | ------------------------------------------------------------------------------ |
    /// | **IN** `This`             | A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` instance.                   |
    /// | **IN** `ModeNumber`       | The mode number to return information on.                                      |
    /// | **OUT** `Columns`, `Rows` | Returns the geometry of the text output device for the requested `ModeNumber`. |
    ///
    /// ## Description
    ///
    /// The `QueryMode()` function returns information for an available text mode that the output device(s)
    /// supports.
    ///
    /// It is required that all output devices support at least `80x25` text mode. This mode is defined
    /// to be mode `0`. If the output devices support `80x50`, that is defined to be mode `1`. All other
    /// text dimensions supported by the device will follow as modes `2` and above. If an output device
    /// supports modes `2` and above, but does not support `80x50`, then querying for mode `1` will return
    /// `EFI_UNSUPPORTED`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                 |
    /// | ------------------ | ----------------------------------------------------------- |
    /// | `EFI_SUCCESS`      | The requested mode information was returned.                |
    /// | `EFI_DEVICE_ERROR` | The device had an error and could not complete the request. |
    /// | `EFI_UNSUPPORTED`  | The mode number was not valid.                              |
    pub QueryMode: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        ModeNumber: UINTN,
        Columns: *mut UINTN,
        Rows: *mut UINTN,
    ) -> EFI_STATUS,
    /// Sets the output device(s) to a specified mode.
    ///
    /// ## Parameters
    ///
    /// | Parameter           | Description                                                  |
    /// | ------------------- | ------------------------------------------------------------ |
    /// | **IN** `This`       | A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` instance. |
    /// | **IN** `ModeNumber` | The text mode to set.                                        |
    ///
    /// ## Description
    ///
    /// The `SetMode()` function sets the output device(s) to the requested mode. On success the device
    /// is in the geometry for the requested mode, and the device has been cleared to the current
    /// background color with the cursor at `(0, 0)`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                 |
    /// | ------------------ | ----------------------------------------------------------- |
    /// | `EFI_SUCCESS`      | The requested text mode was set.                            |
    /// | `EFI_DEVICE_ERROR` | The device had an error and could not complete the request. |
    /// | `EFI_UNSUPPORTED`  | The mode number was not valid.                              |
    pub SetMode: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        ModeNumber: UINTN,
    ) -> EFI_STATUS,
    /// Sets the background and foreground colors for the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.OutputString()`
    /// and `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.ClearScreen()` functions.
    ///
    /// ## Parameters
    ///
    /// | Parameter          | Description                                                                                                     |
    /// | ------------------ | --------------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This`      | A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` instance.                                                    |
    /// | **IN** `Attribute` | The attribute to set. Bits \[`0`, `3`] are the foreground color, and bits \[`4`, `6`] are the background color. |
    ///
    /// ## Description
    ///
    /// The `SetAttribute()` function sets the background and foreground colors for the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.OutputString()`
    /// and `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.ClearScreen()` functions.
    ///
    /// The color mask can be set even when the device is in an invalid text mode.
    ///
    /// Devices supporting a different number of text colors are required to emulate the above colors
    /// to the best of the device’s capabilities.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                 |
    /// | ------------------ | ----------------------------------------------------------- |
    /// | `EFI_SUCCESS`      | The requested attributes were set.                          |
    /// | `EFI_DEVICE_ERROR` | The device had an error and could not complete the request. |
    pub SetAttribute: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        Attribute: UINTN,
    ) -> EFI_STATUS,
    /// Clears the output device(s) display to the currently selected background color.
    ///
    /// ## Parameters
    ///
    /// | Parameter     | Description                                                  |
    /// | ------------- | ------------------------------------------------------------ |
    /// | **IN** `This` | A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` instance. |
    ///
    /// ## Description
    ///
    /// The `ClearScreen()` function clears the output device(s) display to the currently selected
    /// background color. The cursor position is set to `(0, 0)`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                 |
    /// | ------------------ | ----------------------------------------------------------- |
    /// | `EFI_SUCCESS`      | The operation completed successfully.                       |
    /// | `EFI_DEVICE_ERROR` | The device had an error and could not complete the request. |
    /// | `EFI_UNSUPPORTED`  | The output device is not in a valid text mode.              |
    pub ClearScreen:
        unsafe extern "efiapi" fn(This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL) -> EFI_STATUS,
    /// Sets the current coordinates of the cursor position.
    ///
    /// ## Parameters
    ///
    /// | Parameter              | Description                                                  |
    /// | ---------------------- | ------------------------------------------------------------ |
    /// | **IN** `This`          | A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` instance. |
    /// | **IN** `Column`, `Row` | The position to set the cursor to.                           |
    ///
    /// ## Description
    ///
    /// The `SetCursorPosition()` function sets the current coordinates of the cursor position. The
    /// upper left corner of the screen is defined as coordinate `(0, 0)`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                                                            |
    /// | ------------------ | ------------------------------------------------------------------------------------------------------ |
    /// | `EFI_SUCCESS`      | The operation completed successfully.                                                                  |
    /// | `EFI_DEVICE_ERROR` | The device had an error and could not complete the request.                                            |
    /// | `EFI_UNSUPPORTED`  | The output device is not in a valid text mode, or the cursor position is invalid for the current mode. |
    pub SetCursorPosition: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        Column: UINTN,
        Row: UINTN,
    ),
    /// Makes the cursor visible or invisible.
    ///
    /// ## Parameters
    ///
    /// | Parameter        | Description                                                                                |
    /// | ---------------- | ------------------------------------------------------------------------------------------ |
    /// | **IN** `This`    | A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` instance.                               |
    /// | **IN** `Visible` | If `TRUE`, the cursor is set to be visible. If `FALSE`, the cursor is set to be invisible. |
    ///
    /// ## Description
    ///
    /// The `EnableCursor()` function makes the cursor visible or invisible.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                          |
    /// | ------------------ | -------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`      | The operation completed successfully.                                |
    /// | `EFI_DEVICE_ERROR` | The device had an error and could not complete the request.          |
    /// | `EFI_UNSUPPORTED`  | The output device does not support visibility control of the cursor. |
    pub EnableCursor:
        unsafe extern "efiapi" fn(This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL, Visible: BOOLEAN),
    /// Pointer to the `SIMPLE_TEXT_OUTPUT_MODE` data.
    pub Mode: *mut SIMPLE_TEXT_OUTPUT_MODE,
}

/// Information about the current output mode.
#[repr(C)]
pub struct SIMPLE_TEXT_OUTPUT_MODE {
    /// The number of modes supported by `QueryMode()` and `SetMode()`.
    pub MaxMode: INT32,
    /// The text mode of the output device(s).
    pub Mode: INT32,
    /// The current character output attribute.
    pub Attribute: INT32,
    /// The cursor’s column.
    pub CursorColumn: INT32,
    /// The cursor’s row.
    pub CursorRow: INT32,
    /// The cursor is currently visible or not.
    pub CursorVisible: INT32,
}
