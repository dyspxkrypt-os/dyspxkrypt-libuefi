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

use crate::tables::EFI_TABLE_HEADER;
use crate::tables::system::EFI_SPECIFICATION_VERSION;
use crate::types::{BOOLEAN, EFI_STATUS, INT16, UINT16, UINT32, UINT64, UINT8};

pub const EFI_RUNTIME_SERVICES_SIGNATURE: UINT64 = 0x56524553544E5552;
pub const EFI_RUNTIME_SERVICES_REVISION: UINT32 = EFI_SPECIFICATION_VERSION;

/// The EFI Runtime Services containing a table header and pointers to all of the runtime services.
#[repr(C)]
pub struct EFI_RUNTIME_SERVICES {
    /// The table header for the EFI Runtime Services Table. This header contains the `EFI_RUNTIME_SERVICES_SIGNATURE`
    /// and `EFI_RUNTIME_SERVICES_REVISION` values along with the size of the `EFI_RUNTIME_SERVICES`
    /// structure and a 32-bit CRC to verify that the contents of the EFI Runtime Services Table are
    /// valid.
    pub Hdr: EFI_TABLE_HEADER,
    /// Returns the current time and date, and the time-keeping capabilities of the platform.
    ///
    /// ## Parameters
    ///
    /// | Parameter                           | Description                                                                           |
    /// | ----------------------------------- | ------------------------------------------------------------------------------------- |
    /// | **OUT** `Time`                      | A pointer to storage to receive a snapshot of the current time.                       |
    /// | **OUT** `Capabilities` **OPTIONAL** | An optional pointer to a buffer to receive the real time clock device’s capabilities. |
    ///
    /// ## Description
    ///
    /// The `GetTime()` function returns a time that was valid sometime during the call to the function.
    /// While the returned `EFI_TIME` structure contains TimeZone and Daylight savings time information,
    /// the actual clock does not maintain these values. The current time zone and daylight saving time
    /// information returned by `GetTime()` are the values that were last set via `SetTime()`.
    ///
    /// The `GetTime()` function should take approximately the same amount of time to read the time
    /// each time it is called. All reported device capabilities are to be rounded up.
    ///
    /// During runtime, if a PC-AT CMOS device is present in the platform the caller must synchronize
    /// access to the device before calling `GetTime()`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`           | The operation completed successfully.                                                                                                                                                                       |
    /// | `EFI_INVALID_PARAMETER` | `Time` is `NULL`.                                                                                                                                                                                           |
    /// | `EFI_DEVICE_ERROR`      | The time could not be retrieved due to a hardware error.                                                                                                                                                    |
    /// | `EFI_UNSUPPORTED`       | This call is not supported by this platform at the time the call is made. The platform should describe this runtime service as unsupported at runtime via an `EFI_RT_PROPERTIES_TABLE` configuration table. |
    pub GetTime: unsafe extern "efiapi" fn(
        Time: *mut EFI_TIME,
        Capabilities: *mut EFI_TIME_CAPABILITIES
    ) -> EFI_STATUS,
    /// Sets the current local time and date information.
    ///
    /// ## Parameters
    ///
    /// | Parameter     | Description                                                                                                                                                                               |
    /// | ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | **IN** `Time` | A pointer to the current time. Full error checking is performed on the different fields of the `EFI_TIME` structure and `EFI_INVALID_PARAMETER` is returned if any field is out of range. |
    ///
    /// ## Description
    ///
    /// The `SetTime()` function sets the real time clock device to the supplied time, and records
    /// the current time zone and daylight savings time information. The `SetTime()` function is not
    /// allowed to loop based on the current time. For example, if the device does not support a hardware
    /// reset for the sub-resolution time, the code is not to implement the feature by waiting for the
    /// time to wrap.
    ///
    /// During runtime, if a PC-AT CMOS device is present in the platform the caller must synchronize
    /// access to the device before calling `SetTime()`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`           | The operation completed successfully.                                                                                                                                                                       |
    /// | `EFI_INVALID_PARAMETER` | A `Time` field is out of range.                                                                                                                                                                             |
    /// | `EFI_DEVICE_ERROR`      | The time could not be set due to a hardware error.                                                                                                                                                          |
    /// | `EFI_UNSUPPORTED`       | This call is not supported by this platform at the time the call is made. The platform should describe this runtime service as unsupported at runtime via an `EFI_RT_PROPERTIES_TABLE` configuration table. |
    pub SetTime: unsafe extern "efiapi" fn(
        Time: *mut EFI_TIME,
    ) -> EFI_STATUS,
    /// Returns the current wakeup alarm clock setting.
    ///
    /// ## Parameters
    ///
    /// | Parameter         | Description                                                            |
    /// | ----------------- | ---------------------------------------------------------------------- |
    /// | **OUT** `Enabled` | Indicates if the alarm is currently enabled or disabled.               |
    /// | **OUT** `Pending` | Indicates if the alarm signal is pending and requires acknowledgement. |
    /// | **OUT** `Time`    | The current alarm setting.                                             |
    ///
    /// ## Description
    ///
    /// The alarm clock time may be rounded from the set alarm clock time to be within the resolution of the alarm clock
    /// device. The resolution of the alarm clock device is defined to be one second.
    ///
    /// During runtime, if a PC-AT CMOS device is present in the platform the caller must synchronize access to the device
    /// before calling `GetWakeupTime()`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`           | The operation completed successfully.                                                                                                                                                                       |
    /// | `EFI_INVALID_PARAMETER` | Either one of `Enabled`, `Pending` or `Time` is `NULL`.                                                                                                                                                     |
    /// | `EFI_DEVICE_ERROR`      | The wakeup time could not be retrieved due to a hardware error.                                                                                                                                             |
    /// | `EFI_UNSUPPORTED`       | This call is not supported by this platform at the time the call is made. The platform should describe this runtime service as unsupported at runtime via an `EFI_RT_PROPERTIES_TABLE` configuration table. |
    pub GetWakeupTime: unsafe extern "efiapi" fn(
        Enabled: *mut BOOLEAN,
        Pending: *mut BOOLEAN,
        Time: *mut EFI_TIME,
    ) -> EFI_STATUS,
    /// Sets the system wakeup alarm clock time.
    ///
    /// ## Parameters
    ///
    /// | Parameter                  | Description                                                                                                                              |
    /// | -------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
    /// | **IN** `Enabled`           | Indicates if the alarm is currently enabled or disabled.                                                                                 |
    /// | **IN** `Time` **OPTIONAL** | If `Enable` is `TRUE`, the time to set the wakeup alarm for. If `Enable` is `FALSE`, then this parameter is optional, and may be `NULL`. |
    ///
    /// ## Description
    ///
    /// Setting a system wakeup alarm causes the system to wake up or power on at the set time. When the alarm fires, the
    /// alarm signal is latched until it is acknowledged by calling `SetWakeupTime()` to disable the alarm. If the alarm
    /// fires before the system is put into a sleeping or off state, since the alarm signal is latched the system will
    /// immediately wake up. If the alarm fires while the system is off and there is insufficient power to power on the
    /// system, the system is powered on when power is restored.
    ///
    /// For an ACPI-aware operating system, this function only handles programming the wakeup alarm for the desired wakeup
    /// time. The operating system still controls the wakeup event as it normally would through the ACPI Power Management
    /// register set.
    ///
    /// The resolution for the wakeup alarm is defined to be 1 second.
    ///
    /// During runtime, if a PC-AT CMOS device is present in the platform the caller must synchronize access to the device
    /// before calling `SetWakeupTime()`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`           | The operation completed successfully.                                                                                                                                                                       |
    /// | `EFI_INVALID_PARAMETER` | A `Time` field is out of range.                                                                                                                                                                             |
    /// | `EFI_DEVICE_ERROR`      | The wakeup time could not be set due to a hardware error.                                                                                                                                                   |
    /// | `EFI_UNSUPPORTED`       | This call is not supported by this platform at the time the call is made. The platform should describe this runtime service as unsupported at runtime via an `EFI_RT_PROPERTIES_TABLE` configuration table. |
    pub SetWakeupTime: unsafe extern "efiapi" fn(
        Enable: BOOLEAN,
        Time: *mut EFI_TIME,
    ) -> EFI_STATUS,
}

/// A snapshot of the current time.
#[repr(C)]
pub struct EFI_TIME {
    /// The year of the current local date.
    pub Year: UINT16,
    /// The month of the current local date.
    pub Month: UINT8,
    /// The day of the current local date.
    pub Day: UINT8,
    /// The hour of the local time.
    pub Hour: UINT8,
    /// The minute of the local time.
    pub Minute: UINT8,
    /// The second of the local time.
    pub Second: UINT8,
    Pad1: UINT8,
    /// The nanosecond of the local time.
    pub Nanosecond: UINT32,
    /// The time’s offset in minutes from UTC. If the value is `EFI_UNSPECIFIED_TIMEZONE`, then the
    /// time is interpreted as a local time. The `TimeZone` is the number of minutes that the local
    /// time is relative to UTC. To calculate the `TimeZone` value, follow this equation: `LocalTime = UTC - TimeZone`.
    ///
    /// To further illustrate this,consider the following example:
    ///
    /// PST (Pacific Standard Time is 12PM) = UTC (8PM) - 8 hours (480 minutes)
    ///
    /// In this case, the value for `TimeZone` would be `480` if the time zone is PST.
    pub TimeZone: INT16,
    /// A bitmask containing the daylight savings time information for the time.
    ///
    /// The `EFI_TIME_ADJUST_DAYLIGHT` bit indicates if the time is affected by daylight savings time
    /// or not. This value does not indicate that the time has been adjusted for daylight savings time.
    /// It indicates only that it should be adjusted when the `EFI_TIME` enters daylight savings time.
    ///
    /// If `EFI_TIME_IN_DAYLIGHT` is set, the time has been adjusted for daylight savings time.
    ///
    /// All other bits must be zero.
    ///
    /// When entering daylight saving time, if the time is affected, but hasn’t been adjusted (DST = 1),
    /// use the new calculation:
    ///
    /// - The date/time should be increased by the appropriate amount.
    ///
    /// - The `TimeZone` should be decreased by the appropriate amount (EX: +`480` changes to +`420`
    /// when moving from PST to PDT).
    ///
    /// - The `Daylight` value changes to `3`.
    ///
    /// When exiting daylight saving time, if the time is affected and has been adjusted (DST = 3),
    /// use the new calculation:
    ///
    /// - The date/time should be decreased by the appropriate amount.
    ///
    /// - The `TimeZone` should be increased by the appropriate amount.
    ///
    /// - The `Daylight` value changes to `1`.
    pub Daylight: UINT8,
    Pad2: UINT8,
}

/// A real time clock device’s capabilities.
#[repr(C)]
pub struct EFI_TIME_CAPABILITIES {
    /// Provides the reporting resolution of the real-time clock device in counts per second. For a
    /// normal PC-AT CMOS RTC device, this value would be 1 Hz, or 1, to indicate that the device
    /// only reports the time to the resolution of 1 second.
    pub Resolution: UINT32,
    /// Provides the timekeeping accuracy of the real-time clock in an error rate of `1E-6` parts per
    /// million. For a clock with an accuracy of 50 parts per million, the value in this field would
    /// be 50000000.
    pub Accuracy: UINT32,
    /// A `TRUE` indicates that a time set operation clears the device’s time below the `Resolution`
    /// reporting level. A `FALSE` indicates that the state below the Resolution level of the device
    /// is not cleared when the time is set. Normal PC-AT CMOS RTC devices set this value to `FALSE`.
    pub SetsToZero: BOOLEAN,
}
