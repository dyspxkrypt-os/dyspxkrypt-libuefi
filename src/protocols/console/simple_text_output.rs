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

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
    pub OutputString: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        String: *mut CHAR16,
    ) -> EFI_STATUS,
    pub TestString: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        String: *mut CHAR16,
    ) -> EFI_STATUS,
    pub QueryMode: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        ModeNumber: UINTN,
        Columns: *mut UINTN,
        Rows: *mut UINTN,
    ) -> EFI_STATUS,
    pub SetMode: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        ModeNumber: UINTN,
    ) -> EFI_STATUS,
    pub SetAttribute: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        Attribute: UINTN,
    ) -> EFI_STATUS,
    pub ClearScreen:
        unsafe extern "efiapi" fn(This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL) -> EFI_STATUS,
    pub SetCursorPosition: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
        Column: UINTN,
        Row: UINTN,
    ),
    pub EnableCursor:
        unsafe extern "efiapi" fn(This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL, Visible: BOOLEAN),
    pub Mode: *mut SIMPLE_TEXT_OUTPUT_MODE,
}

#[repr(C)]
pub struct SIMPLE_TEXT_OUTPUT_MODE {
    pub MaxMode: INT32,
    pub Mode: INT32,
    pub Attribute: INT32,
    pub CursorColumn: INT32,
    pub CursorRow: INT32,
    pub CursorVisible: INT32,
}
