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

use crate::tables::boot::EFI_MEMORY_DESCRIPTOR;
use crate::tables::system::EFI_SPECIFICATION_VERSION;
use crate::tables::EFI_TABLE_HEADER;
use crate::types::{BOOLEAN, EFI_STATUS, INT16, UINT16, UINT32, UINT64, UINT8, UINTN, VOID};

pub const EFI_RUNTIME_SERVICES_SIGNATURE: UINT64 = 0x56524553544E5552;
pub const EFI_RUNTIME_SERVICES_REVISION: UINT32 = EFI_SPECIFICATION_VERSION;

pub const EFI_OPTIONAL_PTR: UINTN = 0x00000001;

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
        Capabilities: *mut EFI_TIME_CAPABILITIES,
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
    pub SetTime: unsafe extern "efiapi" fn(Time: *mut EFI_TIME) -> EFI_STATUS,
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
    pub SetWakeupTime:
        unsafe extern "efiapi" fn(Enable: BOOLEAN, Time: *mut EFI_TIME) -> EFI_STATUS,
    /// Changes the runtime addressing mode of EFI firmware from physical to virtual.
    ///
    /// ## Parameters
    ///
    /// | Parameter                  | Description                                                                                                  |
    /// | -------------------------- | ------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `MemoryMapSize`     | The size in bytes of VirtualMap.                                                                             |
    /// | **IN** `DescriptorSize`    | The size in bytes of an entry in the `VirtualMap`.                                                           |
    /// | **IN** `DescriptorVersion` | The version of the structure entries in `VirtualMap`.                                                        |
    /// | **IN** `VirtualMap`        | An array of memory descriptors which contain new virtual address mapping information for all runtime ranges. |
    ///
    /// ## Description
    ///
    /// The `SetVirtualAddressMap()` function is used by the OS loader. The function can only be called at runtime, and
    /// is called by the owner of the system’s memory map: i.e., the component which called `EFI_BOOT_SERVICES.ExitBootServices()`.
    /// All events of type `EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE` must be signaled before `SetVirtualAddressMap()` returns.
    ///
    /// This call changes the addresses of the runtime components of the EFI firmware to the new virtual addresses supplied
    /// in the `VirtualMap`. The supplied `VirtualMap` must provide a new virtual address for every entry in the memory
    /// map at `ExitBootServices()` that is marked as being needed for runtime usage. All of the virtual address fields
    /// in the `VirtualMap` must be aligned on 4 KiB boundaries.
    ///
    /// The call to `SetVirtualAddressMap()` must be done with the physical mappings. On successful return from this function,
    /// the system must then make any future calls with the newly assigned virtual mappings. All address space mappings
    /// must be done in accordance to the cache-ability flags as specified in the original address map.
    ///
    /// When this function is called, all events that were registered to be signaled on an address map change are notified.
    /// Each component that is notified must update any internal pointers for their new addresses. This can be done with
    /// the `ConvertPointer()` function. Once all events have been notified, the EFI firmware reapplies image “fix-up”
    /// information to virtually relocate all runtime images to their new addresses. In addition, all of the fields of
    /// the EFI Runtime Services Table except `SetVirtualAddressMap` and `ConvertPointer` must be converted from physical
    /// pointers to virtual pointers using the `ConvertPointer()` service. The `SetVirtualAddressMap()` and `ConvertPointer()`
    /// services are only callable in physical mode, so they do not need to be converted from physical pointers to virtual
    /// pointers. Several fields of the EFI System Table must be converted from physical pointers to virtual pointers using
    /// the `ConvertPointer()` service. These fields include `FirmwareVendor`, `RuntimeServices`, and `ConfigurationTable`.
    /// Because contents of both the EFI Runtime Services Table and the EFI System Table are modified by this service, the
    /// 32-bit CRC for the EFI Runtime Services Table and the EFI System Table must be recomputed.
    ///
    /// A virtual address map may only be applied one time. Once the runtime system is in virtual mode, calls to this
    /// function return `EFI_UNSUPPORTED`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`           | The virtual address map has been applied.                                                                                                                                                                   |
    /// | `EFI_UNSUPPORTED`       | EFI firmware is not at runtime, or the EFI firmware is already in virtual address mapped mode.                                                                                                              |
    /// | `EFI_INVALID_PARAMETER` | `DescriptorSize` or `DescriptorVersion` is invalid.                                                                                                                                                         |
    /// | `EFI_NO_MAPPING`        | A virtual address was not supplied for a range in the memory map that requires a mapping.                                                                                                                   |
    /// | `EFI_NOT_FOUND`         | A virtual address was supplied for an address that is not found in the memory map.                                                                                                                          |
    /// | `EFI_UNSUPPORTED`       | This call is not supported by this platform at the time the call is made. The platform should describe this runtime service as unsupported at runtime via an `EFI_RT_PROPERTIES_TABLE` configuration table. |
    pub SetVirtualAddressMap: unsafe extern "efiapi" fn(
        MemoryMapSize: UINTN,
        DescriptorSize: UINTN,
        DescriptorVersion: UINT32,
        VirtualMap: *mut EFI_MEMORY_DESCRIPTOR,
    ) -> EFI_STATUS,
    /// Determines the new virtual address that is to be used on subsequent memory accesses.
    ///
    /// ## Parameters
    ///
    /// | Parameter                 | Description                                                                                                           |
    /// | ------------------------- | --------------------------------------------------------------------------------------------------------------------- |
    /// | **IN** `DebugDisposition` | Supplies type information for the pointer being converted.                                                            |
    /// | **IN** `Address`          | A pointer to a pointer that is to be fixed to be the value needed for the new virtual address mappings being applied. |
    ///
    /// ## Description
    ///
    /// The `ConvertPointer()` function is used by an EFI component during the `SetVirtualAddressMap()` operation. `ConvertPointer()`
    /// must be called using physical address pointers during the execution of `SetVirtualAddressMap()`.
    ///
    /// The `ConvertPointer()` function updates the current pointer pointed to by Address to be the proper value for the
    /// new address map. Only runtime components need to perform this operation. The `EFI_BOOT_SERVICES.CreateEvent()`
    /// function is used to create an event that is to be notified when the address map is changing. All pointers the
    /// component has allocated or assigned must be updated.
    ///
    /// If the `EFI_OPTIONAL_PTR` flag is specified, the pointer being converted is allowed to be `NULL`.
    ///
    /// Once all components have been notified of the address map change, firmware fixes any compiled in pointers that are
    /// embedded in any runtime image.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`           | The pointer pointed to by `Address` was modified.                                                                                                                                                           |
    /// | `EFI_NOT_FOUND`         | The pointer pointed to by `Address` was not found to be part of the current memory map. This is normally fatal.                                                                                             |
    /// | `EFI_INVALID_PARAMETER` | `Address` is `NULL`.                                                                                                                                                                                        |
    /// | `EFI_INVALID_PARAMETER` | `*Address` is `NULL` and `DebugDisposition` does not have the `EFI_OPTIONAL_PTR` bit set.                                                                                                                   |
    /// | `EFI_UNSUPPORTED`       | This call is not supported by this platform at the time the call is made. The platform should describe this runtime service as unsupported at runtime via an `EFI_RT_PROPERTIES_TABLE` configuration table. |
    pub ConvertPointer: unsafe extern "efiapi" fn(
        DebugDisposition: UINTN,
        Address: *mut *mut VOID,
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
