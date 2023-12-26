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

use crate::tables::boot::{EFI_MEMORY_DESCRIPTOR, EFI_PHYSICAL_ADDRESS};
use crate::tables::system::EFI_SPECIFICATION_VERSION;
use crate::tables::EFI_TABLE_HEADER;
use crate::types::{
    BOOLEAN, CHAR16, EFI_GUID, EFI_STATUS, INT16, UINT16, UINT32, UINT64, UINT8, UINTN, VOID,
};

pub const EFI_RUNTIME_SERVICES_SIGNATURE: UINT64 = 0x56524553544E5552;
pub const EFI_RUNTIME_SERVICES_REVISION: UINT32 = EFI_SPECIFICATION_VERSION;

pub const EFI_OPTIONAL_PTR: UINTN = 0x00000001;

pub const EFI_VARIABLE_NON_VOLATILE: UINT32 = 0x00000001;
pub const EFI_VARIABLE_BOOTSERVICE_ACCESS: UINT32 = 0x00000002;
pub const EFI_VARIABLE_RUNTIME_ACCESS: UINT32 = 0x00000004;
pub const EFI_VARIABLE_HARDWARE_ERROR_RECORD: UINT32 = 0x00000008;
#[deprecated(
    since = "0.1.0",
    note = "this attribute is deprecated and should be considered as reserved"
)]
pub const EFI_VARIABLE_AUTHENTICATED_WRITE_ACCESS: UINT32 = 0x00000010;
pub const EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS: UINT32 = 0x00000020;
pub const EFI_VARIABLE_APPEND_WRITE: UINT32 = 0x00000040;
pub const EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS: UINT32 = 0x00000080;

pub const CAPSULE_FLAGS_PERSIST_ACROSS_RESET: UINT32 = 0x00010000;
pub const CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE: UINT32 = 0x00020000;
pub const CAPSULE_FLAGS_INITIATE_RESET: UINT32 = 0x00040000;

#[repr(C)]
pub enum EFI_RESET_TYPE {
    EfiResetCold,
    EfiResetWarm,
    EfiResetShutdown,
    EfiResetPlatformSpecific,
}

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
    pub ConvertPointer:
        unsafe extern "efiapi" fn(DebugDisposition: UINTN, Address: *mut *mut VOID) -> EFI_STATUS,
    /// Returns the value of a variable.
    ///
    /// ## Parameters
    ///
    /// | Parameter                         | Description                                                                                                                                                                                                               |
    /// | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | **IN** `VariableName`             | A null-terminated string that is the name of the vendor’s variable.                                                                                                                                                       |
    /// | **IN** `VendorGuid`               | A unique identifier for the vendor.                                                                                                                                                                                       |
    /// | **OUT** `Attributes` **OPTIONAL** | If not `NULL`, a pointer to the memory location to return the attributes bitmask for the variable. If not `NULL`, then `Attributes` is set on output both when `EFI_SUCCESS` and when `EFI_BUFFER_TOO_SMALL` is returned. |
    /// | **IN OUT** `DataSize`             | On input, the size in bytes of the return `Data` buffer. On output the size of data returned in `Data`.                                                                                                                   |
    /// | **OUT** `Data` **OPTIONAL**       | The buffer to return the contents of the variable. May be `NULL` with a zero `DataSize` in order to determine the size buffer needed.                                                                                     |
    ///
    /// ## Description
    ///
    /// Each vendor may create and manage its own variables without the risk of name conflicts by using a unique `VendorGuid`.
    /// When a variable is set its `Attributes` are supplied to indicate how the data variable should be stored and maintained
    /// by the system. The attributes affect when the variable may be accessed and volatility of the data. If `EFI_BOOT_SERVICES.ExitBootServices()`
    /// has already been executed, data variables without the `EFI_VARIABLE_RUNTIME_ACCESS` attribute set will not be visible
    /// to `GetVariable()` and will return an `EFI_NOT_FOUND` error.
    ///
    /// If the `Data` buffer is too small to hold the contents of the variable, the error `EFI_BUFFER_TOO_SMALL` is returned
    /// and `DataSize` is set to the required buffer size to obtain the data.
    ///
    /// The `EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS` and the `EFI_VARIABLE_AUTHENTICATED_WRITE_ACCESS` attributes
    /// may both be set in the returned `Attributes` bitmask parameter of a `GetVariable()` call, though it should be noted
    /// that the `EFI_VARIABLE_AUTHENTICATED_WRITE_ACCESS` attribute is deprecated and should no longer be used. The
    /// `EFI_VARIABLE_APPEND_WRITE` attribute will never be set in the returned `Attributes` bitmask parameter.
    ///
    /// Variables stored with the `EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS` attribute set will return metadata in addition
    /// to variable data when `GetVariable()` is called. If a `GetVariable()` call indicates that this attribute is set,
    /// the `GetVariable()` payload must be interpreted according to the metadata headers. In addition to the headers described
    /// in `SetVariable()`, the `EFI_VARIABLE_AUTHENTICATION_3_CERT_ID` header is used to indicate what certificate may be
    /// currently associated with a variable.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code              | Description                                                                                                                                                                                                                                                          |
    /// | ------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`            | The function completed successfully.                                                                                                                                                                                                                                 |
    /// | `EFI_NOT_FOUND`          | The variable was not found.                                                                                                                                                                                                                                          |
    /// | `EFI_BUFFER_TOO_SMALL`   | The `DataSize` is too small for the result. `DataSize` has been updated with the size needed to complete the request. If `Attributes` is not `NULL`, then the attributes bitmask for the variable has been stored to the memory location pointed to by `Attributes`. |
    /// | `EFI_INVALID_PARAMETER`  | `VariableName` is `NULL`.                                                                                                                                                                                                                                            |
    /// | `EFI_INVALID_PARAMETER`  | `VendorGuid` is `NULL`.                                                                                                                                                                                                                                              |
    /// | `EFI_INVALID_PARAMETER`  | `DataSize` is `NULL`.                                                                                                                                                                                                                                                |
    /// | `EFI_INVALID_PARAMETER`  | The `DataSize` is not too small and `Data` is `NULL`.                                                                                                                                                                                                                |
    /// | `EFI_DEVICE_ERROR`       | The variable could not be retrieved due to a hardware error.                                                                                                                                                                                                         |
    /// | `EFI_SECURITY_VIOLATION` | The variable could not be retrieved due to an authentication failure.                                                                                                                                                                                                |
    /// | `EFI_UNSUPPORTED`        | After `ExitBootServices()` has been called, this return code may be returned if no variable storage is supported. The platform should describe this runtime service as unsupported at runtime via an `EFI_RT_PROPERTIES_TABLE` configuration table.                  |
    pub GetVariable: unsafe extern "efiapi" fn(
        VariableName: *mut CHAR16,
        VendorGuid: *mut EFI_GUID,
        Attributes: *mut UINT32,
        DataSize: *mut UINTN,
        Data: *mut VOID,
    ) -> EFI_STATUS,
    /// Enumerates the current variable names.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                                                                     |
    /// | ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | **IN OUT** `VariableNameSize` | The size of the `VariableName` buffer. The size must be large enough to fit input string supplied in `VariableName` buffer.                                     |
    /// | **IN OUT** `VariableName`     | On input, supplies the last `VariableName` that was returned by `GetNextVariableName()`. On output, returns the null-terminated string of the current variable. |
    /// | **IN OUT** `VendorGuid`       | On input, supplies the last `VendorGuid` that was returned by `GetNextVariableName()`. On output, returns the VendorGuid of the current variable.               |
    ///
    /// ## Description
    ///
    /// `GetNextVariableName()` is called multiple times to retrieve the `VariableName` and `VendorGuid` of all variables
    /// currently available in the system. On each call to `GetNextVariableName()` the previous results are passed into
    /// the interface, and on output the interface returns the next variable name data. When the entire variable list has
    /// been returned, the error `EFI_NOT_FOUND` is returned.
    ///
    /// Note that if `EFI_BUFFER_TOO_SMALL` is returned, the `VariableName` buffer was too small for the next variable.
    /// When such an error occurs, the `VariableNameSize` is updated to reflect the size of buffer needed. In all cases
    /// when calling `GetNextVariableName()` the `VariableNameSize` must not exceed the actual buffer size that was allocated
    /// for `VariableName`. The `VariableNameSize` must not be smaller the size of the variable name string passed to
    /// `GetNextVariableName()` on input in the `VariableName` buffer.
    ///
    /// To start the search, a null-terminated string is passed in `VariableName`; that is, `VariableName` is a pointer to
    /// a `NULL` character. This is always done on the initial call to `GetNextVariableName()`. When `VariableName` is
    /// a pointer to a `NULL` character, VendorGuid is ignored. `GetNextVariableName()` cannot be used as a filter to
    /// return variable names with a specific GUID. Instead, the entire list of variables must be retrieved, and the caller
    /// may act as a filter if it chooses. Calls to `SetVariable()` between calls to `GetNextVariableName()` may produce
    /// unpredictable results. If a `VariableName` buffer on input is not a null-terminated string, `EFI_INVALID_PARAMETER`
    /// is returned. If input values of `VariableName` and `VendorGuid` are not a name and GUID of an existing variable,
    /// `EFI_INVALID_PARAMETER` is returned.
    ///
    /// Once `EFI_BOOT_SERVICES.ExitBootServices()` is performed, variables that are only visible during boot services
    /// will no longer be returned. To obtain the data contents or attribute for a variable returned by `GetNextVariableName()`,
    /// the `GetVariable()` interface is used.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code              | Description                                                                                                                                                                                                                                         |
    /// | ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`            | The function completed successfully.                                                                                                                                                                                                                |
    /// | `EFI_NOT_FOUND`          | The next variable was not found.                                                                                                                                                                                                                    |
    /// | `EFI_BUFFER_TOO_SMALL`   | The `VariableNameSize` is too small for the result. `VariableNameSize` has been updated with the size needed to complete the request.                                                                                                               |
    /// | `EFI_INVALID_PARAMETER`  | `VariableNameSize` is `NULL`.                                                                                                                                                                                                                       |
    /// | `EFI_INVALID_PARAMETER`  | `VariableName` is NULL.                                                                                                                                                                                                                             |
    /// | `EFI_INVALID_PARAMETER`  | `VendorGuid` is `NULL`.                                                                                                                                                                                                                             |
    /// | `EFI_INVALID_PARAMETER`  | The input values of `VariableName` and `VendorGuid` are not a name and GUID of an existing variable.                                                                                                                                                |
    /// | `EFI_INVALID_PARAMETER`  | Null-terminator is not found in the first `VariableNameSize` bytes of the input `VariableName` buffer.                                                                                                                                              |
    /// | `EFI_DEVICE_ERROR`       | The variable name could not be retrieved due to a hardware error.                                                                                                                                                                                   |
    /// | `EFI_UNSUPPORTED`        | After `ExitBootServices()` has been called, this return code may be returned if no variable storage is supported. The platform should describe this runtime service as unsupported at runtime via an `EFI_RT_PROPERTIES_TABLE` configuration table. |
    pub GetNextVariableName: unsafe extern "efiapi" fn(
        VariableNameSize: *mut UINTN,
        VariableName: *mut CHAR16,
        VendorGuid: *mut EFI_GUID,
    ) -> EFI_STATUS,
    /// Sets the value of a variable. This service can be used to create a new variable, modify the value of an existing
    /// variable, or to delete an existing variable.
    ///
    /// ## Parameters
    ///
    /// | Parameter             | Description                                                                                                                                                                                                                                              |
    /// | --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | **IN** `VariableName` | A null-terminated string that is the name of the vendor’s variable. Each VariableName is unique for each `VendorGuid`. `VariableName` must contain 1 or more characters. If `VariableName` is an empty string, then `EFI_INVALID_PARAMETER` is returned. |
    /// | **IN** `VendorGuid`   | A unique identifier for the vendor.                                                                                                                                                                                                                      |
    /// | **IN** `Attributes`   | Attributes bitmask to set for the variable.                                                                                                                                                                                                              |
    /// | **IN** `DataSize`     | The size in bytes of the Data buffer. Unless the `EFI_VARIABLE_APPEND_WRITE`, `EFI_VARIABLE_AUTHENTICATED_WRITE_ACCESS`, `EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS`, or `EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS` attribute is set, a size of zero causes the variable to be deleted. When the `EFI_VARIABLE_APPEND_WRITE` attribute is set, then a `SetVariable()` call with a `DataSize` of zero will not cause any change to the variable value (the timestamp associated with the variable may be updated however, even if no new data value is provided; see the description of the `EFI_VARIABLE_AUTHENTICATION_2` descriptor). In this case the `DataSize` will not be zero since the `EFI_VARIABLE_AUTHENTICATION_2` descriptor will be populated). |
    /// | **IN** `Data`         | The contents for the variable.                                                                                                                                                                                                                           |
    ///
    /// ## Description
    ///
    /// Variables are stored by the firmware and may maintain their values across power cycles. Each vendor may create
    /// and manage its own variables without the risk of name conflicts by using a unique `VendorGuid`.
    ///
    /// Each variable has `Attributes` that define how the firmware stores and maintains the data value. If the `EFI_VARIABLE_NON_VOLATILE`
    /// attribute is not set, the firmware stores the variable in normal memory and it is not maintained across a power cycle.
    /// Such variables are used to pass information from one component to another. An example of this is the firmware’s
    /// language code support variable. It is created at firmware initialization time for access by EFI components that
    /// may need the information, but does not need to be backed up to nonvolatile storage.
    ///
    /// `EFI_VARIABLE_NON_VOLATILE` variables are stored in fixed hardware that has a limited storage capacity; sometimes
    /// a severely limited capacity. Software should only use a nonvolatile variable when absolutely necessary. In addition,
    /// if software uses a nonvolatile variable it should use a variable that is only accessible at boot services time if
    /// possible.
    ///
    /// A variable must contain one or more bytes of `Data`. Unless the `EFI_VARIABLE_APPEND_WRITE`, `EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS`,
    /// or `EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS` attribute is set, using `SetVariable()` with a `DataSize` of zero
    /// will cause the entire variable to be deleted. The space consumed by the deleted variable may not be available
    /// until the next power cycle.
    ///
    /// If a variable with matching name, GUID, and attributes already exists, its value is updated.
    ///
    /// The Attributes have the following usage rules:
    ///
    /// - If a preexisting variable is rewritten with different attributes, `SetVariable()` shall not modify the variable
    /// and shall return `EFI_INVALID_PARAMETER`. The only exception to this is when the only attribute differing is `EFI_VARIABLE_APPEND_WRITE`.
    /// In such cases the call’s successful outcome or not is determined by the actual value being written. There are two
    /// exceptions to this rule:
    ///
    ///   - If a preexisting variable is rewritten with no access attributes specified, the variable will be deleted.
    ///
    ///   - `EFI_VARIABLE_APPEND_WRITE` attribute presents a special case. It is acceptable to rewrite the variable
    /// with or without `EFI_VARIABLE_APPEND_WRITE` attribute.
    ///
    /// - Setting a data variable with no access attributes causes it to be deleted.
    ///
    /// - `EFI_VARIABLE_AUTHENTICATED_WRITE_ACCESS` is deprecated and should not be used. Platforms should return
    /// `EFI_UNSUPPORTED` if a caller to `SetVariable()` specifies this attribute.
    ///
    /// - Unless the `EFI_VARIABLE_APPEND_WRITE`, `EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS`, or `EFI_VARIABLE_ENHANCED_AUTHENTICATED_WRITE_ACCESS`
    /// attribute is set, setting a data variable with zero `DataSize` specified, causes it to be deleted.
    ///
    /// - Runtime access to a data variable implies boot service access. Attributes that have `EFI_VARIABLE_RUNTIME_ACCESS`
    /// set must also have `EFI_VARIABLE_BOOTSERVICE_ACCESS` set. The caller is responsible for following this rule.
    ///
    /// - Once `EFI_BOOT_SERVICES.ExitBootServices()` is performed, data variables that did not have `EFI_VARIABLE_RUNTIME_ACCESS`
    /// set are no longer visible to `GetVariable()`.
    ///
    /// - Once ExitBootServices() is performed, only variables that have `EFI_VARIABLE_RUNTIME_ACCESS` and `EFI_VARIABLE_NON_VOLATILE`
    /// set can be set with `SetVariable()`. Variables that have runtime access but that are not nonvolatile are read-only
    /// data variables once `ExitBootServices()` is performed. When the `EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS` attribute
    /// is set in a `SetVariable()` call, the authentication shall use the `EFI_VARIABLE_AUTHENTICATION_3` descriptor, which
    /// will be followed by any descriptors indicated in the `Type` and `Flags` fields.
    ///
    /// - When the `EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS` attribute is set in a `SetVariable()` call, the
    /// authentication shall use the `EFI_VARIABLE_AUTHENTICATION_2` descriptor.
    ///
    /// - If both the `EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS` and the `EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS`
    /// attribute are set in a `SetVariable()` call, then the firmware must return `EFI_INVALID_PARAMETER`.
    ///
    /// - If the `EFI_VARIABLE_APPEND_WRITE` attribute is set in a `SetVariable()` call, then any existing variable value
    /// shall be appended with the value of the `Data` parameter. If the firmware does not support the append operation,
    /// then the `SetVariable()` call shall return `EFI_INVALID_PARAMETER`.
    ///
    /// - If the `EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS` attribute is set in a `SetVariable()` call, and firmware
    /// does not support signature type of the certificate included in the `EFI_VARIABLE_AUTHENTICATION_2` descriptor, then
    /// the `SetVariable()` call shall return `EFI_INVALID_PARAMETER`. The list of signature types supported by the firmware
    /// is defined by the `SignatureSupport` variable. Signature type of the certificate is defined by its digest and encryption
    /// algorithms.
    ///
    /// - If the `EFI_VARIABLE_HARDWARE_ERROR_RECORD` attribute is set, `VariableName` and `VendorGuid` must comply with the
    /// rules stated in Hardware Error Record Variables and Hardware Error Record Persistence Usage. Otherwise, the `SetVariable()`
    /// call shall return `EFI_INVALID_PARAMETER`.
    ///
    /// - Globally Defined Variables must be created with the attributes defined in the Table Global Variables. If a globally
    /// defined variable is created with the wrong attributes, the result is indeterminate and may vary between implementations.
    ///
    /// - If using the `EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS` interface to update the cert authority for a given variable,
    /// it is valid for the `Data` region of the payload to be empty. This would update the cert without modifying the data
    /// itself. If the `Data` region is empty AND no `NewCert` is specified, the variable will be deleted (assuming all
    /// authorizations are verified).
    ///
    /// - Secure Boot Policy Variable must be created with the `EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS` attribute
    /// set, and the authentication shall use the `EFI_VARIABLE_AUTHENTICATION_2` descriptor. If the appropriate attribute
    /// bit is not set, then the firmware shall return `EFI_INVALID_PARAMETER`.
    ///
    /// - The only rules the firmware must implement when saving a nonvolatile variable is that it has actually been saved
    /// to nonvolatile storage before returning `EFI_SUCCESS`, and that a partial save is not performed. If power fails
    /// during a call to `SetVariable()` the variable may contain its previous value, or its new value. In addition there
    /// is no read, write, or delete security protection.
    ///
    /// - To delete a variable created with the `EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS` attribute, `SetVariable()`
    /// must be used with attributes matching the existing variable and the `DataSize` set to the size of the `AuthInfo`
    /// descriptor. The `Data` buffer must contain an instance of the `AuthInfo` descriptor which will be validated according
    /// to the steps in the appropriate section above referring to updates of `Authenticated` variables. An attempt to delete
    /// a variable created with the `EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS` attribute for which the prescribed
    /// AuthInfo `validation` fails or when called using `DataSize` of zero will fail with an `EFI_SECURITY_VIOLATION` status.
    ///
    /// - To delete a variable created with the `EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS` attribute, `SetVariable()` must
    /// be used with attributes matching the existing variable and the `DataSize` set to the size of the entire payload
    /// including all descriptors and certificates. The `Data` buffer must contain an instance of the `EFI_VARIABLE_AUTHENTICATION_3`
    /// descriptor which will indicate how to validate the payload according to the description in Using the `EFI_VARIABLE_AUTHENTICATION_3`
    /// descriptor. An attempt to delete a variable created with the `EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS` attribute
    /// for which the prescribed validation fails or when called using DataSize of zero will fail with an `EFI_SECURITY_VIOLATION`
    /// status.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code              | Description                                                                                                                                                                                                                                         |
    /// | ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`            | The firmware has successfully stored the variable and its data as defined by the `Attributes`. |
    /// | `EFI_INVALID_PARAMETER`  | An invalid combination of attribute bits, name, and GUID was supplied, or the `DataSize` exceeds the maximum allowed. |
    /// | `EFI_INVALID_PARAMETER`  | `VariableName` is an empty string. |
    /// | `EFI_OUT_OF_RESOURCES`   | Not enough storage is available to hold the variable and its data. |
    /// | `EFI_DEVICE_ERROR`       | The variable could not be saved due to a hardware failure. |
    /// | `EFI_WRITE_PROTECTED`    | The variable in question is read-only. |
    /// | `EFI_WRITE_PROTECTED`    | The variable in question cannot be deleted. |
    /// | `EFI_SECURITY_VIOLATION` | The variable could not be written due to `EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS` or `EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS` being set, but the payload does NOT pass the validation check carried out by the firmware. |
    /// | `EFI_NOT_FOUND`          | The variable trying to be updated or deleted was not found. |
    /// | `EFI_UNSUPPORTED`        | After `ExitBootServices()` has been called, this return code may be returned if no variable storage is supported. The platform should describe this runtime service as unsupported at runtime via an `EFI_RT_PROPERTIES_TABLE` configuration table. |
    pub SetVariable: unsafe extern "efiapi" fn(
        VariableName: *mut CHAR16,
        VendorGuid: *mut EFI_GUID,
        Attributes: UINT32,
        DataSize: UINTN,
        Data: *mut VOID,
    ) -> EFI_STATUS,
    /// Returns the next high 32 bits of the platform’s monotonic counter.
    ///
    /// ## Parameters
    ///
    /// | Parameter                 | Description                                                                                                           |
    /// | ------------------------- | --------------------------------------------------------------------------------------------------------------------- |
    /// | **OUT** `HighCount` | Pointer to returned value. |
    ///
    /// ## Description
    ///
    /// The `GetNextHighMonotonicCount()` function returns the next high 32 bits of the platform’s monotonic counter.
    ///
    /// The platform’s monotonic counter is comprised of two 32-bit quantities: the high 32 bits and the low 32 bits.
    /// During boot service time the low 32-bit value is volatile: it is reset to zero on every system reset and is increased
    /// by 1 on every call to `GetNextMonotonicCount()`. The high 32-bit value is nonvolatile and is increased by 1 whenever
    /// the system resets, whenever `GetNextHighMonotonicCount()` is called, or whenever the low 32-bit count (returned
    /// by `GetNextMonotonicCount()`) overflows.
    ///
    /// The `EFI_BOOT_SERVICES.GetNextMonotonicCount()` function is only available at boot services time. If the operating
    /// system wishes to extend the platform monotonic counter to runtime, it may do so by utilizing `GetNextHighMonotonicCount()`.
    /// To do this, before calling `EFI_BOOT_SERVICES.ExitBootServices()` the operating system would call `GetNextMonotonicCount()`
    /// to obtain the current platform monotonic count. The operating system would then provide an interface that returns
    /// the next count by:
    ///
    /// - Adding 1 to the last count.
    ///
    /// - Before the lower 32 bits of the count overflows, call `GetNextHighMonotonicCount()`. This will increase the high
    /// 32 bits of the platform’s nonvolatile portion of the monotonic count by 1.
    ///
    /// This function may only be called at runtime.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code              | Description                                                                                                                                                                                                                                         |
    /// | ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`            | The next high monotonic count was returned. |
    /// | `EFI_DEVICE_ERROR`  | The device is not functioning properly. |
    /// | `EFI_INVALID_PARAMETER`  | `HighCount` is `NULL`. |
    /// | `EFI_UNSUPPORTED`  | This call is not supported by this platform at the time the call is made. The platform should describe this runtime service as unsupported at runtime via an `EFI_RT_PROPERTIES_TABLE` configuration table. |
    pub GetNextHighMonotonicCount: unsafe extern "efiapi" fn(HighCount: *mut UINT32) -> EFI_STATUS,
    /// Resets the entire platform. If the platform supports `EFI_RESET_NOTIFICATION_PROTOCOL`, then prior to completing
    /// the reset of the platform, all of the pending notifications must be called.
    ///
    /// ## Parameters
    ///
    /// | Parameter                 | Description                                                                                                           |
    /// | ------------------------- | --------------------------------------------------------------------------------------------------------------------- |
    /// | **IN** `ResetType` | The type of reset to perform. |
    /// | **IN** `ResetStatus` | The status code for the reset. If the system reset is part of a normal operation, the status code would be `EFI_SUCCESS`. If the system reset is due to some type of failure the most appropriate EFI Status code would be used. |
    /// | **IN** `DataSize` | The size, in bytes, of `ResetData`. |
    /// | **IN** `ResetData` | For a `ResetType` of `EfiResetCold`, `EfiResetWarm`, or `EfiResetShutdown` the data buffer starts with a null-terminated string, optionally followed by additional binary data. The string is a description that the caller may use to further indicate the reason for the system reset. For a `ResetType` of `EfiResetPlatformSpecific` the data buffer also starts with a null-terminated string that is followed by an `EFI_GUID` that describes the specific type of reset to perform. |
    ///
    /// ## Description
    ///
    /// The `ResetSystem()` function resets the entire platform, including all processors and devices, and reboots the system.
    ///
    /// Calling this interface with `ResetType` of `EfiResetCold` causes a system-wide reset. This sets all circuitry within
    /// the system to its initial state. This type of reset is asynchronous to system operation and operates without regard
    /// to cycle boundaries. `EfiResetCold` is tantamount to a system power cycle.
    ///
    /// Calling this interface with `ResetType` of `EfiResetWarm` causes a system-wide initialization. The processors are
    /// set to their initial state, and pending cycles are not corrupted. If the system does not support this reset type,
    /// then an `EfiResetCold` must be performed.
    ///
    /// Calling this interface with `ResetType` of `EfiResetShutdown` causes the system to enter a power state equivalent
    /// to the ACPI G2/S5 or G3 states. If the system does not support this reset type, then when the system is rebooted,
    /// it should exhibit the `EfiResetCold` attributes.
    ///
    /// Calling this interface with `ResetType` of `EfiResetPlatformSpecific` causes a system-wide reset. The exact type
    /// of the reset is defined by the `EFI_GUID` that follows the null-terminated Unicode string passed into `ResetData`.
    /// If the platform does not recognize the `EFI_GUID` in `ResetData` the platform must pick a supported reset type to
    /// perform.The platform may optionally log the parameters from any non-normal reset that occurs.
    ///
    /// The `ResetSystem()` function does not return.
    pub ResetSystem: unsafe extern "efiapi" fn(
        ResetType: EFI_RESET_TYPE,
        ResetStatus: EFI_STATUS,
        DataSize: UINTN,
        ResetData: *mut VOID,
    ) -> VOID,
    /// Passes capsules to the firmware with both virtual and physical mapping. Depending on the intended consumption,
    /// the firmware may process the capsule immediately. If the payload should persist across a system reset, the reset
    /// value returned from `EFI_RUNTIME_SERVICES.QueryCapsuleCapabilities()` must be passed into `EFI_RUNTIME_SERVICES.ResetSystem()`
    /// and will cause the capsule to be processed by the firmware as part of the reset process.
    ///
    /// ## Parameters
    ///
    /// | Parameter                 | Description                                                                                                           |
    /// | ------------------------- | --------------------------------------------------------------------------------------------------------------------- |
    /// | **IN** `CapsuleHeaderArray` | Virtual pointer to an array of virtual pointers to the capsules being passed into update capsule. Each capsules is assumed to stored in contiguous virtual memory. The capsules in the `CapsuleHeaderArray` must be the same capsules as the `ScatterGatherList`. The `CapsuleHeaderArray` must have the capsules in the same order as the `ScatterGatherList`. |
    /// | **IN** `CapsuleCount` | Number of pointers to `EFI_CAPSULE_HEADER` in `CapsuleHeaderArray`. |
    /// | **IN** `ScatterGatherList` | Physical pointer to a set of `EFI_CAPSULE_BLOCK_DESCRIPTOR` that describes the location in physical memory of a set of capsules. The capsules in the `ScatterGatherList` must be in the same order as the `CapsuleHeaderArray`. This parameter is only referenced if the capsules are defined to persist across system reset. |
    ///
    /// ## Description
    ///
    /// The `UpdateCapsule()` function allows the operating system to pass information to firmware. The `UpdateCapsule()`
    /// function supports passing capsules in operating system virtual memory back to firmware. Each capsule is contained
    /// in a contiguous virtual memory range in the operating system, but both a virtual and physical mapping for the capsules
    /// are passed to the firmware.
    ///
    /// If a capsule has the `CAPSULE_FLAGS_PERSIST_ACROSS_RESET` flag set in its header, the firmware will process the
    /// capsules after system reset. The caller must ensure to reset the system using the required reset value obtained
    /// from `QueryCapsuleCapabilities`. If this flag is not set, the firmware will process the capsules immediately.
    ///
    /// A capsule which has the `CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE` flag must have `CAPSULE_FLAGS_PERSIST_ACROSS_RESET`
    /// set in its header as well. Firmware that processes a capsule that has the `CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE`
    /// flag set in its header will coalesce the contents of the capsule from the `ScatterGatherList` into a contiguous
    /// buffer and must then place a pointer to this coalesced capsule in the EFI System Table after the system has been
    /// reset. Agents searching for this capsule will look in the `EFI_CONFIGURATION_TABLE` and search for the capsule’s
    /// GUID and associated pointer to retrieve the data after the reset.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code              | Description                                                                                                                                                                                                                                         |
    /// | ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`            | Valid capsule was passed. If `CAPSULE_FLAGS_PERSIST_ACROSS_RESET` is not set, the capsule has been successfully processed by the firmware. |
    /// | `EFI_INVALID_PARAMETER`  | `CapsuleSize`, or an incompatible set of flags were set in the capsule header. |
    /// | `EFI_INVALID_PARAMETER`  | `CapsuleCount` is `0`. |
    /// | `EFI_DEVICE_ERROR`  | The capsule update was started, but failed due to a device error. |
    /// | `EFI_UNSUPPORTED`  | The capsule type is not supported on this platform. |
    /// | `EFI_OUT_OF_RESOURCES`  | When `ExitBootServices()` has been previously called this error indicates the capsule is compatible with this platform but is not capable of being submitted or processed in runtime. The caller may resubmit the capsule prior to `ExitBootServices()`. |
    /// | `EFI_OUT_OF_RESOURCES`  | When `ExitBootServices()` has not been previously called then this error indicates the capsule is compatible with this platform but there are insufficient resources to process. |
    /// | `EFI_UNSUPPORTED`  | This call is not supported by this platform at the time the call is made. The platform should describe this runtime service as unsupported at runtime via an `EFI_RT_PROPERTIES_TABLE` configuration table. |
    pub UpdateCapsule: unsafe extern "efiapi" fn(
        CapsuleHeaderArray: *mut *mut EFI_CAPSULE_HEADER,
        CapsuleCount: UINTN,
        ScatterGatherList: EFI_PHYSICAL_ADDRESS,
    ) -> EFI_STATUS,
    /// Returns information about the EFI variables.
    ///
    /// ## Parameters
    ///
    /// | Parameter                 | Description                                                                                                           |
    /// | ------------------------- | --------------------------------------------------------------------------------------------------------------------- |
    /// | **IN** `Attributes` | Attributes bitmask to specify the type of variables on which to return information. The `EFI_VARIABLE_APPEND_WRITE` attribute, if set in the attributes bitmask, will be ignored. |
    /// | **OUT** `MaximumVariableStorageSize` | On output the maximum size of the storage space available for the EFI variables associated with the attributes specified. |
    /// | **OUT** `RemainingVariableStorageSize` | Returns the remaining size of the storage space available for EFI variables associated with the attributes specified. |
    /// | **OUT** `MaximumVariableSize` | Returns the maximum size of an individual EFI variable associated with the attributes specified. |
    ///
    /// ## Description
    ///
    /// The `QueryVariableInfo()` function allows a caller to obtain the information about the maximum size of the storage
    /// space available for the EFI variables, the remaining size of the storage space available for the EFI variables
    /// and the maximum size of each individual EFI variable, associated with the attributes specified.
    ///
    /// The `MaximumVariableSize` value will reflect the overhead associated with the saving of a single EFI variable
    /// with the exception of the overhead associated with the length of the string name of the EFI variable.
    ///
    /// The returned `MaximumVariableStorageSize`, `RemainingVariableStorageSize`, `MaximumVariableSize` information may
    /// change immediately after the call based on other runtime activities including asynchronous error events. Also,
    /// these values associated with different attributes are not additive in nature.
    ///
    /// After the system has transitioned into runtime (after `ExitBootServices()` is called), an implementation may not
    /// be able to accurately return information about the Boot Services variable store. In such cases, `EFI_INVALID_PARAMETER`
    /// should be returned.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code              | Description                                                                                                                                                                                                                                         |
    /// | ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS`            | Valid answer returned. |
    /// | `EFI_INVALID_PARAMETER`  | An invalid combination of attribute bits was supplied |
    /// | `EFI_UNSUPPORTED`        | The attribute is not supported on this platform, and the `MaximumVariableStorageSize`, `RemainingVariableStorageSize`, `MaximumVariableSize` are undefined. |
    pub QueryVariableInfo: unsafe extern "efiapi" fn(
        Attributes: UINT32,
        MaximumVariableStorageSize: *mut UINT64,
        RemainingVariableStorageSize: *mut UINT64,
        MaximumVariableSize: *mut UINT64,
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
    /// When entering daylight saving time, if the time is affected, but hasn't been adjusted (DST = 1),
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

/// An extensible structure to identify a unique x509 certificate associated with a given variable
#[repr(C)]
pub struct EFI_VARIABLE_AUTHENTICATION_3_CERT_ID {
    /// Identifies the type of ID that is returned and how the ID should be interpreted.
    pub Type: UINT8,
    /// Indicates the size of the `Id` buffer that follows this field in the structure.
    pub IdSize: UINT32,
}

#[repr(C)]
pub struct EFI_CAPSULE_BLOCK_DESCRIPTOR {
    /// Length in bytes of the data pointed to by `DataBlock` or `ContinuationPointer`.
    pub Length: UINT64,
    pub Union: __,
}

#[repr(C)]
pub struct EFI_CAPSULE_HEADER {
    /// A GUID that defines the contents of a capsule.
    pub CapsuleGuid: EFI_GUID,
    /// The size of the capsule header. This may be larger than the size of the `EFI_CAPSULE_HEADER` since `CapsuleGuid`
    /// may imply extended header entries.
    pub HeaderSize: UINT32,
    /// The Bits \[`0`, `15`] bits are defined by `CapsuleGuid`. Bits \[`16`, `31`] are defined by this specification.
    pub Flags: UINT32,
    /// Size in bytes of the capsule (including capsule header).
    pub CapsuleImageSize: UINT32,
}

#[repr(C)]
pub union __ {
    /// Physical address of the data block. This member of the union is used if `Length` is not equal to zero.
    pub DataBlock: EFI_PHYSICAL_ADDRESS,
    /// Physical address of another block of `EFI_CAPSULE_BLOCK_DESCRIPTOR` structures. This member of the union is used
    /// if Length is equal to zero. If `ContinuationPointer` is zero this entry represents the end of the list.
    pub ContinuationPointer: EFI_PHYSICAL_ADDRESS,
}
