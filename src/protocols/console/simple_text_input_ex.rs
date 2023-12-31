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

/// This protocol is used to obtain input from the `ConsoleIn` device. The EFI specification requires
/// that the `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` supports the same languages as the corresponding
/// `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
///
/// The `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL` is used on the `ConsoleIn` device. It is an extension to the Simple Text Input
/// protocol which allows a variety of extended shift state information to be returned.
#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL {
    /// Resets the input device hardware.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This`                 | A pointer to the `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL` instance.                                             |
    /// | **IN** `ExtendedVerification` | Indicates that the driver may perform a more exhaustive verification operation of the device during reset. |
    ///
    /// ## Description
    ///
    /// The `Reset()` function resets the input device hardware.
    ///
    /// The implementation of `Reset` is required to clear the contents of any input queues resident
    /// in memory used for buffering keystroke data and put the input stream in a known empty state.
    ///
    /// As part of initialization process, the firmware/device will make a quick but reasonable attempt
    /// to verify that the device is functioning. If the `ExtendedVerification` flag is `TRUE` the
    /// firmware may take an extended amount of time to verify the device is operating on reset.
    /// Otherwise the reset operation is to occur as quickly as possible.
    ///
    /// The hardware verification process is not defined by this specification and is left up to the
    /// platform firmware or driver to implement.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS`      | The device was reset.                                           |
    /// | `EFI_DEVICE_ERROR` | The device is not functioning correctly and could not be reset. |
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
    /// Reads the next keystroke from the input device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This`                 | A pointer to the `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL` instance.                                             |
    /// | **OUT** `KeyData` | A pointer to a buffer that is filled in with the keystroke state data for the key that was pressed. |
    ///
    /// ## Description
    ///
    /// The `ReadKeyStrokeEx()` function reads the next keystroke from the input device. If there is
    /// no pending keystroke the function returns `EFI_NOT_READY`. If there is a pending keystroke,
    /// then `KeyData.Key.ScanCode` is the EFI scan code defined in EFI Scan Codes for `EFI_SIMPLE_TEXT_INPUT_PROTOCOL`.
    /// The `KeyData.Key.UnicodeChar` is the actual printable character or is zero if the key does
    /// not represent a printable character (control key, function key, etc.). The `KeyData.KeyState`
    /// is the modifier shift state for the character reflected in `KeyData.Key.UnicodeChar` or
    /// `KeyData.Key.ScanCode`. This function mirrors the behavior of `ReadKeyStroke()` in the Simple
    /// Input Protocol in that a keystroke will only be returned when `KeyData.Key` has data within it.
    ///
    /// When interpreting the data from this function, it should be noted that if a class of printable
    /// characters that are normally adjusted by shift modifiers (e.g. Shift Key + "F" key) would be
    /// presented solely as a `KeyData.Key.UnicodeChar` without the associated shift state. So in the
    /// previous example of a Shift Key + "F" key being pressed, the only pertinent data returned
    /// would be `KeyData.Key.UnicodeChar` with the value of "F". This of course would not typically
    /// be the case for non-printable characters such as the pressing of the Right Shift Key + F10 key
    /// since the corresponding returned data would be reflected both in the `KeyData.KeyState.KeyShiftState`
    /// and `KeyData.Key.ScanCode` values.
    ///
    /// UEFI drivers which implement the `EFI_SIMPLE_TEXT_INPUT_EX` protocol are required to return
    /// `KeyData.Key` and `KeyData.KeyState values`. These drivers must always return the most current
    /// state of `KeyData.KeyState.KeyShiftState` and `KeyData.KeyState.KeyToggleState`. It should also be noted that certain input devices may not be able to produce shift or toggle state information, and in those cases the high order bit in the respective Toggle and Shift state fields should not be active.
    ///
    /// If the `EFI_KEY_STATE_EXPOSED` bit is turned on, then this instance of the `EFI_SIMPLE_INPUT_EX_PROTOCOL`
    /// supports the ability to return partial keystrokes. With `EFI_KEY_STATE_EXPOSED` bit enabled,
    /// the `ReadKeyStrokeEx()` function will allow the return of incomplete keystrokes such as the
    /// holding down of certain keys which are expressed as a part of `KeyState` when there is no
    /// `Key` data.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The keystroke information was returned. |
    /// | `EFI_NOT_READY` | There was no keystroke data available. Current `KeyData.KeyState` values are exposed. |
    /// | `EFI_DEVICE_ERROR` | The keystroke information was not returned due to hardware errors. |
    /// | `EFI_UNSUPPORTED` | The device does not support the ability to read keystroke data. |
    pub ReadKeyStrokeEx: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
        KeyData: *mut EFI_KEY_DATA,
    ),
    /// Set certain state for the input device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This`                 | A pointer to the `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL` instance.                                             |
    /// | **IN** `KeyToggleState` | Pointer to the `EFI_KEY_TOGGLE_STATE` to set the state for the input device. |
    ///
    /// # Description
    ///
    /// The `SetState()` function allows the input device hardware to have state settings adjusted.
    /// By calling the `SetState()` function with the `EFI_KEY_STATE_EXPOSED` bit active in the
    /// `KeyToggleState` parameter, this will enable the `ReadKeyStrokeEx()` function to return
    /// incomplete keystrokes such as the holding down of certain keys which are expressed as a part
    /// of `KeyState` when there is no `Key` data.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The device state was set appropriately. |
    /// | `EFI_DEVICE_ERROR` | The device is not functioning correctly and could not have the setting adjusted. |
    /// | `EFI_UNSUPPORTED` | The device does not support the ability to have its state set or the requested state change was not supported. |
    pub SetState: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
        KeyToggleState: *mut EFI_KEY_TOGGLE_STATE,
    ) -> EFI_STATUS,
    /// Register a notification function for a particular keystroke for the input device.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL` instance. |
    /// | **IN** `KeyData` | A pointer to a buffer that is filled in with the keystroke information for the key that was pressed. If `KeyData.Key`, `KeyData.KeyState.KeyToggleState` and `KeyData.KeyState.KeyShiftState` are `0`, then any incomplete keystroke will trigger a notification of the `KeyNotificationFunction`. |
    /// | **IN** `KeyNotificationFunction` | Points to the function to be called when the key sequence is typed specified by `KeyData`. This notification function should be called at `<= TPL_CALLBACK`. |
    /// | **OUT** `NotifyHandle` | Points to the unique handle assigned to the registered notification. |
    ///
    /// ## Description
    ///
    /// The `RegisterKeystrokeNotify()` function registers a function which will be called when a
    /// specified keystroke will occur. The keystroke being specified can be for any combination of
    /// `KeyData.Key` or `KeyData.KeyState` information.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | Key notify was registered successfully. |
    /// | `EFI_OUT_OF_RESOURCES` | Unable to allocate necessary data structures. |
    pub RegisterKeyNotify: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
        KeyData: *mut EFI_KEY_DATA,
        KeyNotificationFunction: EFI_KEY_NOTIFY_FUNCTION,
        NotifyHandle: *mut *mut VOID,
    ) -> EFI_STATUS,
    /// Remove the notification that was previously registered.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL` instance. |
    /// | **IN** `NotificationHandle` | The handle of the notification function being unregistered. |
    ///
    /// ## Description
    ///
    /// The `UnregisterKeystrokeNotify()` function removes the notification which was previously
    /// registered.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | Key notify was unregistered successfully. |
    /// | `EFI_INVALID_PARAMETER` | The `NotificationHandle` is invalid. |
    pub UnregisterKeyNotify: unsafe extern "efiapi" fn(
        This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
        NotificationHandle: *mut VOID,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_KEY_DATA {
    /// The EFI scan code and Unicode value returned from the input device.
    pub Key: EFI_INPUT_KEY,
    /// The current state of various toggled attributes as well as input modifier values.
    pub KeyState: EFI_KEY_STATE,
}

#[repr(C)]
pub struct EFI_KEY_STATE {
    /// Reflects the currently pressed shift modifiers for the input device. The returned value is
    /// valid only if the high order bit has been set.
    pub KeyShiftState: UINT32,
    /// Reflects the current internal state of various toggled attributes. The returned value is
    /// valid only if the high order bit has been set.
    pub KeyToggleState: EFI_KEY_TOGGLE_STATE,
}

pub type EFI_KEY_TOGGLE_STATE = UINT8;

pub type EFI_KEY_NOTIFY_FUNCTION =
    unsafe extern "efiapi" fn(KeyData: *mut EFI_KEY_DATA) -> EFI_STATUS;
