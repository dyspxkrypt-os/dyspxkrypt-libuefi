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
use crate::types::{EFI_STATUS, EFI_TPL, UINT32, UINT64, UINTN, VOID};

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
    pub RaiseTPL: unsafe extern "efiapi" fn(
        NewTPL: EFI_TPL,
    ) -> EFI_TPL,
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
    pub RestoreTPL: unsafe extern "efiapi" fn(
        OldTPL: EFI_TPL,
    ) -> VOID,
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
    pub FreePages: unsafe extern "efiapi" fn(
        Memory: EFI_PHYSICAL_ADDRESS,
        Pages: UINTN,
    ) -> EFI_STATUS,
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
