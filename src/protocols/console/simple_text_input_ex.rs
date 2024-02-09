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

use crate::protocols::console::simple_text_input::EFI_INPUT_KEY;
use crate::types::{BOOLEAN, EFI_GUID, EFI_STATUS, UINT32, UINT8, VOID};

pub const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xDD9E7534,
        0x7762,
        0x4698,
        [0x8C, 0x14, 0xF5, 0x85, 0x17, 0xA6, 0x25, 0xAA],
    )
};

pub const EFI_SHIFT_STATE_VALID: UINT32 = 0x80000000;
pub const EFI_RIGHT_SHIFT_PRESSED: UINT32 = 0x00000001;
pub const EFI_LEFT_SHIFT_PRESSED: UINT32 = 0x00000002;
pub const EFI_RIGHT_CONTROL_PRESSED: UINT32 = 0x00000004;
pub const EFI_LEFT_CONTROL_PRESSED: UINT32 = 0x00000008;
pub const EFI_RIGHT_ALT_PRESSED: UINT32 = 0x00000010;
pub const EFI_LEFT_ALT_PRESSED: UINT32 = 0x00000020;
pub const EFI_RIGHT_LOGO_PRESSED: UINT32 = 0x00000040;
pub const EFI_LEFT_LOGO_PRESSED: UINT32 = 0x00000080;
pub const EFI_MENU_KEY_PRESSED: UINT32 = 0x00000100;
pub const EFI_SYS_REQ_PRESSED: UINT32 = 0x00000200;

pub const EFI_TOGGLE_STATE_VALID: UINT8 = 0x80;
pub const EFI_KEY_STATE_EXPOSED: UINT8 = 0x40;
pub const EFI_SCROLL_LOCK_ACTIVE: UINT8 = 0x01;
pub const EFI_NUM_LOCK_ACTIVE: UINT8 = 0x02;
pub const EFI_CAPS_LOCK_ACTIVE: UINT8 = 0x04;

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL {
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,

    pub ReadKeyStrokeEx: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
        KeyData: *mut EFI_KEY_DATA,
    ),

    pub SetState: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
        KeyToggleState: *mut EFI_KEY_TOGGLE_STATE,
    ) -> EFI_STATUS,

    pub RegisterKeyNotify: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
        KeyData: *mut EFI_KEY_DATA,
        KeyNotificationFunction: EFI_KEY_NOTIFY_FUNCTION,
        NotifyHandle: *mut *mut VOID,
    ) -> EFI_STATUS,

    pub UnregisterKeyNotify: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
        NotificationHandle: *mut VOID,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_KEY_DATA {
    pub Key: EFI_INPUT_KEY,

    pub KeyState: EFI_KEY_STATE,
}

#[repr(C)]
pub struct EFI_KEY_STATE {
    pub KeyShiftState: UINT32,

    pub KeyToggleState: EFI_KEY_TOGGLE_STATE,
}

pub type EFI_KEY_TOGGLE_STATE = UINT8;

pub type EFI_KEY_NOTIFY_FUNCTION =
    unsafe extern "efiapi" fn(KeyData: *mut EFI_KEY_DATA) -> EFI_STATUS;
