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

use crate::tables::system::EFI_SPECIFICATION_VERSION;
use crate::tables::EFI_TABLE_HEADER;
use crate::types::{EFI_EVENT, EFI_GUID, EFI_HANDLE, EFI_STATUS, EFI_TPL, UINT32, UINT64, UINTN, VOID};

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
        Handle: *mut EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        OldInterface: *mut VOID,
        NewInterface: *mut VOID,
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
