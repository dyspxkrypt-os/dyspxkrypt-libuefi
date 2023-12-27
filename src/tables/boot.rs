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

use crate::protocols::device_path::EFI_DEVICE_PATH_PROTOCOL;
use crate::tables::system::EFI_SPECIFICATION_VERSION;
use crate::tables::EFI_TABLE_HEADER;
use crate::types::{
    BOOLEAN, CHAR16, EFI_EVENT, EFI_GUID, EFI_HANDLE, EFI_STATUS, EFI_TPL, UINT32, UINT64, UINTN,
    VOID,
};

pub const EFI_BOOT_SERVICES_SIGNATURE: UINT64 = 0x56524553544f4F42;
pub const EFI_BOOT_SERVICES_REVISION: UINT32 = EFI_SPECIFICATION_VERSION;

pub const EFI_MEMORY_UC: UINT64 = 0x0000000000000001;
pub const EFI_MEMORY_WC: UINT64 = 0x0000000000000002;
pub const EFI_MEMORY_WT: UINT64 = 0x0000000000000004;
pub const EFI_MEMORY_WB: UINT64 = 0x0000000000000008;
pub const EFI_MEMORY_UCE: UINT64 = 0x0000000000000010;
pub const EFI_MEMORY_WP: UINT64 = 0x0000000000001000;
pub const EFI_MEMORY_RP: UINT64 = 0x0000000000002000;
pub const EFI_MEMORY_XP: UINT64 = 0x0000000000004000;
pub const EFI_MEMORY_NV: UINT64 = 0x0000000000008000;
pub const EFI_MEMORY_MORE_RELIABLE: UINT64 = 0x0000000000010000;
pub const EFI_MEMORY_RO: UINT64 = 0x0000000000020000;
pub const EFI_MEMORY_SP: UINT64 = 0x0000000000040000;
pub const EFI_MEMORY_CPU_CRYPTO: UINT64 = 0x0000000000080000;
pub const EFI_MEMORY_RUNTIME: UINT64 = 0x8000000000000000;
pub const EFI_MEMORY_ISA_VALID: UINT64 = 0x4000000000000000;
pub const EFI_MEMORY_ISA_MASK: UINT64 = 0x0FFFF00000000000;

pub const EVT_TIMER: UINT32 = 0x80000000;
pub const EVT_RUNTIME: UINT32 = 0x40000000;

pub const EVT_NOTIFY_WAIT: UINT32 = 0x00000100;
pub const EVT_NOTIFY_SIGNAL: UINT32 = 0x00000200;

pub const EVT_SIGNAL_EXIT_BOOT_SERVICES: UINT32 = 0x00000201;
pub const EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE: UINT32 = 0x60000202;

pub const EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL: UINT32 = 0x00000001;
pub const EFI_OPEN_PROTOCOL_GET_PROTOCOL: UINT32 = 0x00000002;
pub const EFI_OPEN_PROTOCOL_TEST_PROTOCOL: UINT32 = 0x00000004;
pub const EFI_OPEN_PROTOCOL_BY_CHILD_CONTROLLER: UINT32 = 0x00000008;
pub const EFI_OPEN_PROTOCOL_BY_DRIVER: UINT32 = 0x00000010;
pub const EFI_OPEN_PROTOCOL_EXCLUSIVE: UINT32 = 0x00000020;

pub const EFI_EVENT_GROUP_EXIT_BOOT_SERVICES: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x27ABF055,
        0xB1B8,
        0x4C26,
        [0x80, 0x48, 0x74, 0x8F, 0x37, 0xBA, 0xA2, 0xDF],
    )
};

pub const EFI_EVENT_GROUP_BEFORE_EXIT_BOOT_SERVICES: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x8BE0E274,
        0x3970,
        0x4B44,
        [0x80, 0xC5, 0x1A, 0xB9, 0x50, 0x2F, 0x3B, 0xFC],
    )
};

pub const EFI_EVENT_GROUP_VIRTUAL_ADDRESS_CHANGE: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x13FA7698,
        0xC831,
        0x49C7,
        [0x87, 0xEA, 0x8F, 0x43, 0xFC, 0xC2, 0x51, 0x96],
    )
};

pub const EFI_EVENT_GROUP_MEMORY_MAP_CHANGE: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x78BEE926,
        0x692F,
        0x48FD,
        [0x9E, 0xDB, 0x01, 0x42, 0x2E, 0xF0, 0xD7, 0xAB],
    )
};

pub const EFI_EVENT_GROUP_READY_TO_BOOT: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x7CE88FB3,
        0x4BD7,
        0x4679,
        [0x87, 0xA8, 0xA8, 0xD8, 0xDE, 0xE5, 0x0D, 0x2B],
    )
};

pub const EFI_EVENT_GROUP_AFTER_READY_TO_BOOT: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x3A2A00AD,
        0x98B9,
        0x4CDF,
        [0xA4, 0x78, 0x70, 0x27, 0x77, 0xF1, 0xC1, 0x0B],
    )
};

pub const EFI_EVENT_GROUP_RESET_SYSTEM: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x62DA6A56,
        0x13FB,
        0x485A,
        [0xA8, 0xDA, 0xA3, 0xDD, 0x79, 0x12, 0xCB, 0x6B],
    )
};

#[repr(C)]
pub enum EFI_ALLOCATE_TYPE {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[repr(C)]
pub enum EFI_MEMORY_TYPE {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
    EfiMaxMemoryType,
}

#[repr(C)]
pub enum EFI_TIMER_DELAY {
    TimerCancel,
    TimerPeriodic,
    TimerRelative,
}

#[repr(C)]
pub enum EFI_INTERFACE_TYPE {
    EFI_NATIVE_INTERFACE,
}

#[repr(C)]
pub enum EFI_LOCATE_SEARCH_TYPE {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

/// The EFI Boot Services containing a table header and pointers to all of the boot services.
#[repr(C)]
pub struct EFI_BOOT_SERVICES {
    /// The table header for the EFI Boot Services Table. This header contains the `EFI_BOOT_SERVICES_SIGNATURE` and
    /// `EFI_BOOT_SERVICES_REVISION` values along with the size of the `EFI_BOOT_SERVICES` structure and a 32-bit CRC to
    /// verify that the contents of the EFI Boot Services Table are valid.
    pub Hdr: EFI_TABLE_HEADER,
    /// Raises a task’s priority level and returns its previous level.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                       |
    /// | --------------- | ------------------------------------------------------------------------------------------------- |
    /// | **IN** `NewTPL` | The new task priority level. It must be greater than or equal to the current task priority level. |
    ///
    /// ## Description
    ///
    /// This function raises the priority of the currently executing task and returns its previous priority level.
    ///
    /// Only three task priority levels are exposed outside of the firmware during boot services execution. The first is
    /// `TPL_APPLICATION` where all normal execution occurs. That level may be interrupted to perform various asynchronous
    /// interrupt style notifications, which occur at the `TPL_CALLBACK` or `TPL_NOTIFY` level. By raising the task priority level
    /// to `TPL_NOTIFY` such notifications are masked until the task priority level is restored, thereby synchronizing
    /// execution with such notifications. Synchronous blocking I/O functions execute at `TPL_NOTIFY`. `TPL_CALLBACK` is
    /// typically used for application level notification functions. Device drivers will typically use `TPL_CALLBACK`
    /// or `TPL_NOTIFY` for their notification functions. Applications and drivers may also use `TPL_NOTIFY` to protect
    /// data structures in critical sections of code.
    ///
    /// The caller must restore the task priority level with `EFI_BOOT_SERVICES.RestoreTPL()` to the previous level
    /// before returning.
    ///
    /// **Note:** If `NewTpl` is below the current TPL level, then the system behavior is indeterminate. Additionally,
    /// only `TPL_APPLICATION`, `TPL_CALLBACK`, `TPL_NOTIFY` and `TPL_HIGH_LEVEL` may be used. All other values are
    /// reserved for use by the firmware; using them will result in unpredictable behavior. Good coding practice
    /// dictates that all code should execute at its lowest possible TPL level, and the use of TPL levels above
    /// `TPL_APPLICATION` must be minimized. Executing at TPL levels above `TPL_APPLICATION` for extended periods of
    /// time may also result in unpredictable behavior.
    ///
    /// ## Status Codes Returned
    ///
    /// Unlike other UEFI interface functions, `EFI_BOOT_SERVICES.RaiseTPL()` does not return a status code. Instead, it
    /// returns the previous task priority level, which is to be restored later with a matching call to `RestoreTPL()`.
    pub RaiseTPL: unsafe extern "efiapi" fn(NewTPL: EFI_TPL) -> EFI_TPL,
    /// Restores a task’s priority level to its previous value.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `OldTPL` | The previous task priority level to restore (the value from a previous, matching call to `EFI_BOOT_SERVICES.RaiseTPL()`. |
    ///
    /// ## Description
    ///
    /// The `RestoreTPL()` function restores a task’s priority level to its previous value. Calls to `RestoreTPL()` are
    /// matched with calls to `RaiseTPL()`.
    ///
    /// NOTE: If `OldTpl` is above the current TPL level, then the system behavior is indeterminate. Additionally, only
    /// `TPL_APPLICATION`, `TPL_CALLBACK`, `TPL_NOTIFY`, and `TPL_HIGH_LEVEL` may be used. All other values are reserved
    /// for use by the firmware; using them will result in unpredictable behavior. Good coding practice dictates that all
    /// code should execute at its lowest possible TPL level, and the use of TPL levels above `TPL_APPLICATION` must be
    /// minimized. Executing at TPL levels above `TPL_APPLICATION` for extended periods of time may also result in
    /// unpredictable behavior.
    ///
    /// ## Status Codes Returned
    ///
    /// None.
    pub RestoreTPL: unsafe extern "efiapi" fn(OldTPL: EFI_TPL) -> VOID,
    /// Allocates memory pages from the system.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Type` | The type of allocation to perform. |
    /// | **IN** `MemoryType` | The type of memory to allocate. These memory types are also described in more detail in Memory Type Usage before `ExitBootServices()`, and Memory Type Usage after `ExitBootServices()`. Normal allocations (that is, allocations by any UEFI application) are of type `EfiLoaderData`. `MemoryType` values in the range `0x70000000..0x7FFFFFFF` are reserved for OEM use. `MemoryType` values in the range `0x80000000..0xFFFFFFFF` are reserved for use by UEFI OS loaders that are provided by operating system vendors. |
    /// | **IN** `Pages` | The number of contiguous 4 KiB pages to allocate. |
    /// | **IN OUT** `Memory` | Pointer to a physical address. On input, the way in which the address is used depends on the value of `Type`. On output the address is set to the base of the page range that was allocated. |
    ///
    /// ## Description
    ///
    /// The `AllocatePages()` function allocates the requested number of pages and returns a pointer to the base address
    /// of the page range in the location referenced by Memory. The function scans the memory map to locate free pages.
    /// When it finds a physically contiguous block of pages that is large enough and also satisfies the allocation requirements
    /// of `Type`, it changes the memory map to indicate that the pages are now of type `MemoryType`.
    ///
    /// In general, UEFI OS loaders and UEFI applications should allocate memory (and pool) of type EfiLoaderData. UEFI
    /// boot service drivers must allocate memory (and pool) of type `EfiBootServicesData`. UEFI runtime drivers should
    /// allocate memory (and pool) of type `EfiRuntimeServicesData` (although such allocation can only be made during boot
    /// services time).
    ///
    /// Allocation requests of type `AllocateAnyPages` allocate any available range of pages that satisfies the request.
    /// On input, the address pointed to by `Memory` is ignored.
    ///
    /// Allocation requests of type `AllocateMaxAddress` allocate any available range of pages whose uppermost address
    /// is less than or equal to the address pointed to by `Memory` on input.
    ///
    /// Allocation requests of type `AllocateAddress` allocate pages at the address pointed to by `Memory` on input.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The requested pages were allocated. |
    /// | `EFI_OUT_OF_RESOURCES` | The pages could not be allocated. |
    /// | `EFI_INVALID_PARAMETER` | `Type` is not `AllocateAnyPages` or `AllocateMaxAddress` or `AllocateAddress`. |
    /// | `EFI_INVALID_PARAMETER` | `MemoryType` is in the range `EfiMaxMemoryType..0x6FFFFFFF`. |
    /// | `EFI_INVALID_PARAMETER` | `MemoryType` is `EfiPersistentMemoryType` or `EfiUnacceptedMemory`. |
    /// | `EFI_INVALID_PARAMETER` | `Memory` is `NULL`. |
    /// | `EFI_NOT_FOUND` | The requested pages could not be found. |
    pub AllocatePages: unsafe extern "efiapi" fn(
        Type: EFI_ALLOCATE_TYPE,
        MemoryType: EFI_MEMORY_TYPE,
        Pages: UINTN,
        Memory: *mut EFI_PHYSICAL_ADDRESS,
    ) -> EFI_STATUS,
    /// Frees memory pages.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Memory` | The base physical address of the pages to be freed. |
    /// | **IN** `Pages` | The number of contiguous 4 KiB pages to free. |
    ///
    /// ## Description
    ///
    /// The `FreePages()` function returns memory allocated by `AllocatePages()` to the firmware.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The requested memory pages were freed. |
    /// | `EFI_NOT_FOUND` | The requested memory pages were not allocated with `AllocatePages()`. |
    /// | `EFI_INVALID_PARAMETER` | `Memory` is not a page-aligned address or `Pages` is invalid. |
    pub FreePages:
        unsafe extern "efiapi" fn(Memory: EFI_PHYSICAL_ADDRESS, Pages: UINTN) -> EFI_STATUS,
    /// Returns the current memory map.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN OUT** `MemoryMapSize` | A pointer to the size, in bytes, of the `MemoryMap` buffer. On input, this is the size of the buffer allocated by the caller. On output, it is the size of the buffer returned by the firmware if the buffer was large enough, or the size of the buffer needed to contain the map if the buffer was too small. |
    /// | **OUT** `MemoryMap` | A pointer to the buffer in which firmware places the current memory map. The map is an array of `EFI_MEMORY_DESCRIPTOR`s. |
    /// | **OUT** `MapKey` | A pointer to the location in which firmware returns the key for the current memory map. |
    /// | **OUT** `DescriptorSize` | A pointer to the location in which firmware returns the size, in bytes, of an individual `EFI_MEMORY_DESCRIPTOR`. |
    /// | **OUT** `DescriptorVersion` | A pointer to the location in which firmware returns the version number associated with the `EFI_MEMORY_DESCRIPTOR`. |
    ///
    /// ## Description
    ///
    /// The `GetMemoryMap()` function returns a copy of the current memory map. The map is an array of memory descriptors,
    /// each of which describes a contiguous block of memory. The map describes all of memory, no matter how it is being used.
    /// That is, it includes blocks allocated by `EFI_BOOT_SERVICES.AllocatePages()` and `EFI_BOOT_SERVICES.AllocatePool()`,
    /// as well as blocks that the firmware is using for its own purposes. The memory map is only used to describe memory
    /// that is present in the system. The firmware does not return a range description for address space regions that are
    /// not backed by physical hardware. Regions that are backed by physical hardware, but are not supposed to be accessed
    /// by the OS, must be returned as `EfiReservedMemoryType`. The OS may use addresses of memory ranges that are not
    /// described in the memory map at its own discretion.
    ///
    /// Until `EFI_BOOT_SERVICES.ExitBootServices()` is called, the memory map is owned by the firmware and the currently
    /// executing UEFI Image should only use memory pages it has explicitly allocated.
    ///
    /// If the `MemoryMap` buffer is too small, the `EFI_BUFFER_TOO_SMALL` error code is returned and the `MemoryMapSize`
    /// value contains the size of the buffer needed to contain the current memory map. The actual size of the buffer allocated
    /// for the consequent call to `GetMemoryMap()` should be bigger then the value returned in `MemoryMapSize`, since allocation
    /// of the new buffer may potentially increase memory map size.
    ///
    /// On success a `MapKey` is returned that identifies the current memory map. The firmware’s key is changed every time
    /// something in the memory map changes. In order to successfully invoke `EFI_BOOT_SERVICES.ExitBootServices()` the
    /// caller must provide the current memory map key.
    ///
    /// The `GetMemoryMap()` function also returns the size and revision number of the `EFI_MEMORY_DESCRIPTOR`. The `DescriptorSize`
    /// represents the size in bytes of an `EFI_MEMORY_DESCRIPTOR` array element returned in `MemoryMap`. The size is returned
    /// to allow for future expansion of the `EFI_MEMORY_DESCRIPTOR` in response to hardware innovation. The structure of
    /// the `EFI_MEMORY_DESCRIPTOR` may be extended in the future but it will remain backwards compatible with the current
    /// definition. Thus OS software must use the `DescriptorSize` to find the start of each `EFI_MEMORY_DESCRIPTOR` in the
    /// `MemoryMap` array.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The memory map was returned in the `MemoryMap` buffer. |
    /// | `EFI_BUFFER_TOO_SMALL` | The `MemoryMap` buffer was too small. The current buffer size needed to hold the memory map is returned in `MemoryMapSize`. |
    /// | `EFI_INVALID_PARAMETER` | `MemoryMapSize` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | The `MemoryMap` buffer is not too small and `MemoryMap` is `NULL`. |
    pub GetMemoryMap: unsafe extern "efiapi" fn(
        MemoryMapSize: *mut UINTN,
        MemoryMap: *mut EFI_MEMORY_DESCRIPTOR,
        MapKey: *mut UINTN,
        DescriptorSize: *mut UINTN,
        DescriptorVersion: *mut UINT32,
    ) -> EFI_STATUS,
    /// Allocates pool memory.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `PoolType` | The type of pool to allocate. `PoolType` values in the range `0x70000000..0x7FFFFFFF` are reserved for OEM use. PoolType values in the range `0x80000000..0xFFFFFFFF` are reserved for use by UEFI OS loaders that are provided by operating system vendors. |
    /// | **IN** `Size` | The number of bytes to allocate from the pool. |
    /// | **OUT** `Buffer` | A pointer to a pointer to the allocated buffer if the call succeeds; undefined otherwise. |
    ///
    /// ## Description
    ///
    /// The `AllocatePool()` function allocates a memory region of `Size` bytes from memory of type `PoolType` and returns
    /// the address of the allocated memory in the location referenced by Buffer. This function allocates pages from
    /// `EfiConventionalMemory` as needed to grow the requested pool type. All allocations are eight-byte aligned.
    ///
    /// The allocated pool memory is returned to the available pool with the `EFI_BOOT_SERVICES.FreePool()` function.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The requested number of bytes was allocated. |
    /// | `EFI_OUT_OF_RESOURCES` | The pool requested could not be allocated. |
    /// | `EFI_INVALID_PARAMETER` | `PoolType` is in the range `EfiMaxMemoryType..0x6FFFFFFF`. |
    /// | `EFI_INVALID_PARAMETER` | `PoolType` is `EfiPersistentMemory`. |
    /// | `EFI_INVALID_PARAMETER` | `Buffer` is `NULL`. |
    pub AllocatePool: unsafe extern "efiapi" fn(
        PoolType: EFI_MEMORY_TYPE,
        Size: UINTN,
        Buffer: *mut *mut VOID,
    ) -> EFI_STATUS,
    /// Returns pool memory to the system.
    ///
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Buffer` | Pointer to the buffer to free. |
    ///
    /// ## Description
    ///
    /// The `FreePool()` function returns the memory specified by `Buffer` to the system. On return, the memory’s type is
    /// `EfiConventionalMemory`. The `Buffer` that is freed must have been allocated by `AllocatePool()`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The memory was returned to the system. |
    /// | `EFI_INVALID_PARAMETER` | `Buffer` was invalid. |
    pub FreePool: unsafe extern "efiapi" fn(Buffer: *mut VOID) -> EFI_STATUS,
    /// Creates an event.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Type` | The type of event to create and its mode and attributes. |
    /// | **IN** `NotifyTPL` | The task priority level of event notifications, if needed. |
    /// | **IN** `NotifyFunction` **OPTIONAL** | Pointer to the event’s notification function, if any. |
    /// | **IN** `NotifyContext` **OPTIONAL** | Pointer to the notification function’s context; corresponds to parameter `Context` in the notification function. |
    /// | **OUT** `Event` | Pointer to the newly created event if the call succeeds; undefined otherwise. |
    ///
    /// ## Description
    ///
    /// The `CreateEvent()` function creates a new event of type `Type` and returns it in the location referenced by
    /// `Event`. The event’s notification function, context, and task priority level are specified by `NotifyFunction`,
    /// `NotifyContext`, and `NotifyTPL`, respectively.
    ///
    /// Events exist in one of two states, “waiting” or “signaled.” When an event is created, firmware puts it in the
    /// “waiting” state. When the event is signaled, firmware changes its state to “signaled” and, if `EVT_NOTIFY_SIGNAL`
    /// is specified, places a call to its notification function in a FIFO queue. There is a queue for each of the “basic”
    /// task priority levels defined in Event, Timer, and Task Priority Services (`TPL_CALLBACK`, and `TPL_NOTIFY`). The
    /// functions in these queues are invoked in FIFO order, starting with the highest priority level queue and proceeding
    /// to the lowest priority queue that is unmasked by the current TPL. If the current TPL is equal to or greater than
    /// the queued notification, it will wait until the TPL is lowered via `EFI_BOOT_SERVICES.RestoreTPL()`.
    ///
    /// In a general sense, there are two “types” of events, synchronous and asynchronous. Asynchronous events are closely
    /// related to timers and are used to support periodic or timed interruption of program execution. This capability
    /// is typically used with device drivers. For example, a network device driver that needs to poll for the presence
    /// of new packets could create an event whose type includes `EVT_TIMER` and then call the `EFI_BOOT_SERVICES.SetTimer()`
    /// function. When the timer expires, the firmware signals the event.
    ///
    /// Synchronous events have no particular relationship to timers. Instead, they are used to ensure that certain
    /// activities occur following a call to a specific interface function. One example of this is the cleanup that
    /// needs to be performed in response to a call to the `EFI_BOOT_SERVICES.ExitBootServices()` function. `ExitBootServices()`
    /// can clean up the firmware since it understands firmware internals, but it cannot clean up on behalf of drivers
    /// that have been loaded into the system. The drivers have to do that themselves by creating an event whose type is
    /// `EVT_SIGNAL_EXIT_BOOT_SERVICES` and whose notification function is a function within the driver itself. Then, when
    /// `ExitBootServices()` has finished its cleanup, it signals each event of type `EVT_SIGNAL_EXIT_BOOT_SERVICES`.
    ///
    /// Another example of the use of synchronous events occurs when an event of type `EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE`
    /// is used in conjunction with the `SetVirtualAddressMap()`.
    ///
    /// The `EVT_NOTIFY_WAIT` and `EVT_NOTIFY_SIGNAL` flags are exclusive. If neither flag is specified, the caller does
    /// not require any notification concerning the event and the `NotifyTPL`, `NotifyFunction`, and `NotifyContext`
    /// parameters are ignored. If `EVT_NOTIFY_WAIT` is specified and the event is not in the signaled state, then the
    /// `EVT_NOTIFY_WAIT` notify function is queued whenever a consumer of the event is waiting for the event (via
    /// `EFI_BOOT_SERVICES.WaitForEvent()` or `EFI_BOOT_SERVICES.CheckEvent()`). If the `EVT_NOTIFY_SIGNAL` flag is
    /// specified then the event’s notify function is queued whenever the event is signaled.
    ///
    /// **Note:** Because its internal structure is unknown to the caller, Event cannot be modified by the caller. The
    /// only way to manipulate it is to use the published event interfaces.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The event structure was created. |
    /// | `EFI_INVALID_PARAMETER` | One of the parameters has an invalid value. |
    /// | `EFI_INVALID_PARAMETER` | `Event` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Type` has an unsupported bit set. |
    /// | `EFI_INVALID_PARAMETER` | `Type` has both `EVT_NOTIFY_SIGNAL` and `EVT_NOTIFY_WAIT` set. |
    /// | `EFI_INVALID_PARAMETER` | `Type` has either `EVT_NOTIFY_SIGNAL` or `EVT_NOTIFY_WAIT` set and `NotifyFunction` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Type` has either `EVT_NOTIFY_SIGNAL` or `EVT_NOTIFY_WAIT` set and `NotifyTPL` is not a supported TPL level. |
    /// | `EFI_OUT_OF_RESOURCES` | The event could not be allocated. |
    pub CreateEvent: unsafe extern "efiapi" fn(
        Type: UINT32,
        NotifyTPL: EFI_TPL,
        NotifyFunction: EFI_EVENT_NOTIFY,
        NotifyContext: *mut VOID,
        Event: *mut EFI_EVENT,
    ) -> EFI_STATUS,
    /// Sets the type of timer and the trigger time for a timer event.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Event` | The timer event that is to be signaled at the specified time. |
    /// | **IN** `Type` | The type of time that is specified in `TriggerTime`. |
    /// | **IN** `TriggerTime` | The number of 100ns units until the timer expires. A `TriggerTime` of `0` is legal. If `Type` is `TimerRelative` and `TriggerTime` is `0`, then the timer event will be signaled on the next timer tick. If `Type` is `TimerPeriodic` and `TriggerTime` is `0`, then the timer event will be signaled on every timer tick. |
    ///
    /// ## Description
    ///
    /// The `SetTimer()` function cancels any previous time trigger setting for the event, and sets the new trigger time
    /// for the event. This function can only be used on events of type `EVT_TIMER`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The event has been set to be signaled at the requested time. |
    /// | `EFI_INVALID_PARAMETER` | `Event` or `Type` is not valid. |
    pub SetTimer: unsafe extern "efiapi" fn(
        Event: EFI_EVENT,
        Type: EFI_TIMER_DELAY,
        TriggerTime: UINT64,
    ) -> EFI_STATUS,
    /// Stops execution until an event is signaled.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `NumberOfEvents` | The number of events in the `Event` array. |
    /// | **IN** `Event` | An array of `EFI_EVENT`. |
    /// | **OUT** `Index` | Pointer to the index of the event which satisfied the wait condition. |
    ///
    /// ## Description
    ///
    /// This function must be called at priority level `TPL_APPLICATION`. If an attempt is made to call it at any other
    /// priority level, `EFI_UNSUPPORTED` is returned.
    ///
    /// The list of events in the `Event` array are evaluated in order from first to last, and this evaluation is repeated
    /// until an event is signaled or an error is detected. The following checks are performed on each event in the `Event`
    /// array.
    ///
    /// - If an event is of type `EVT_NOTIFY_SIGNAL`, then `EFI_INVALID_PARAMETER` is returned and Index indicates the event
    /// that caused the failure.
    ///
    /// - If an event is in the signaled state, the signaled state is cleared and `EFI_SUCCESS` is returned, and Index
    /// indicates the event that was signaled.
    ///
    /// - If an event is not in the signaled state but does have a notification function, the notification function is queued
    /// at the event’s notification task priority level. If the execution of the event’s notification function causes the
    /// event to be signaled, then the signaled state is cleared, `EFI_SUCCESS` is returned, and Index indicates the event
    /// that was signaled.
    ///
    /// To wait for a specified time, a timer event must be included in the `Event` array.
    ///
    /// To check if an event is signaled without waiting, an already signaled event can be used as the last event in the
    /// list being checked, or the `CheckEvent()` interface may be used.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The event indicated by `Index` was signaled. |
    /// | `EFI_INVALID_PARAMETER` | `NumberOfEvents` is `0`. |
    /// | `EFI_INVALID_PARAMETER` | The event indicated by Index is of type `EVT_NOTIFY_SIGNAL`. |
    /// | `EFI_UNSUPPORTED` | The current TPL is not `TPL_APPLICATION`. |
    pub WaitForEvent: unsafe extern "efiapi" fn(
        NumberOfEvents: UINTN,
        Event: *mut EFI_EVENT,
        Index: *mut UINTN,
    ) -> EFI_STATUS,
    /// Signals an event.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Event` | The event to signal. |
    ///
    /// ## Description
    ///
    /// The supplied `Event` is placed in the signaled state. If `Event` is already in the signaled state, then `EFI_SUCCESS`
    /// is returned. If Event is of type `EVT_NOTIFY_SIGNAL`, then the event’s notification function is scheduled to be
    /// invoked at the event’s notification task priority level. `SignalEvent()` may be invoked from any task priority
    /// level.
    ///
    /// If the supplied `Event` is a part of an event group, then all of the events in the event group are also signaled
    /// and their notification functions are scheduled.
    ///
    /// When signaling an event group, it is possible to create an event in the group, signal it and then close the event
    /// to remove it from the group.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The event was signaled. |
    pub SignalEvent: unsafe extern "efiapi" fn(Event: EFI_EVENT) -> EFI_STATUS,
    /// Closes an event.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Event` | The event to close. |
    ///
    /// ## Description
    ///
    /// The `CloseEvent()` function removes the caller’s reference to the event, removes it from any event group to which
    /// it belongs, and closes it. Once the event is closed, the event is no longer valid and may not be used on any
    /// subsequent function calls. If Event was registered with `RegisterProtocolNotify()` then `CloseEvent()` will remove
    /// the corresponding registration. It is safe to call `CloseEvent()` within the corresponding notify function.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The event has been closed. |
    pub CloseEvent: unsafe extern "efiapi" fn(Event: EFI_EVENT) -> EFI_STATUS,
    /// Checks whether an event is in the signaled state.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Event` | The event to check. |
    ///
    /// ## Description
    ///
    /// The `CheckEvent()` function checks to see whether Event is in the signaled state. If `Event` is of type
    /// `EVT_NOTIFY_SIGNAL`, then `EFI_INVALID_PARAMETER` is returned. Otherwise, there are three possibilities:
    ///
    /// - If `Event` is in the signaled state, it is cleared and `EFI_SUCCESS` is returned.
    ///
    /// - If `Event` is not in the signaled state and has no notification function, `EFI_NOT_READY` is returned.
    ///
    /// - If `Event` is not in the signaled state but does have a notification function, the notification function is
    /// queued at the event’s notification task priority level. If the execution of the notification function causes
    /// `Event` to be signaled, then the signaled state is cleared and `EFI_SUCCESS` is returned; if the `Event` is not
    /// signaled, then `EFI_NOT_READY` is returned.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The event is in the signaled state. |
    /// | `EFI_NOT_READY` | The event is not in the signaled state. |
    /// | `EFI_INVALID_PARAMETER` | `Event` is of type `EVT_NOTIFY_SIGNAL`. |
    pub CheckEvent: unsafe extern "efiapi" fn(Event: EFI_EVENT) -> EFI_STATUS,
    /// Installs a protocol interface on a device handle. If the handle does not exist, it is created and added to the
    /// list of handles in the system. `InstallMultipleProtocolInterfaces()` performs more error checking than
    /// `InstallProtocolInterface()`, so it is recommended that `InstallMultipleProtocolInterfaces()` be used in place of
    /// `InstallProtocolInterface()`.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN OUT** `Handle` | A pointer to the `EFI_HANDLE` on which the interface is to be installed. If `*Handle` is `NULL` on input, a new handle is created and returned on output. If `*Handle` is not NULL on input, the protocol is added to the handle, and the handle is returned unmodified. If `*Handle` is not a valid handle, then `EFI_INVALID_PARAMETER` is returned. |
    /// | **IN** `Protocol` | The numeric ID of the protocol interface. It is the caller’s responsibility to pass in a valid GUID. |
    /// | **IN** `InterfaceType` | Indicates whether `Interface` is supplied in native form. This value indicates the original execution environment of the request. |
    /// | **IN** `Interface` | A pointer to the protocol interface. The `Interface` must adhere to the structure defined by `Protocol`. `NULL` can be used if a structure is not associated with `Protocol`. |
    ///
    /// ## Description
    ///
    /// The `InstallProtocolInterface()` function installs a protocol interface (a GUID/Protocol Interface structure
    /// pair) on a device handle. The same GUID cannot be installed more than once onto the same handle. If installation
    /// of a duplicate GUID on a handle is attempted, an `EFI_INVALID_PARAMETER` will result.
    ///
    /// Installing a protocol interface allows other components to locate the Handle, and the interfaces installed on it.
    ///
    /// When a protocol interface is installed, the firmware calls all notification functions that have registered to wait
    /// for the installation of `Protocol`. For more information, see the `EFI_BOOT_SERVICES.RegisterProtocolNotify()` function
    /// description.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The protocol interface was installed. |
    /// | `EFI_OUT_OF_RESOURCES` | Space for a new handle could not be allocated. |
    /// | `EFI_INVALID_PARAMETER` | `Handle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Protocol` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `InterfaceType` is not `EFI_NATIVE_INTERFACE`. |
    /// | `EFI_INVALID_PARAMETER` | `Protocol` is already installed on the handle specified by `Handle`. |
    pub InstallProtocolInterface: unsafe extern "efiapi" fn(
        Handle: *mut EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        InterfaceType: EFI_INTERFACE_TYPE,
        Interface: *mut VOID,
    ) -> EFI_STATUS,
    /// Reinstalls a protocol interface on a device handle.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Handle` | Handle on which the interface is to be reinstalled. If `Handle` is not a valid handle, then `EFI_INVALID_PARAMETER` is returned. |
    /// | **IN** `Protocol` | The numeric ID of the interface. It is the caller’s responsibility to pass in a valid GUID. |
    /// | **IN** `InterfaceType` | A pointer to the old interface. `NULL` can be used if a structure is not associated with `Protocol`. |
    /// | **IN** `Interface` | A pointer to the new interface. `NULL` can be used if a structure is not associated with `Protocol`. |
    ///
    /// ## Description
    ///
    /// The `ReinstallProtocolInterface()` function reinstalls a protocol interface on a device handle. The `OldInterface`
    /// for `Protocol` is replaced by the `NewInterface`. `NewInterface` may be the same as `OldInterface`. If it is, the
    /// registered protocol notifies occur for the handle without replacing the interface on the handle.
    ///
    /// As with `InstallProtocolInterface()`, any process that has registered to wait for the installation of the interface
    /// is notified.
    ///
    /// The caller is responsible for ensuring that there are no references to the `OldInterface` that is being removed.
    ///
    /// #### EFI 1.10 Extension
    ///
    /// The extension to this service directly addresses the limitations described in the section above. There may be some
    /// number of drivers currently consuming the protocol interface that is being reinstalled. In this case, it may be
    /// dangerous to replace a protocol interface in the system. It could result in an unstable state, because a driver
    /// may attempt to use the old protocol interface after a new one has been reinstalled. Since the usage of protocol
    /// interfaces is now being tracked for components that use the `EFI_BOOT_SERVICES.OpenProtocol()` and `EFI_BOOT_SERVICES.CloseProtocol()`
    /// boot services, a safe version of this function can be implemented.
    ///
    /// When this function is called, a call is first made to the boot service `UninstallProtocolInterface()`. This will
    /// guarantee that all of the agents are currently consuming the protocol interface `OldInterface` will stop using
    /// `OldInterface`. If `UninstallProtocolInterface()` returns `EFI_ACCESS_DENIED`, then this function returns `EFI_ACCESS_DENIED`,
    /// `OldInterface` remains on `Handle`, and the protocol notifies are not processed because `NewInterface` was never
    /// installed.
    ///
    /// If `UninstallProtocolInterface()` succeeds, then a call is made to the boot service `EFI_BOOT_SERVICES.InstallProtocolInterface()`
    /// to put the `NewInterface` onto `Handle`.
    ///
    /// Finally, the boot service `EFI_BOOT_SERVICES.ConnectController()` is called so all agents that were forced to
    /// release `OldInterface` with `UninstallProtocolInterface()` can now consume the protocol interface `NewInterface`
    /// that was installed with `InstallProtocolInterface()`. After `OldInterface` has been replaced with `NewInterface`,
    /// any process that has registered to wait for the installation of the interface is notified.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The protocol interface was reinstalled. |
    /// | `EFI_NOT_FOUND` | The `OldInterface` on the handle was not found. |
    /// | `EFI_ACCESS_DENIED` | The protocol interface could not be reinstalled, because `OldInterface` is still being used by a driver that will not release it. |
    /// | `EFI_INVALID_PARAMETER` | `Handle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Protocol` is `NULL`. |
    pub ReinstallProtocolInterface: unsafe extern "efiapi" fn(
        Handle: EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        OldInterface: *mut VOID,
        NewInterface: *mut VOID,
    ) -> EFI_STATUS,
    /// Removes a protocol interface from a device handle. It is recommended to use `UninstallMultipleProtocolInterfaces()`
    /// in place of `UninstallProtocolInterface()`.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Handle` | The handle on which the interface was installed. If `Handle` is not a valid handle, then `EFI_INVALID_PARAMETER` is returned. |
    /// | **IN** `Protocol` | The numeric ID of the interface. It is the caller’s responsibility to pass in a valid GUID. |
    /// | **IN** `Interface` | A pointer to the interface. `NULL` can be used if a structure is not associated with `Protocol`. |
    ///
    /// ## Description
    ///
    /// The `UninstallProtocolInterface()` function removes a protocol interface from the handle on which it was previously
    /// installed. The `Protocol` and `Interface` values define the protocol interface to remove from the handle.
    ///
    /// The caller is responsible for ensuring that there are no references to a protocol interface that has been removed.
    /// In some cases, outstanding reference information is not available in the protocol, so the protocol, once added,
    /// cannot be removed. Examples include Console I/O, Block I/O, Disk I/O, and (in general) handles to device protocols.
    ///
    /// If the last protocol interface is removed from a handle, the handle is freed and is no longer valid.
    ///
    /// #### EFI 1.10 Extension
    ///
    /// The extension to this service directly addresses the limitations described in the section above. There may be
    /// some drivers that are currently consuming the protocol interface that needs to be uninstalled, so it may be
    /// dangerous to just blindly remove a protocol interface from the system. Since the usage of protocol interfaces is
    /// now being tracked for components that use the `EFI_BOOT_SERVICES.OpenProtocol()` and `EFI_BOOT_SERVICES.CloseProtocol()`
    /// boot services, a safe version of this function can be implemented. Before the protocol interface is removed, an
    /// attempt is made to force all the drivers that are consuming the protocol interface to stop consuming that protocol
    /// interface. This is done by calling `EFI_BOOT_SERVICES.DisconnectController()` for the driver that currently have
    /// the protocol interface open with an attribute of `EFI_OPEN_PROTOCOL_BY_DRIVER` or `EFI_OPEN_PROTOCOL_BY_DRIVER | EFI_OPEN_PROTOCOL_EXCLUSIVE`.
    ///
    /// If the disconnect succeeds, then those agents will have called the boot service `EFI_BOOT_SERVICES.CloseProtocol()`
    /// to release the protocol interface. Lastly, all of the agents that have the protocol interface open with an attribute
    /// of `EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL`, `EFI_OPEN_PROTOCOL_GET_PROTOCOL`, or `EFI_OPEN_PROTOCOL_TEST_PROTOCOL`
    /// are closed. If there are any agents remaining that still have the protocol interface open, the protocol interface
    /// is not removed from the handle and `EFI_ACCESS_DENIED` is returned. In addition, all of the drivers that were
    /// disconnected with the boot service `DisconnectController()` earlier, are reconnected with the boot service
    /// `EFI_BOOT_SERVICES.ConnectController()`. If there are no agents remaining that are consuming the protocol interface,
    /// then the protocol interface is removed from the handle as described above.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The interface was removed. |
    /// | `EFI_NOT_FOUND` | The interface was not found. |
    /// | `EFI_ACCESS_DENIED` | The interface was not removed because the interface is still being used by a driver. |
    /// | `EFI_INVALID_PARAMETER` | `Handle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Protocol` is `NULL`. |
    pub UninstallProtocolInterface: unsafe extern "efiapi" fn(
        Handle: EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        Interface: *mut VOID,
    ) -> EFI_STATUS,
    /// Queries a handle to determine if it supports a specified protocol.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Handle` | The handle being queried. If `Handle` is `NULL`, then `EFI_INVALID_PARAMETER` is returned. |
    /// | **IN** `Protocol` | The numeric ID of the interface. It is the caller’s responsibility to pass in a valid GUID. |
    /// | **OUT** `Interface` | Supplies the address where a pointer to the corresponding `Protocol` `Interface` is returned. `NULL` will be returned in `*Interface` if a structure is not associated with `Protocol`. |
    ///
    /// ## Description
    ///
    /// The `HandleProtocol()` function queries `Handle` to determine if it supports `Protocol`. If it does, then on return
    /// `Interface` points to a pointer to the corresponding `Protocol` `Interface`. `Interface` can then be passed to any
    /// protocol service to identify the context of the request.
    ///
    /// #### EFI 1.10 Extension
    ///
    /// The `HandleProtocol()` function is still available for use by old EFI applications and drivers. However, all new
    /// applications and drivers should use `EFI_BOOT_SERVICES.OpenProtocol()` in place of `HandleProtocol()`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The interface information for the specified protocol was returned. |
    /// | `EFI_UNSUPPORTED` | The device does not support the specified protocol. |
    /// | `EFI_INVALID_PARAMETER` | `Handle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Protocol` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Interface` is `NULL`. |
    #[deprecated(since = "0.1.0", note = "use the OpenProtocol() function instead")]
    pub HandleProtocol: unsafe extern "efiapi" fn(
        Handle: EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        Interface: *mut *mut VOID,
    ) -> EFI_STATUS,
    #[doc(hidden)]
    pub Reserved: *mut VOID,
    /// Creates an event that is to be signaled whenever an interface is installed for a specified protocol.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Protocol` | The numeric ID of the protocol for which the event is to be registered. |
    /// | **IN** `Event` | Event that is to be signaled whenever a protocol interface is registered for `Protocol`. The same `EFI_EVENT` may be used for multiple protocol notify registrations. |
    /// | **OUT** `Registration` | A pointer to a memory location to receive the registration value. This value must be saved and used by the notification function of Event to retrieve the list of handles that have added a protocol interface of type `Protocol`. |
    ///
    /// ## Description
    ///
    /// The `RegisterProtocolNotify()` function creates an event that is to be signaled whenever a protocol interface is
    /// installed for `Protocol` by `InstallProtocolInterface()` or `EFI_BOOT_SERVICES.ReinstallProtocolInterface()`.
    ///
    /// Once `Event` has been signaled, the `EFI_BOOT_SERVICES.LocateHandle()` function can be called to identify the
    /// newly installed, or reinstalled, handles that support `Protocol`. The `Registration` parameter in `EFI_BOOT_SERVICES.RegisterProtocolNotify()`
    /// corresponds to the `SearchKey` parameter in `LocateHandle()`. Note that the same handle may be returned multiple
    /// times if the handle reinstalls the target protocol ID multiple times. This is typical for removable media devices,
    /// because when such a device reappears, it will reinstall the Block I/O protocol to indicate that the device needs
    /// to be checked again. In response, layered Disk I/O and Simple File System protocols may then reinstall their
    /// protocols to indicate that they can be re-checked, and so forth.
    ///
    /// Events that have been registered for protocol interface notification can be unregistered by calling `CloseEvent()`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The notification event has been registered. |
    /// | `EFI_OUT_OF_RESOURCES` | Space for the notification event could not be allocated. |
    /// | `EFI_INVALID_PARAMETER` | `Protocol` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Event` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Registration` is `NULL`. |
    pub RegisterProtocolNotify: unsafe extern "efiapi" fn(
        Protocol: *mut EFI_GUID,
        Event: EFI_EVENT,
        Registration: *mut *mut VOID,
    ) -> EFI_STATUS,
    /// Returns an array of handles that support a specified protocol.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `SearchType` | Specifies which handle(s) are to be returned. |
    /// | **IN** `Protocol` **OPTIONAL** | Specifies the protocol to search by. This parameter is only valid if `SearchType` is `ByProtocol`. |
    /// | **IN** `SearchKey` **OPTIONAL** | Specifies the search key. This parameter is ignored if `SearchType` is `AllHandles` or `ByProtocol`. If `SearchType` is `ByRegisterNotify`, the parameter must be the `Registration` value returned by function `EFI_BOOT_SERVICES.RegisterProtocolNotify()`. |
    /// | **IN OUT** `BufferSize` | On input, the size in bytes of Buffer. On output, the size in bytes of the array returned in `Buffer` (if the buffer was large enough) or the size, in bytes, of the buffer needed to obtain the array (if the buffer was not large enough). |
    /// | **OUT** `Buffer` | The buffer in which the array is returned. |
    ///
    /// ## Description
    ///
    /// The `LocateHandle()` function returns an array of handles that match the `SearchType` request. If the input value
    /// of `BufferSize` is too small, the function returns `EFI_BUFFER_TOO_SMALL` and updates `BufferSize` to the size of
    /// the buffer needed to obtain the array.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The array of handles was returned. |
    /// | `EFI_NOT_FOUND` | No handles match the search. |
    /// | `EFI_BUFFER_TOO_SMALL` | The `BufferSize` is too small for the result. `BufferSize` has been updated with the size needed to complete the request. |
    /// | `EFI_INVALID_PARAMETER` | `SearchType` is not a member of `EFI_LOCATE_SEARCH_TYPE`. |
    /// | `EFI_INVALID_PARAMETER` | `SearchType` is `ByRegisterNotify` and `SearchKey` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `SearchType` is `ByProtocol` and `Protocol` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | One or more matches are found and `BufferSize` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `BufferSize` is large enough for the result and `Buffer` is `NULL`. |
    pub LocateHandle: unsafe extern "efiapi" fn(
        SearchType: EFI_LOCATE_SEARCH_TYPE,
        Protocol: *mut EFI_GUID,
        SearchKey: *mut VOID,
        BufferSize: UINTN,
        Buffer: *mut EFI_HANDLE,
    ) -> EFI_STATUS,
    /// Locates the handle to a device on the device path that supports the specified protocol.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Protocol` | The protocol to search for. |
    /// | **IN OUT** `DevicePath` | On input, a pointer to a pointer to the device path. On output, the device path pointer is modified to point to the remaining part of the device path–that is, when the function finds the closest handle, it splits the device path into two parts, stripping off the front part, and returning the remaining portion. |
    /// | **OUT** `Device` | A pointer to the returned device handle. |
    ///
    /// ## Description
    ///
    /// The `LocateDevicePath()` function locates all devices on DevicePath that support Protocol and returns the handle
    /// to the device that is closest to `DevicePath`. `DevicePath` is advanced over the device path nodes that were matched.
    ///
    /// This function is useful for locating the proper instance of a protocol interface to use from a logical parent device
    /// driver. For example, a target device driver may issue the request with its own device path and locate the interfaces
    /// to perform I/O on its bus. It can also be used with a device path that contains a file path to strip off the file
    /// system portion of the device path, leaving the file path and handle to the file system driver needed to access the
    /// file.
    ///
    /// If the handle for `DevicePath` supports the protocol (a direct match), the resulting device path is advanced to
    /// the device path terminator node. If `DevicePath` is a multi-instance device path, the function will operate on
    /// the first instance.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The resulting handle was returned. |
    /// | `EFI_NOT_FOUND` | No handles match the search. |
    /// | `EFI_INVALID_PARAMETER` | `Protocol` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `DevicePath` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | A handle matched the search and `Device` is NULL. |
    pub LocateDevicePath: unsafe extern "efiapi" fn(
        Protocol: *mut EFI_GUID,
        DevicePath: *mut *mut EFI_DEVICE_PATH_PROTOCOL,
        Device: *mut EFI_HANDLE,
    ) -> EFI_STATUS,
    /// Adds, updates, or removes a configuration table entry from the EFI System Table.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Guid` | A pointer to the GUID for the entry to add, update, or remove. |
    /// | **IN** `Table` | A pointer to the configuration table for the entry to add, update, or remove. May be `NULL`. |
    ///
    /// ## Description
    ///
    /// The `InstallConfigurationTable()` function is used to maintain the list of configuration tables that are stored
    /// in the EFI System Table. The list is stored as an array of (GUID, Pointer) pairs. The list must be allocated from
    /// pool memory with `PoolType` set to `EfiRuntimeServicesData`.
    ///
    /// If `Guid` is `NULL`, `EFI_INVALID_PARAMETER` is returned. If `Guid` is valid, there are four possibilities:
    ///
    /// - If `Guid` is not present in the System Table, and `Table` is not `NULL`, then the (Guid, Table) pair is added
    /// to the System Table. See Note below.
    ///
    /// - If `Guid` is not present in the System Table, and `Table` is `NULL`, then `EFI_NOT_FOUND` is returned.
    ///
    /// - If `Guid` is present in the System Table, and `Table` is not `NULL`, then the (Guid, Table) pair is updated with
    /// the new `Table` value.
    ///
    /// - If `Guid` is present in the System Table, and `Table` is `NULL`, then the entry associated with `Guid` is removed
    /// from the System Table.
    ///
    /// If an add, modify, or remove operation is completed, then `EFI_SUCCESS` is returned.
    ///
    /// **Note:** If there is not enough memory to perform an add operation, then `EFI_OUT_OF_RESOURCES` is returned.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The (`Guid`, `Table`) pair was added, updated, or removed. |
    /// | `EFI_INVALID_PARAMETER` | `Guid` is `NULL`. |
    /// | `EFI_NOT_FOUND` | An attempt was made to delete a nonexistent entry. |
    /// | `EFI_OUT_OF_RESOURCES` | There is not enough memory available to complete the operation. |
    pub InstallConfigurationTable:
        unsafe extern "efiapi" fn(Guid: *mut EFI_GUID, Table: *mut VOID) -> EFI_STATUS,
    /// Loads an EFI image into memory.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `BootPolicy` | If `TRUE`, indicates that the request originates from the boot manager, and that the boot manager is attempting to load `DevicePath` as a boot selection. Ignored if `SourceBuffer` is not `NULL`. |
    /// | **IN** `ParentImageHandle` | The caller’s image handle. This field is used to initialize the `ParentHandle` field of the EFI Loaded Image Protocol for the image that is being loaded. |
    /// | **IN** `DevicePath` **OPTIONAL** | The `DeviceHandle` specific file path from which the image is loaded. |
    /// | **IN** `SourceBuffer` **OPTIONAL** | If not `NULL`, a pointer to the memory location containing a copy of the image to be loaded. |
    /// | **IN** `SourceSize` | The size in bytes of `SourceBuffer`. Ignored if `SourceBuffer` is `NULL`. |
    /// | **OUT** `ImageHandle` | Pointer to the returned image handle that is created when the image is successfully loaded. |
    ///
    /// ## Description
    ///
    /// The `LoadImage()` function loads an EFI image into memory and returns a handle to the image. The image is loaded
    /// in one of two ways.
    ///
    /// - If `SourceBuffer` is not `NULL`, the function is a memory-to-memory load in which `SourceBuffer` points to the
    /// image to be loaded and `SourceSize` indicates the image’s size in bytes. In this case, the caller has copied the
    /// image into `SourceBuffer` and can free the buffer once loading is complete. The `DevicePath` is optional in this
    /// case. A `DevicePath` should still be provided since certain portions of firmware may use it to make certain security
    /// policy decisions.
    ///
    /// - If SourceBuffer is NULL, the function is a file copy operation that uses the `EFI_SIMPLE_FILE_SYSTEM_PROTOCOL`.
    ///
    /// If there is no instance of `EFI_SIMPLE_FILE_SYSTEM_PROTOCOL` associated with file path, then this function will
    /// attempt to use `EFI_LOAD_FILE_PROTOCOL` (`BootPolicy` is `TRUE`) or `EFI_LOAD_FILE2_PROTOCOL`, and then
    /// `EFI_LOAD_FILE_PROTOCOL` (`BootPolicy` is `FALSE`).
    ///
    /// In all cases, this function will use the instance of these protocols associated with the handle that most closely
    /// matches `DevicePath` will be used. See the boot service description for more information on how the closest handle
    /// is located.
    ///
    /// In the case of `EFI_SIMPLE_FILE_SYSTEM_PROTOCOL`, the path name from the File Path Media Device Path node(s) of
    /// `DevicePath` is used.
    ///
    /// In the case of `EFI_LOAD_FILE_PROTOCOL`, the remaining device path nodes of `DevicePath` and the `BootPolicy`
    /// flag are passed to the `EFI_LOAD_FILE_PROTOCOL` function. The default image responsible for booting is loaded when
    /// `DevicePath` specifies only the device (and there are no further device nodes). For more information see the
    /// discussion of `EFI_LOAD_FILE_PROTOCOL`.
    ///
    /// In the case of `EFI_LOAD_FILE2_PROTOCOL`, the behavior is the same as above, except that it is only used if
    /// `BootOption` is `FALSE`. For more information, see the discussion of the `EFI_LOAD_FILE2_PROTOCOL`.
    ///
    /// If the platform supports driver signing, as specified in Image Execution Information Table and the image signature
    /// is not valid, then information about the image is recorded in the `EFI_IMAGE_EXECUTION_INFO_TABLE` and
    /// `EFI_SECURITY_VIOLATION` is returned.
    ///
    /// If the platform supports user authentication, as described in User Identification and loading of images on the
    /// specified `FilePath` is forbidden in the current user profile, then the information about the image is recorded
    /// (see Deferred Execution in Image Execution Information Table) and `EFI_SECURITY_VIOLATION` is returned.
    ///
    /// Once the image is loaded, firmware creates and returns an `EFI_HANDLE` that identifies the image and supports
    /// EFI Loaded Image Protocol and the `EFI_LOADED_IMAGE_DEVICE_PATH_PROTOCOL`. The caller may fill in the image’s
    /// “load options” data, or add additional protocol support to the handle before passing control to the newly loaded
    /// image by calling `EFI_BOOT_SERVICES.StartImage()`. Also, once the image is loaded, the caller either starts it
    /// by calling `StartImage()` or unloads it by calling `EFI_BOOT_SERVICES.UnloadImage()`.
    ///
    /// Once the image is loaded, `LoadImage()` installs `EFI_HII_PACKAGE_LIST_PROTOCOL` on the handle if the image contains
    /// a custom PE/COFF resource with the type 'HII'. The protocol’s interface pointer points to the HII package list
    /// which is contained in the resource’s data. The format of this is in `EFI_HII_PACKAGE_HEADER`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | Image was loaded into memory correctly. |
    /// | `EFI_NOT_FOUND` | Both `SourceBuffer` and `DevicePath` are `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | One of the parameters has an invalid value. |
    /// | `EFI_INVALID_PARAMETER` | `ImageHandle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `ParentImageHandle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `ParentImageHandle` is `NULL`. |
    /// | `EFI_UNSUPPORTED` | The image type is not supported. |
    /// | `EFI_OUT_OF_RESOURCES` | Image was not loaded due to insufficient resources. |
    /// | `EFI_LOAD_ERROR` | Image was not loaded because the image format was corrupt or not understood. |
    /// | `EFI_DEVICE_ERROR` | Image was not loaded because the device returned a read error. |
    /// | `EFI_ACCESS_DENIED` | Image was not loaded because the platform policy prohibits the image from being loaded. `NULL` is returned in `ImageHandle`. |
    /// | `EFI_SECURITY_VIOLATION` | Image was loaded and an `ImageHandle` was created with a valid `EFI_LOADED_IMAGE_PROTOCOL`. However, the current platform policy specifies that the image should not be started. |
    pub LoadImage: unsafe extern "efiapi" fn(
        BootPolicy: BOOLEAN,
        ParentImageHandle: EFI_HANDLE,
        DevicePath: *mut EFI_DEVICE_PATH_PROTOCOL,
        SourceBuffer: *mut VOID,
        SourceSize: UINTN,
        ImageHandle: *mut EFI_HANDLE,
    ) -> EFI_STATUS,
    /// Transfers control to a loaded image’s entry point.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `ImageHandle` | Handle of image to be started. |
    /// | **OUT** `ExitDataSize` | Pointer to the size, in bytes, of `ExitData`. If `ExitData` is `NULL`, then this parameter is ignored and the contents of `ExitDataSize` are not modified. |
    /// | **OUT** `ExitData` **OPTIONAL** | Pointer to a pointer to a data buffer that includes a null-terminated string, optionally followed by additional binary data. The string is a description that the caller may use to further indicate the reason for the image’s exit. |
    ///
    /// ## Description
    ///
    /// The `StartImage()` function transfers control to the entry point of an image that was loaded by `EFI_BOOT_SERVICES.LoadImage()`.
    /// The image may only be started one time.
    ///
    /// Control returns from `StartImage()` when the loaded image’s `EFI_IMAGE_ENTRY_POINT` returns or when the loaded
    /// image calls `EFI_BOOT_SERVICES.Exit()` When that call is made, the `ExitData` buffer and `ExitDataSize` from `Exit()`
    /// are passed back through the `ExitData` buffer and `ExitDataSize` in this function. The caller of this function is
    /// responsible for returning the `ExitData` buffer to the pool by calling `EFI_BOOT_SERVICES.FreePool()` when the buffer
    /// is no longer needed. Using `Exit()` is similar to returning from the image’s `EFI_IMAGE_ENTRY_POINT` except that
    /// `Exit()` may also return additional `ExitData`. `Exit()` function description defines clean up procedure performed
    /// by the firmware once loaded image returns control.
    ///
    /// #### EFI 1.10 Extension
    ///
    /// To maintain compatibility with UEFI drivers that are written to the EFI 1.02 Specification, `StartImage()` must
    /// monitor the handle database before and after each image is started. If any handles are created or modified when
    /// an image is started, then `EFI_BOOT_SERVICES.ConnectController()` must be called with the `Recursive` parameter
    /// set to `TRUE` for each of the newly created or modified handles before `StartImage()` returns.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_INVALID_PARAMETER` | `ImageHandle` is either an invalid image handle or the image has already been initialized with `StartImage`. |
    /// | `EFI_SECURITY_VIOLATION` | The current platform policy specifies that the image should not be started. |
    /// | exit code from image | Exit code from image. |
    pub StartImage: unsafe extern "efiapi" fn(
        ImageHandle: EFI_HANDLE,
        ExitDataSize: *mut UINTN,
        ExitData: *mut *mut CHAR16,
    ) -> EFI_STATUS,
    /// Terminates a loaded EFI image and returns control to boot services.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `ImageHandle` | Handle that identifies the image. This parameter is passed to the image on entry. |
    /// | **IN** `ExitStatus` | The image’s exit code. |
    /// | **IN** `ExitDataSize` | The size, in bytes, of `ExitData`. Ignored if `ExitStatus` is `EFI_SUCCESS`. |
    /// | **IN** `ExitData` **OPTIONAL** | Pointer to a data buffer that includes a null-terminated string, optionally followed by additional binary data. The string is a description that the caller may use to further indicate the reason for the image’s exit. `ExitData` is only valid if `ExitStatus` is something other than `EFI_SUCCESS`. The `ExitData` buffer must be allocated by calling `EFI_BOOT_SERVICES.AllocatePool()`. |
    ///
    /// ## Description
    ///
    /// The `Exit()` function terminates the image referenced by `ImageHandle` and returns control to boot services. This
    /// function may not be called if the image has already returned from its entry point (`EFI_IMAGE_ENTRY_POINT`) or if
    /// it has loaded any child images that have not exited (all child images must exit before this image can exit).
    ///
    /// Using `Exit()` is similar to returning from the image’s `EFI_IMAGE_ENTRY_POINT` except that `Exit()` may also
    /// return additional `ExitData`.
    ///
    /// When an application exits a compliant system, firmware frees the memory used to hold the image. The firmware also
    /// frees its references to the `ImageHandle` and the handle itself. Before exiting, the application is responsible
    /// for freeing any resources it allocated. This includes memory (pages and/or pool), open file system handles, and
    /// so forth. The only exception to this rule is the `ExitData` buffer, which must be freed by the caller of
    /// `EFI_BOOT_SERVICES.StartImage()`. (If the buffer is needed, firmware must allocate it by calling
    /// `EFI_BOOT_SERVICES.AllocatePool()` and must return a pointer to it to the caller of `StartImage()`.)
    ///
    /// When an EFI boot service driver or runtime service driver exits, firmware frees the image only if the `ExitStatus`
    /// is an error code; otherwise the image stays resident in memory. The driver must not return an error code if it
    /// has installed any protocol handlers or other active callbacks into the system that have not (or cannot) be cleaned
    /// up. If the driver exits with an error code, it is responsible for freeing all resources before exiting. This includes
    /// any allocated memory (pages and/or pool), open file system handles, and so forth.
    ///
    /// It is valid to call `Exit()` or `UnloadImage()` for an image that was loaded by `EFI_BOOT_SERVICES.LoadImage()`
    /// before calling `EFI_BOOT_SERVICES.StartImage()`. This will free the image from memory without having started it.
    ///
    /// #### EFI 1.10 Extension
    ///
    /// If `ImageHandle` is a UEFI application, then all of the protocols that were opened by `ImageHandle` using the boot
    /// service `EFI_BOOT_SERVICES.OpenProtocol()` are automatically closed with the boot service `EFI_BOOT_SERVICES.CloseProtocol()`.
    /// If `ImageHandle` is a UEFI boot service driver or UEFI runtime service driver, and `ExitStatus` is an error code,
    /// then all of the protocols that were opened by `ImageHandle` using the boot service `OpenProtocol()` are automatically
    /// closed with the boot service `CloseProtocol()`. If `ImageHandle` is a UEFI boot service driver or UEFI runtime service
    /// driver, and `ExitStatus` is not an error code, then no protocols are automatically closed by this service.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The image specified by `ImageHandle` was unloaded. This condition only occurs for images that have been loaded with `LoadImage()` but have not been started with `StartImage()`. |
    /// | `EFI_INVALID_PARAMETER` | The image specified by `ImageHandle` has been loaded and started with `LoadImage()` and `StartImage()`, but the image is not the currently executing image. |
    /// | does not return | Image exit. Control is returned to the `StartImage()` call that invoked the image specified by `ImageHandle`. |
    pub Exit: unsafe extern "efiapi" fn(
        ImageHandle: EFI_HANDLE,
        ExitStatus: EFI_STATUS,
        ExitDataSize: UINTN,
        ExitData: *mut CHAR16,
    ) -> EFI_STATUS,
    /// Unloads an image.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `ImageHandle` | Handle that identifies the image to be unloaded. |
    ///
    /// ## Description
    ///
    /// The `UnloadImage()` function unloads a previously loaded image.
    ///
    /// There are three possible scenarios. If the image has not been started, the function unloads the image and returns
    /// `EFI_SUCCESS`.
    ///
    /// If the image has been started and has an `Unload()` entry point, control is passed to that entry point. If the
    /// image’s unload function returns `EFI_SUCCESS`, the image is unloaded; otherwise, the error returned by the image’s
    /// unload function is returned to the caller. The image unload function is responsible for freeing all allocated memory
    /// and ensuring that there are no references to any freed memory, or to the image itself, before returning `EFI_SUCCESS`.
    ///
    /// If the image has been started and does not have an `Unload()` entry point, the function returns `EFI_UNSUPPORTED`.
    ///
    /// #### EFI 1.10 Extension
    ///
    /// All of the protocols that were opened by `ImageHandle` using the boot service `EFI_BOOT_SERVICES.OpenProtocol()`
    /// are automatically closed with the boot service `EFI_BOOT_SERVICES.CloseProtocol()`. If all of the open protocols
    /// are closed, then `EFI_SUCCESS` is returned. If any call to `CloseProtocol()` fails, then the error code from
    /// `CloseProtocol()` is returned.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The image has been unloaded. |
    /// | `EFI_UNSUPPORTED` | The image has been started, but does not support unload. |
    /// | `EFI_INVALID_PARAMETER` | `ImageHandle` is not a valid image handle. |
    /// | exit code from unload handler | Exit code from the image’s unload function. |
    pub UnloadImage: unsafe extern "efiapi" fn(ImageHandle: EFI_HANDLE) -> EFI_STATUS,
    /// Terminates all boot services.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `ImageHandle` | Handle that identifies the exiting image. |
    /// | **IN** `MapKey` | Key to the latest memory map. |
    ///
    /// ## Description
    ///
    /// The `ExitBootServices()` function is called by the currently executing UEFI OS loader image to terminate all boot
    /// services. On success, the UEFI OS loader becomes responsible for the continued operation of the system. All events
    /// from the `EFI_EVENT_GROUP_BEFORE_EXIT_BOOT_SERVICES` and `EFI_EVENT_GROUP_EXIT_BOOT_SERVICES` event notification
    /// groups as well as events of type `EVT_SIGNAL_EXIT_BOOT_SERVICES` must be signaled before `ExitBootServices()`
    /// returns `EFI_SUCCESS`. The events are only signaled once even if `ExitBootServices()` is called multiple times.
    ///
    /// A UEFI OS loader must ensure that it has the system’s current memory map at the time it calls `ExitBootServices()`.
    /// This is done by passing in the current memory map’s `MapKey` value as returned by `EFI_BOOT_SERVICES.GetMemoryMap()`.
    /// Care must be taken to ensure that the memory map does not change between these two calls. It is suggested that
    /// `GetMemoryMap()` be called immediately before calling `ExitBootServices()`. If `MapKey` value is incorrect,
    /// `ExitBootServices()` returns `EFI_INVALID_PARAMETER` and `GetMemoryMap()` with `ExitBootServices()` must be called
    /// again. Firmware implementation may choose to do a partial shutdown of the boot services during the first call to
    /// `ExitBootServices()`. A UEFI OS loader should not make calls to any boot service function other than Memory Allocation
    /// Services after the first call to `ExitBootServices()`.
    ///
    /// On success, the UEFI OS loader owns all available memory in the system. In addition, the UEFI OS loader can treat
    /// all memory in the map marked as `EfiBootServicesCode` and `EfiBootServicesData` as available free memory. No further
    /// calls to boot service functions or EFI device-handle-based protocols may be used, and the boot services watchdog
    /// timer is disabled. On success, several fields of the EFI System Table should be set to NULL. These include
    /// `ConsoleInHandle`, `ConIn`, `ConsoleOutHandle`, `ConOut`, `StandardErrorHandle`, `StdErr`, and `BootServicesTable`.
    /// In addition, since fields of the EFI System Table are being modified, the 32-bit CRC for the EFI System Table must
    /// be recomputed.
    ///
    /// Firmware must guarantee the following order of processing:
    ///
    /// - `EFI_EVENT_GROUP_BEFORE_EXIT_BOOT_SERVICES` handlers are called.
    ///
    /// - Timer services are deactivated (timer event activity stopped).
    ///
    /// - `EVT_SIGNAL_EXIT_BOOT_SERVICES` and `EFI_EVENT_GROUP_BEFORE_EXIT_BOOT_SERVICES` handlers are called.
    ///
    /// **Note:** The `EVT_SIGNAL_EXIT_BOOT_SERVICES` event type and `EFI_EVENT_GROUP_BEFORE_EXIT_BOOT_SERVICES` event
    /// group are functionally equivalent. Firmware does not distinguish between the two while ordering the handlers.
    ///
    /// Refer to `EFI_EVENT_GROUP_EXIT_BOOT_SERVICES` description in the `EFI_BOOT_SERVICES.CreateEventEx()` section for
    /// additional restrictions on `EXIT_BOOT_SERVICES` handlers.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | Boot services have been terminated. |
    /// | `EFI_INVALID_PARAMETER` | `MapKey` is incorrect. |
    pub ExitBootServices:
        unsafe extern "efiapi" fn(ImageHandle: EFI_HANDLE, MapKey: UINTN) -> EFI_STATUS,
    /// Returns a monotonically increasing count for the platform.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **OUT** `Count` | Pointer to returned value. |
    ///
    /// ## Description
    ///
    /// The `GetNextMonotonicCount()` function returns a 64-bit value that is numerically larger then the last time the
    /// function was called.
    ///
    /// The platform’s monotonic counter is comprised of two parts: the high 32 bits and the low 32 bits. The low 32-bit
    /// value is volatile and is reset to zero on every system reset. It is increased by 1 on every call to `GetNextMonotonicCount()`.
    /// The high 32-bit value is nonvolatile and is increased by one on whenever the system resets or the low 32-bit counter
    /// overflows.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The next monotonic count was returned. |
    /// | `EFI_DEVICE_ERROR` | The device is not functioning properly. |
    /// | `EFI_INVALID_PARAMETER` | `Count` is `NULL`. |
    pub GetNextMonotonicCount: unsafe extern "efiapi" fn(Count: *mut UINT64) -> EFI_STATUS,
    /// Induces a fine-grained stall.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Microseconds` | The number of microseconds to stall execution. |
    ///
    /// ## Description
    ///
    /// The `Stall()` function stalls execution on the processor for at least the requested number of microseconds.
    /// Execution of the processor is not yielded for the duration of the stall.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | Execution was stalled at least the requested number of `Microseconds`. |
    pub Stall: unsafe extern "efiapi" fn(Microseconds: UINTN) -> EFI_STATUS,
    /// Sets the system’s watchdog timer.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Timeout` | The number of seconds to set the watchdog timer to. A value of zero disables the timer. |
    /// | **IN** `WatchdogCode` | The numeric code to log on a watchdog timer timeout event. The firmware reserves codes `0x0000` to `0xFFFF`. Loaders and operating systems may use other timeout codes. |
    /// | **IN** `DataSize` | The size, in bytes, of `WatchdogData`. |
    /// | **IN** `WatchdogData` **OPTIONAL** | A data buffer that includes a null-terminated string, optionally followed by additional binary data. The string is a description that the call may use to further indicate the reason to be logged with a watchdog event. |
    ///
    /// ## Description
    ///
    /// The `SetWatchdogTimer()` function sets the system’s watchdog timer.
    ///
    /// If the watchdog timer expires, the event is logged by the firmware. The system may then either reset with the
    /// Runtime Service `ResetSystem()` or perform a platform specific action that must eventually cause the platform to
    /// be reset. The watchdog timer is armed before the firmware’s boot manager invokes an EFI boot option. The watchdog
    /// must be set to a period of 5 minutes. The EFI Image may reset or disable the watchdog timer as needed. If control
    /// is returned to the firmware’s boot manager, the watchdog timer must be disabled.
    ///
    /// The watchdog timer is only used during boot services. On successful completion of `EFI_BOOT_SERVICES.ExitBootServices()`
    /// the watchdog timer is disabled.
    ///
    /// The accuracy of the watchdog timer is +/- 1 second from the requested `Timeout`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The timeout has been set. |
    /// | `EFI_INVALID_PARAMETER` | The supplied `WatchdogCode` is invalid. |
    /// | `EFI_UNSUPPORTED` | The system does not have a watchdog timer. |
    /// | `EFI_DEVICE_ERROR` | The watch dog timer could not be programmed due to a hardware error. |
    pub SetWatchdogTimer: unsafe extern "efiapi" fn(
        Timeout: UINTN,
        WatchdogCode: UINT64,
        DataSize: UINTN,
        WatchdogData: *mut CHAR16,
    ) -> EFI_STATUS,
    /// Connects one or more drivers to a controller.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `ControllerHandle` | The handle of the controller to which driver(s) are to be connected. |
    /// | **IN** `DriverImageHandle` **OPTIONAL** | A pointer to an ordered list handles that support the `EFI_DRIVER_BINDING_PROTOCOL`. The list is terminated by a `NULL` handle value. These handles are candidates for the Driver Binding Protocol(s) that will manage the controller specified by `ControllerHandle`. This is an optional parameter that may be `NULL`. This parameter is typically used to debug new drivers. |
    /// | **IN** `RemainingDevicePath` **OPTIONAL** | A pointer to the device path that specifies a child of the controller specified by `ControllerHandle`. This is an optional parameter that may be `NULL`. If it is `NULL`, then handles for all the children of `ControllerHandle` will be created. This parameter is passed unchanged to the `EFI_DRIVER_BINDING_PROTOCOL.Supported()` and `EFI_DRIVER_BINDING_PROTOCOL.Start()` services of the `EFI_DRIVER_BINDING_PROTOCOL` attached to `ControllerHandle`. |
    /// | **IN** `Recursive` | If `TRUE`, then `ConnectController()` is called recursively until the entire tree of controllers below the controller specified by `ControllerHandle` have been created. If `FALSE`, then the tree of controllers is only expanded one level. |
    ///
    /// ## Description
    ///
    /// This function connects one or more drivers to the controller specified by `ControllerHandle`. If `ControllerHandle`
    /// is `NULL`, then `EFI_INVALID_PARAMETER` is returned. If there are no `EFI_DRIVER_BINDING_PROTOCOL` instances present
    /// in the system, then return `EFI_NOT_FOUND`. If there are not enough resources available to complete this function,
    /// then `EFI_OUT_OF_RESOURCES` is returned.
    ///
    /// If the platform supports user authentication, as specified in User Identification the device path associated with
    /// `ControllerHandle` is checked against the connect permissions in the current user profile. If forbidden, then
    /// `EFI_SECURITY_VIOLATION` is returned. Then, before connecting any of the `DriverImageHandles`, the device path
    /// associated with the handle is checked against the connect permissions in the current user profile.
    ///
    /// If `Recursive` is `FALSE`, then this function returns after all drivers have been connected to `ControllerHandle`.
    /// If `Recursive` is `TRUE`, then `ConnectController()` is called recursively on all of the child controllers of
    /// `ControllerHandle`. The child controllers can be identified by searching the handle database for all the controllers
    /// that have opened `ControllerHandle` with an attribute of `EFI_OPEN_PROTOCOL_BY_CHILD_CONTROLLER`.
    ///
    /// This functions uses five precedence rules when deciding the order that drivers are tested against controllers.
    /// These five rules from highest precedence to lowest precedence are as follows:
    ///
    /// 1. Context Override: `DriverImageHandle` is an ordered list of handles that support the `EFI_DRIVER_BINDING_PROTOCOL`.
    /// The highest priority image handle is the first element of the list, and the lowest priority image handle is the
    /// last element of the list. The list is terminated with a `NULL` image handle.
    ///
    /// 2. Platform Driver Override: If an `EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL` instance is present in the system,
    /// then the EFI Platform Driver Override Protocol service of this protocol is used to retrieve an ordered list of
    /// image handles for `ControllerHandle`. From this list, the image handles found in rule (1) above are removed.
    /// The first image handle returned from `GetDriver()` has the highest precedence, and the last image handle returned
    /// from `GetDriver()` has the lowest precedence. The ordered list is terminated when `GetDriver()` returns `EFI_NOT_FOUND`.
    /// It is legal for no image handles to be returned by `GetDriver()`. There can be at most a single instance in the
    /// system of the `EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL`. If there is more than one, then the system behavior is
    /// not deterministic.
    ///
    /// 3. Driver Family Override Search: The list of available driver image handles can be found by using the boot
    /// service `EFI_BOOT_SERVICES.LocateHandle()` with a SearchType of `ByProtocol` for the GUID of the `EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL`.
    /// From this list, the image handles found in rules (1), and (2) above are removed. The remaining image handles are
    /// sorted from highest to lowest based on the value returned from the `GetVersion()` function of the `EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL`
    /// associated with each image handle.
    ///
    /// 4. Bus Specific Driver Override: If there is an instance of the `EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL` attached
    /// to `ControllerHandle`, then the EFI Platform Driver Override Protocol service of this protocol is used to retrieve
    /// an ordered list of image handle for `ControllerHandle`. From this list, the image handles found in rules (1), (2),
    /// and (3) above are removed. The first image handle returned from `GetDriver()` has the highest precedence, and the
    /// last image handle returned from `GetDriver()` has the lowest precedence. The ordered list is terminated when `GetDriver()`
    /// returns `EFI_NOT_FOUND`. It is legal for no image handles to be returned by `GetDriver()`.
    ///
    /// 5. Driver Binding Search: The list of available driver image handles can be found by using the boot service
    /// `EFI_BOOT_SERVICES.LocateHandle()` with a SearchType of ByProtocol for the GUID of the `EFI_DRIVER_BINDING_PROTOCOL`.
    /// From this list, the image handles found in rules (1), (2), (3), and (4) above are removed. The remaining image
    /// handles are sorted from highest to lowest based on the Version field of the `EFI_DRIVER_BINDING_PROTOCOL`
    /// instance associated with each image handle.
    ///
    /// Each of the five groups of image handles listed above is tested against ControllerHandle in order by using the
    /// `EFI_DRIVER_BINDING_PROTOCOL.Supported()`. `RemainingDevicePath` is passed into `Supported()` unmodified. The
    /// first image handle whose `Supported()` service returns `EFI_SUCCESS` is marked so the image handle will not be
    /// tried again during this call to `ConnectController()`. Then, `EFI_DRIVER_BINDING_PROTOCOL.Start()` service of the
    /// `EFI_DRIVER_BINDING_PROTOCOL` is called for `ControllerHandle`. Once again, `RemainingDevicePath` is passed in
    /// unmodified. Every time `Supported()` returns `EFI_SUCCESS`, the search for drivers restarts with the highest
    /// precedence image handle. This process is repeated until no image handles pass the `Supported()` check.
    ///
    /// If at least one image handle returned `EFI_SUCCESS` from its `Start()` service, then `EFI_SUCCESS` is returned.
    ///
    /// If no image handles returned `EFI_SUCCESS` from their `Start()` service then `EFI_NOT_FOUND` is returned unless
    /// `RemainingDevicePath` is not `NULL`, and `RemainingDevicePath` is an End Node. In this special case, `EFI_SUCCESS`
    /// is returned because it is not an error to fail to start a child controller that is specified by an End Device Path
    /// Node.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | One or more drivers were connected to `ControllerHandle`. |
    /// | `EFI_SUCCESS` | No drivers were connected to `ControllerHandle`, but `RemainingDevicePath` is not `NULL`, and it is an End Device Path Node. |
    /// | `EFI_INVALID_PARAMETER` | `ControllerHandle` is `NULL`. |
    /// | `EFI_NOT_FOUND` | There are no `EFI_DRIVER_BINDING_PROTOCOL` instances present in the system. |
    /// | `EFI_NOT_FOUND` | No drivers were connected to `ControllerHandle`. |
    /// | `EFI_SECURITY_VIOLATION` | The user has no permission to start UEFI device drivers on the device path associated with the `ControllerHandle` or specified by the `RemainingDevicePath`. |
    pub ConnectController: unsafe extern "efiapi" fn(
        ControllerHandle: EFI_HANDLE,
        DriverImageHandle: *mut EFI_HANDLE,
        RemainingDevicePath: *mut EFI_DEVICE_PATH_PROTOCOL,
        Recursive: BOOLEAN,
    ) -> EFI_STATUS,
    /// Disconnects one or more drivers from a controller.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `ControllerHandle` | The handle of the controller from which driver(s) are to be disconnected. |
    /// | **IN** `DriverImageHandle` **OPTIONAL** | The driver to disconnect from `ControllerHandle`. If `DriverImageHandle` is `NULL`, then all the drivers currently managing `ControllerHandle` are disconnected from `ControllerHandle`. |
    /// | **IN** `ChildHandle` **OPTIONAL** | The handle of the child to destroy. If `ChildHandle` is `NULL`, then all the children of `ControllerHandle` are destroyed before the drivers are disconnected from `ControllerHandle`. |
    ///
    /// ## Description
    ///
    /// This function disconnects one or more drivers from the controller specified by `ControllerHandle`. If `DriverImageHandle`
    /// is `NULL`, then all of the drivers currently managing `ControllerHandle` are disconnected from `ControllerHandle`.
    /// If `DriverImageHandle` is not `NULL`, then only the driver specified by `DriverImageHandle` is disconnected from
    /// `ControllerHandle`. If `ChildHandle` is `NULL`, then all of the children of `ControllerHandle` are destroyed before
    /// the drivers are disconnected from `ControllerHandle`. If `ChildHandle` is not `NULL`, then only the child controller
    /// specified by `ChildHandle` is destroyed. If `ChildHandle` is the only child of `ControllerHandle`, then the driver
    /// specified by `DriverImageHandle` will be disconnected from `ControllerHandle`. A driver is disconnected from a
    /// controller by calling the `Stop()` service of the `EFI_DRIVER_BINDING_PROTOCOL`. The `EFI_DRIVER_BINDING_PROTOCOL`
    /// is on the driver image handle, and the handle of the controller is passed into the `Stop()` service. The list of
    /// drivers managing a controller, and the list of children for a specific controller can be retrieved from the handle
    /// database with the boot service `EFI_BOOT_SERVICES.OpenProtocolInformation()`. If all the required drivers are
    /// disconnected from ControllerHandle, then `EFI_SUCCESS` is returned.
    ///
    /// If `ControllerHandle` is `NULL`, then `EFI_INVALID_PARAMETER` is returned. If no drivers are managing `ControllerHandle`,
    /// then `EFI_SUCCESS` is returned. If `DriverImageHandle` is not `NULL`, and `DriverImageHandle` is not a valid `EFI_HANDLE`,
    /// then `EFI_INVALID_PARAMETER` is returned. If `DriverImageHandle` is not `NULL`, and `DriverImageHandle` is not
    /// currently managing `ControllerHandle`, then `EFI_SUCCESS` is returned. If `ChildHandle` is not `NULL`, and `ChildHandle`
    /// is not a valid `EFI_HANDLE`, then `EFI_INVALID_PARAMETER` is returned. If there are not enough resources available
    /// to disconnect drivers from `ControllerHandle`, then `EFI_OUT_OF_RESOURCES` is returned.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | One or more drivers were disconnected from the controller. |
    /// | `EFI_SUCCESS` | On entry, no drivers are managing `ControllerHandle`. |
    /// | `EFI_SUCCESS` | `DriverImageHandle` is not `NULL`, and on entry `DriverImageHandle` is not managing `ControllerHandle`. |
    /// | `EFI_INVALID_PARAMETER` | `ControllerHandle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `DriverImageHandle` is not `NULL`, and it is not a valid `EFI_HANDLE`. |
    /// | `EFI_INVALID_PARAMETER` | `ChildHandle` is not `NULL`, and it is not a valid `EFI_HANDLE`. |
    /// | `EFI_OUT_OF_RESOURCES` | There are not enough resources available to disconnect any drivers from `ControllerHandle`. |
    /// | `EFI_DEVICE_ERROR` | The controller could not be disconnected because of a device error. |
    /// | `EFI_INVALID_PARAMETER` | `DriverImageHandle` does not support the `EFI_DRIVER_BINDING_PROTOCOL`. |
    pub DisconnectController: unsafe extern "efiapi" fn(
        ControllerHandle: EFI_HANDLE,
        DriverImageHandle: EFI_HANDLE,
        ChildHandle: EFI_HANDLE,
    ) -> EFI_STATUS,
    /// Queries a handle to determine if it supports a specified protocol. If the protocol is supported by the handle,
    /// it opens the protocol on behalf of the calling agent. This is an extended version of the EFI boot service
    /// `EFI_BOOT_SERVICES.HandleProtocol()`.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Handle` | The handle for the protocol interface that is being opened. |
    /// | **IN** `Protocol` | The published unique identifier of the protocol. It is the caller’s responsibility to pass in a valid GUID. |
    /// | **OUT** `Interface` **OPTIONAL** | Supplies the address where a pointer to the corresponding `Protocol` `Interface` is returned. `NULL` will be returned in `*Interface` if a structure is not associated with `Protocol`. This parameter is optional, and will be ignored if `Attributes` is `EFI_OPEN_PROTOCOL_TEST_PROTOCOL`. |
    /// | **IN** `AgentHandle` | The handle of the agent that is opening the protocol interface specified by `Protocol` and `Interface`. For agents that follow the UEFI Driver Model, this parameter is the handle that contains the `EFI_DRIVER_BINDING_PROTOCOL` instance that is produced by the UEFI driver that is opening the protocol interface. For UEFI applications, this is the image handle of the UEFI application that is opening the protocol interface. For applications that use `HandleProtocol()` to open a protocol interface, this parameter is the image handle of the EFI firmware. |
    /// | **IN** `ControllerHandle` | If the agent that is opening a protocol is a driver that follows the UEFI Driver Model, then this parameter is the controller handle that requires the protocol interface. If the agent does not follow the UEFI Driver Model , then this parameter is optional and may be `NULL`. |
    /// | **IN** `Attributes` | The open mode of the protocol interface specified by `Handle` and `Protocol`. |
    ///
    /// ## Description
    ///
    /// This function opens a protocol interface on the handle specified by `Handle` for the protocol specified by `Protocol`.
    /// The first three parameters are the same as `EFI_BOOT_SERVICES.HandleProtocol()`. The only difference is that the
    /// agent that is opening a protocol interface is tracked in an EFI’s internal handle database. The tracking is used'
    /// by the UEFI Driver Model, and also used to determine if it is safe to uninstall or reinstall a protocol interface.
    ///
    /// The agent that is opening the protocol interface is specified by `AgentHandle`, `ControllerHandle`, and `Attributes`.
    /// If the protocol interface can be opened, then `AgentHandle`, `ControllerHandle`, and `Attributes` are added to the
    /// list of agents that are consuming the protocol interface specified by `Handle` and `Protocol`. In addition, the
    /// protocol interface is returned in `Interface`, and `EFI_SUCCESS` is returned. If Attributes is `TEST_PROTOCOL`,
    /// then `Interface` is optional, and can be `NULL`.
    ///
    /// There are a number of reasons that this function call can return an error. If an error is returned, then
    /// `AgentHandle`, `ControllerHandle`, and `Attributes` are not added to the list of agents consuming the protocol
    /// interface specified by `Handle` and `Protocol`. Interface is returned unmodified for all error conditions except
    /// `EFI_UNSUPPORTED` and `EFI_ALREADY_STARTED`, `NULL` will be returned in `*Interface` when `EFI_UNSUPPORTED` and
    /// `Attributes` is not `EFI_OPEN_PROTOCOL_TEST_PROTOCOL`, the protocol interface will be returned in `*Interface`
    /// when `EFI_ALREADY_STARTED` is returned.
    ///
    /// The following is the list of conditions that must be checked before this function can return `EFI_SUCCESS`:
    ///
    /// - If `Protocol` is `NULL`, then `EFI_INVALID_PARAMETER` is returned.
    ///
    /// - If `Interface` is `NULL` and `Attributes` is not `TEST_PROTOCOL`, then `EFI_INVALID_PARAMETER` is returned.
    ///
    /// - If `Handle` is `NULL`, then `EFI_INVALID_PARAMETER` is returned.
    ///
    /// - If `Handle` does not support `Protocol`, then `EFI_UNSUPPORTED` is returned.
    ///
    /// - If `Attributes` is not a legal value, then `EFI_INVALID_PARAMETER` is returned.
    ///
    /// - If `Attributes` is `BY_CHILD_CONTROLLER`, `BY_DRIVER`, `EXCLUSIVE`, or `BY_DRIVER|EXCLUSIVE`, and `AgentHandle`
    /// is `NULL`, then `EFI_INVALID_PARAMETER` is returned.
    ///
    /// - If `Attributes` is `BY_CHILD_CONTROLLER`, `BY_DRIVER`, or `BY_DRIVER|EXCLUSIVE`, and `ControllerHandle` is `NULL`,
    /// then `EFI_INVALID_PARAMETER` is returned.
    ///
    /// - If `Attributes` is `BY_CHILD_CONTROLLER` and `Handle` is identical to `ControllerHandle`, then `EFI_INVALID_PARAMETER`
    /// is returned.
    ///
    /// - If `Attributes` is `BY_DRIVER`, `BY_DRIVER|EXCLUSIVE`, or `EXCLUSIVE`, and there are any items on the open list
    /// of the protocol interface with an attribute of `EXCLUSIVE` or `BY_DRIVER|EXCLUSIVE`, then `EFI_ACCESS_DENIED` is
    /// returned.
    ///
    /// - If `Attributes` is `BY_DRIVER`, and there are any items on the open list of the protocol interface with an
    /// attribute of `BY_DRIVER`, and `AgentHandle` is the same agent handle in the open list item, then `EFI_ALREADY_STARTED`
    /// is returned.
    ///
    /// - If `Attributes` is `BY_DRIVER`, and there are any items on the open list of the protocol interface with an
    /// attribute of `BY_DRIVER`, and `AgentHandle` is different than the agent handle in the open list item, then
    /// `EFI_ACCESS_DENIED` is returned.
    ///
    /// - If `Attributes` is `BY_DRIVER|EXCLUSIVE`, and there are any items on the open list of the protocol interface
    /// with an attribute of `BY_DRIVER|EXCLUSIVE`, and `AgentHandle` is the same agent handle in the open list item, then
    /// `EFI_ALREADY_STARTED` is returned.
    ///
    /// - If `Attributes` is `BY_DRIVER|EXCLUSIVE`, and there are any items on the open list of the protocol interface
    /// with an attribute of `BY_DRIVER|EXCLUSIVE`, and `AgentHandle` is different than the agent handle in the open list
    /// item, then `EFI_ACCESS_DENIED` is returned.
    ///
    /// - If `Attributes` is `BY_DRIVER|EXCLUSIVE` or `EXCLUSIVE`, and there is an item on the open list of the protocol
    /// interface with an attribute of `BY_DRIVER`, then the boot service `EFI_BOOT_SERVICES.DisconnectController()` is
    /// called for the driver on the open list. If there is an item in the open list of the protocol interface with an
    /// attribute of `BY_DRIVER` remaining after the `DisconnectController()` call has been made, `EFI_ACCESS_DENIED` is returned.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | An item was added to the open list for the protocol interface, and the protocol interface was returned in Interface. |
    /// | `EFI_INVALID_PARAMETER` | `Protocol` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Interface` is `NULL`, and `Attributes` is not `TEST_PROTOCOL`. |
    /// | `EFI_INVALID_PARAMETER` | `Handle` is `NULL`. |
    /// | `EFI_UNSUPPORTED` | `Handle` does not support `Protocol`. |
    /// | `EFI_INVALID_PARAMETER` | `Attributes` is not a legal value. |
    /// | `EFI_INVALID_PARAMETER` | `Attributes` is `BY_CHILD_CONTROLLER` and `AgentHandle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Attributes` is `BY_DRIVER` and `AgentHandle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Attributes` is `BY_DRIVER|EXCLUSIVE` and `AgentHandle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Attributes` is `EXCLUSIVE` and `AgentHandle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Attributes` is `BY_CHILD_CONTROLLER` and `ControllerHandle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Attributes` is `BY_DRIVER` and `ControllerHandle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Attributes` is `BY_DRIVER|EXCLUSIVE` and `ControllerHandle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Attributes` is `BY_CHILD_CONTROLLER` and `Handle` is identical to `ControllerHandle`. |
    /// | `EFI_ACCESS_DENIED` | `Attributes` is `BY_DRIVER` and there is an item on the open list with an attribute of `BY_DRIVER|EXCLUSIVE` or `EXCLUSIVE`. |
    /// | `EFI_ACCESS_DENIED` | `Attributes` is `BY_DRIVER|EXCLUSIVE` and there is an item on the open list with an attribute of `EXCLUSIVE`. |
    /// | `EFI_ACCESS_DENIED` | `Attributes` is `EXCLUSIVE` and there is an item on the open list with an attribute of `BY_DRIVER|EXCLUSIVE` or `EXCLUSIVE`. |
    /// | `EFI_ALREADY_STARTED` | `Attributes` is `BY_DRIVER` and there is an item on the open list with an attribute of `BY_DRIVER` whose agent handle is the same as `AgentHandle`. |
    /// | `EFI_ACCESS_DENIED` | `Attributes` is `BY_DRIVER` and there is an item on the open list with an attribute of `BY_DRIVER` whose agent handle is different than `AgentHandle`. |
    /// | `EFI_ALREADY_STARTED` | `Attributes` is `BY_DRIVER|EXCLUSIVE` and there is an item on the open list with an attribute of `BY_DRIVER|EXCLUSIVE` whose agent handle is the same as `AgentHandle`. |
    /// | `EFI_ACCESS_DENIED` | `Attributes` is `BY_DRIVER|EXCLUSIVE` and there is an item on the open list with an attribute of `BY_DRIVER|EXCLUSIVE` whose agent handle is different than `AgentHandle`. |
    /// | `EFI_ACCESS_DENIED` | `Attributes` is `BY_DRIVER|EXCLUSIVE` or `EXCLUSIVE` and there are items in the open list with an attribute of `BY_DRIVER` that could not be removed when `EFI_BOOT_SERVICES.DisconnectController()` was called for that open item. |
    pub OpenProtocol: unsafe extern "efiapi" fn(
        Handle: EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        Interface: *mut *mut VOID,
        AgentHandle: EFI_HANDLE,
        ControllerHandle: EFI_HANDLE,
        Attributes: UINT32,
    ) -> EFI_STATUS,
    /// Closes a protocol on a handle that was opened using `EFI_BOOT_SERVICES.OpenProtocol()`.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Handle` | The handle for the protocol interface that was previously opened with `OpenProtocol()`, and is now being closed. |
    /// | **IN** `Protocol` | The published unique identifier of the protocol. It is the caller’s responsibility to pass in a valid GUID. |
    /// | **IN** `AgentHandle` | The handle of the agent that is closing the protocol interface. For agents that follow the UEFI Driver Model, this parameter is the handle that contains the `EFI_DRIVER_BINDING_PROTOCOL` instance that is produced by the UEFI driver that is opening the protocol interface. For UEFI applications, this is the image handle of the UEFI application. For applications that used `EFI_BOOT_SERVICES.HandleProtocol()` to open the protocol interface, this will be the image handle of the EFI firmware. |
    /// | **IN** `ControllerHandle` | If the agent that opened a protocol is a driver that follows the UEFI Driver Model, then this parameter is the controller handle that required the protocol interface. If the agent does not follow the UEFI Driver Model, then this parameter is optional and may be `NULL`. |
    ///
    /// ## Description
    ///
    /// This function updates the handle database to show that the protocol instance specified by `Handle` and `Protocol`
    /// is no longer required by the agent and controller specified `AgentHandle` and `ControllerHandle`.
    ///
    /// If `Handle` or `AgentHandle` is `NULL`, then `EFI_INVALID_PARAMETER` is returned. If `ControllerHandle` is `NULL`,
    /// then `EFI_INVALID_PARAMETER` is returned. If `Protocol` is `NULL`, then `EFI_INVALID_PARAMETER` is returned.
    ///
    /// If the interface specified by `Protocol` is not supported by the handle specified by `Handle`, then `EFI_NOT_FOUND`
    /// is returned.
    ///
    /// If the interface specified by `Protocol` is supported by the handle specified by `Handle`, then a check is made
    /// to see if the protocol instance specified by `Protocol` and `Handle` was opened by `AgentHandle` and `ControllerHandle`
    /// with `EFI_BOOT_SERVICES.OpenProtocol()`. If the protocol instance was not opened by `AgentHandle` and `ControllerHandle`,
    /// then `EFI_NOT_FOUND` is returned. If the protocol instance was opened by `AgentHandle` and `ControllerHandle`,
    /// then all of those references are removed from the handle database, and `EFI_SUCCESS` is returned.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The protocol instance was closed. |
    /// | `EFI_INVALID_PARAMETER` | `Handle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `AgentHandle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `ControllerHandle` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Protocol` is `NULL`. |
    /// | `EFI_NOT_FOUND` | `Handle` does not support the protocol specified by `Protocol`. |
    /// | `EFI_NOT_FOUND` | The protocol interface specified by `Handle` and `Protocol` is not currently open by `AgentHandle` and `ControllerHandle`. |
    pub CloseProtocol: unsafe extern "efiapi" fn(
        Handle: EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        AgentHandle: EFI_HANDLE,
        ControllerHandle: EFI_HANDLE,
    ) -> EFI_STATUS,
    /// Creates an event in a group.
    ///
    /// ## Parameters
    ///
    /// | Parameter       | Description                                                                                                              |
    /// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
    /// | **IN** `Type` | The type of event to create and its mode and attributes. |
    /// | **IN** `NotifyTPL` | The task priority level of event notifications, if needed. |
    /// | **IN** `NotifyFunction` **OPTIONAL** | Pointer to the event’s notification function, if any. |
    /// | **IN** `NotifyContext` **OPTIONAL** | Pointer to the notification function’s context; corresponds to parameter `Context` in the notification function. |
    /// | **IN** `EventGroup` **OPTIONAL** | Pointer to the unique identifier of the group to which this event belongs. If this is `NULL`, then the function behaves as if the parameters were passed to `CreateEvent`. |
    /// | **OUT** `Event` | Pointer to the newly created event if the call succeeds; undefined otherwise. |
    ///
    /// ## Description
    ///
    /// The `CreateEventEx()` function creates a new event of type `Type` and returns it in the specified location indicated
    /// by `Event`. The event’s notification function, context and task priority are specified by `NotifyFunction`,
    /// `NotifyContext`, and `NotifyTPL`, respectively. The event will be added to the group of events identified by
    /// `EventGroup`.
    ///
    /// If no group is specified by `EventGroup`, then this function behaves as if the same parameters had been passed
    /// to `CreateEvent`.
    ///
    /// Event groups are collections of events identified by a shared `EFI_GUID` where, when one member event is signaled,
    /// all other events are signaled and their individual notification actions are taken (as described in `CreateEvent`).
    /// All events are guaranteed to be signaled before the first notification action is taken. All notification functions
    /// will be executed in the order specified by their `NotifyTPL`.
    ///
    /// A single event can only be part of a single event group. An event may be removed from an event group by using
    /// `CloseEvent()`.
    ///
    /// The `Type` of an event uses the same values as defined in `CreateEvent()` except that `EVT_SIGNAL_EXIT_BOOT_SERVICES`
    /// and `EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE` are not valid.
    ///
    /// If `Type` has `EVT_NOTIFY_SIGNAL` or `EVT_NOTIFY_WAIT`, then `NotifyFunction` must be non-`NULL` and `NotifyTPL`
    /// must be a valid task priority level. Otherwise these parameters are ignored.
    ///
    /// More than one event of type `EVT_TIMER` may be part of a single event group. However, there is no mechanism for
    /// determining which of the timers was signaled.
    ///
    /// #### Configuration Table Groups
    ///
    /// The GUID for a configuration table also defines a corresponding event group GUID with the same value. If the data
    /// represented by a configuration table is changed, `InstallConfigurationTable()` should be called. When `InstallConfigurationTable()`
    /// is called, the corresponding event is signaled. When this event is signaled, any components that cache information
    /// from the configuration table can optionally update their cached state.
    ///
    /// For example, `EFI_ACPI_TABLE_GUID` defines a configuration table for ACPI data. When ACPI data is changed,
    /// `InstallConfigurationTable()` is called. During the execution of `InstallConfigurationTable()`, a corresponding
    /// event group with `EFI_ACPI_TABLE_GUID` is signaled, allowing an application to invalidate any cached ACPI data.
    ///
    /// #### Pre-Defined Event Groups
    ///
    /// This section describes the pre-defined event groups used by the UEFI specification.
    ///
    /// ### `EFI_EVENT_GROUP_EXIT_BOOT_SERVICES`
    ///
    /// This event group is notified by the system when `ExitBootServices()` is invoked after notifying
    /// `EFI_EVENT_GROUP_BEFORE_EXIT_BOOT_SERVICES` event group. This event group is functionally equivalent to the
    /// `EVT_SIGNAL_EXIT_BOOT_SERVICES` flag for the `Type` argument of `CreateEvent()`. The notification function for
    /// this event must comply with the following requirements:
    ///
    /// - The notification function is not allowed to use the Memory Allocation Services, or call any functions that use
    /// the Memory Allocation Services, because these services modify the current memory map.
    ///
    /// **Note:** Since consumer of the service does not necessarily knows if the service uses memory allocation services,
    /// this requirement is effectively a mandate to reduce usage of any external services (services implemented outside
    /// of the driver owning the notification function) to an absolute minimum required to perform an orderly transition
    /// to a runtime environment. Usage of the external services may yield unexpected results. Since UEFI specification
    /// does not guarantee any given order of notification function invocation, a notification function consuming the
    /// service may be invoked before or after the notification function of the driver providing the service. As a result,
    /// a service being called by the notification function may exhibit boot time behavior or a runtime behavior (which is
    /// undefined for pure boot services).
    ///
    /// - The notification function must not depend on timer events since timer services will be deactivated before any
    /// notification functions are called.
    ///
    /// Refer to `EFI_BOOT_SERVICES.ExitBootServices()` for additional details.
    ///
    /// ### `EFI_EVENT_GROUP_BEFORE_EXIT_BOOT_SERVICES`
    ///
    /// This event group is notified by the system `ExitBootServices()` is invoked right before notifying
    /// `EFI_EVENT_GROUP_EXIT_BOOT_SERVICES` event group. The event presents the last opportunity to use firmware
    /// interfaces in the boot environment.
    ///
    /// The notification function for this event must not depend on any kind of delayed processing (processing that happens
    /// in a timer callback beyond the time span of the notification function) because system firmware deactivates timer
    /// services right after dispatching handlers for this event group.
    ///
    /// Refer to `EFI_BOOT_SERVICES.ExitBootServices()` for additional details.
    ///
    /// ### `EFI_EVENT_GROUP_VIRTUAL_ADDRESS_CHANGE`
    ///
    /// This event group is notified by the system when `SetVirtualAddressMap()` is invoked. This is functionally equivalent
    /// to the `VT_SIGNAL_VIRTUAL_ADDRESS_CHANGE` flag for the `Type` argument of `CreateEvent`.
    ///
    /// ### `EFI_EVENT_GROUP_MEMORY_MAP_CHANGE`
    ///
    /// This event group is notified by the system when the memory map has changed. The notification function for this
    /// event should not use Memory Allocation Services to avoid reentrancy complications.
    ///
    /// ### `EFI_EVENT_GROUP_READY_TO_BOOT`
    ///
    /// This event group is notified by the system right before notifying `EFI_EVENT_GROUP_AFTER_READY_TO_BOOT` event
    /// group when the Boot Manager is about to load and execute a boot option or a platform or OS recovery option. The
    /// event group presents the last chance to modify device or system configuration prior to passing control to a boot
    /// option.
    ///
    /// ### `EFI_EVENT_GROUP_AFTER_READY_TO_BOOT`
    ///
    /// This event group is notified by the system immediately after notifying `EFI_EVENT_GROUP_READY_TO_BOOT` event group
    /// when the Boot Manager is about to load and execute a boot option or a platform or OS recovery option. The event
    /// group presents the last chance to survey device or system configuration prior to passing control to a boot
    /// option.
    ///
    /// ### `EFI_EVENT_GROUP_RESET_SYSTEM`
    ///
    /// This event group is notified by the system when `ResetSystem()` is invoked and the system is about to be reset.
    /// The event group is only notified prior to `ExitBootServices()` invocation.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code             | Description                                                                                                                                                                                                 |
    /// | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The event structure was created. |
    /// | `EFI_INVALID_PARAMETER` | One of the parameters has an invalid value. |
    /// | `EFI_INVALID_PARAMETER` | `Event` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Type` has an unsupported bit set. |
    /// | `EFI_INVALID_PARAMETER` | `Type` has both `EVT_NOTIFY_SIGNAL` and `EVT_NOTIFY_WAIT` set. |
    /// | `EFI_INVALID_PARAMETER` | `Type` has either `EVT_NOTIFY_SIGNAL` or `EVT_NOTIFY_WAIT` set and `NotifyFunction` is `NULL`. |
    /// | `EFI_INVALID_PARAMETER` | `Type` has either `EVT_NOTIFY_SIGNAL` or `EVT_NOTIFY_WAIT` set and `NotifyTPL` is not a supported TPL level. |
    /// | `EFI_OUT_OF_RESOURCES` | The event could not be allocated. |
    pub CreateEventEx: unsafe extern "efiapi" fn(
        Type: UINT32,
        NotifyTPL: EFI_TPL,
        NotifyFunction: EFI_EVENT_NOTIFY,
        NotifyContext: *const VOID,
        EventGroup: *const EFI_GUID,
        Event: *mut EFI_EVENT,
    ) -> EFI_STATUS,
}

/// A descriptor for a memory map.
#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
    /// Type of the memory region.
    pub Type: UINT32,
    /// Physical address of the first byte in the memory region. `PhysicalStart` must be aligned on a
    /// 4 KiB boundary, and must not be above `0xFFFFFFFFFFFFF000`.
    pub PhysicalStart: EFI_PHYSICAL_ADDRESS,
    /// Virtual address of the first byte in the memory region. `VirtualStart` must be aligned on a
    /// 4 KiB boundary, and must not be above `0xFFFFFFFFFFFFF000`.
    pub VirtualStart: EFI_VIRTUAL_ADDRESS,
    /// Number of 4 KiB pages in the memory region. `NumberOfPages` must not be `0`, and must not be
    /// any value that would represent a memory page with a start address, either physical or virtual,
    /// above `0xFFFFFFFFFFFFF000`.
    pub NumberOfPages: UINT64,
    /// Attributes of the memory region that describe the bit mask of capabilities for that memory region,
    /// and not necessarily the current settings for that memory region.
    pub Attribute: UINT64,
}

/// A physical address.
pub type EFI_PHYSICAL_ADDRESS = UINT64;
/// A virtual address.
pub type EFI_VIRTUAL_ADDRESS = UINT64;

/// ## Parameters
///
/// | Parameter       | Description                                                                                                              |
/// | --------------- | ------------------------------------------------------------------------------------------------------------------------ |
/// | **IN** `Event` | Event whose notification function is being invoked. |
/// | **IN** `Context` | Pointer to the notification function’s context, which is implementation-dependent. `Context` corresponds to `NotifyContext` in `EFI_BOOT_SERVICES.CreateEvent()`. |
pub type EFI_EVENT_NOTIFY =
    unsafe extern "efiapi" fn(Event: EFI_EVENT, Context: *mut VOID) -> EFI_STATUS;
