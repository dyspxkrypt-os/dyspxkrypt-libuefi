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
    BOOLEAN, CHAR16, EFI_EVENT, EFI_GUID, EFI_HANDLE, EFI_STATUS, EFI_TPL, UINT32, UINT64, UINT8,
    UINTN, VOID,
};

pub const EFI_BOOT_SERVICES_SIGNATURE: UINT64 = 0x56524553544F4F42;
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

#[repr(C)]
pub struct EFI_BOOT_SERVICES {
    pub Hdr: EFI_TABLE_HEADER,

    pub RaiseTPL: unsafe extern "efiapi" fn(NewTPL: EFI_TPL) -> EFI_TPL,

    pub RestoreTPL: unsafe extern "efiapi" fn(OldTPL: EFI_TPL) -> VOID,

    pub AllocatePages: unsafe extern "efiapi" fn(
        Type: EFI_ALLOCATE_TYPE,
        MemoryType: EFI_MEMORY_TYPE,
        Pages: UINTN,
        Memory: *mut EFI_PHYSICAL_ADDRESS,
    ) -> EFI_STATUS,

    pub FreePages:
        unsafe extern "efiapi" fn(Memory: EFI_PHYSICAL_ADDRESS, Pages: UINTN) -> EFI_STATUS,

    pub GetMemoryMap: unsafe extern "efiapi" fn(
        MemoryMapSize: *mut UINTN,
        MemoryMap: *mut EFI_MEMORY_DESCRIPTOR,
        MapKey: *mut UINTN,
        DescriptorSize: *mut UINTN,
        DescriptorVersion: *mut UINT32,
    ) -> EFI_STATUS,

    pub AllocatePool: unsafe extern "efiapi" fn(
        PoolType: EFI_MEMORY_TYPE,
        Size: UINTN,
        Buffer: *mut *mut VOID,
    ) -> EFI_STATUS,

    pub FreePool: unsafe extern "efiapi" fn(Buffer: *mut VOID) -> EFI_STATUS,

    pub CreateEvent: unsafe extern "efiapi" fn(
        Type: UINT32,
        NotifyTPL: EFI_TPL,
        NotifyFunction: EFI_EVENT_NOTIFY,
        NotifyContext: *mut VOID,
        Event: *mut EFI_EVENT,
    ) -> EFI_STATUS,

    pub SetTimer: unsafe extern "efiapi" fn(
        Event: EFI_EVENT,
        Type: EFI_TIMER_DELAY,
        TriggerTime: UINT64,
    ) -> EFI_STATUS,

    pub WaitForEvent: unsafe extern "efiapi" fn(
        NumberOfEvents: UINTN,
        Event: *mut EFI_EVENT,
        Index: *mut UINTN,
    ) -> EFI_STATUS,

    pub SignalEvent: unsafe extern "efiapi" fn(Event: EFI_EVENT) -> EFI_STATUS,

    pub CloseEvent: unsafe extern "efiapi" fn(Event: EFI_EVENT) -> EFI_STATUS,

    pub CheckEvent: unsafe extern "efiapi" fn(Event: EFI_EVENT) -> EFI_STATUS,

    pub InstallProtocolInterface: unsafe extern "efiapi" fn(
        Handle: *mut EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        InterfaceType: EFI_INTERFACE_TYPE,
        Interface: *mut VOID,
    ) -> EFI_STATUS,

    pub ReinstallProtocolInterface: unsafe extern "efiapi" fn(
        Handle: EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        OldInterface: *mut VOID,
        NewInterface: *mut VOID,
    ) -> EFI_STATUS,

    pub UninstallProtocolInterface: unsafe extern "efiapi" fn(
        Handle: EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        Interface: *mut VOID,
    ) -> EFI_STATUS,

    #[deprecated(since = "0.1.0", note = "use the OpenProtocol() function instead")]
    pub HandleProtocol: unsafe extern "efiapi" fn(
        Handle: EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        Interface: *mut *mut VOID,
    ) -> EFI_STATUS,
    #[doc(hidden)]
    pub Reserved: *mut VOID,

    pub RegisterProtocolNotify: unsafe extern "efiapi" fn(
        Protocol: *mut EFI_GUID,
        Event: EFI_EVENT,
        Registration: *mut *mut VOID,
    ) -> EFI_STATUS,

    pub LocateHandle: unsafe extern "efiapi" fn(
        SearchType: EFI_LOCATE_SEARCH_TYPE,
        Protocol: *mut EFI_GUID,
        SearchKey: *mut VOID,
        BufferSize: UINTN,
        Buffer: *mut EFI_HANDLE,
    ) -> EFI_STATUS,

    pub LocateDevicePath: unsafe extern "efiapi" fn(
        Protocol: *mut EFI_GUID,
        DevicePath: *mut *mut EFI_DEVICE_PATH_PROTOCOL,
        Device: *mut EFI_HANDLE,
    ) -> EFI_STATUS,

    pub InstallConfigurationTable:
        unsafe extern "efiapi" fn(Guid: *mut EFI_GUID, Table: *mut VOID) -> EFI_STATUS,

    pub LoadImage: unsafe extern "efiapi" fn(
        BootPolicy: BOOLEAN,
        ParentImageHandle: EFI_HANDLE,
        DevicePath: *mut EFI_DEVICE_PATH_PROTOCOL,
        SourceBuffer: *mut VOID,
        SourceSize: UINTN,
        ImageHandle: *mut EFI_HANDLE,
    ) -> EFI_STATUS,

    pub StartImage: unsafe extern "efiapi" fn(
        ImageHandle: EFI_HANDLE,
        ExitDataSize: *mut UINTN,
        ExitData: *mut *mut CHAR16,
    ) -> EFI_STATUS,

    pub Exit: unsafe extern "efiapi" fn(
        ImageHandle: EFI_HANDLE,
        ExitStatus: EFI_STATUS,
        ExitDataSize: UINTN,
        ExitData: *mut CHAR16,
    ) -> EFI_STATUS,

    pub UnloadImage: unsafe extern "efiapi" fn(ImageHandle: EFI_HANDLE) -> EFI_STATUS,

    pub ExitBootServices:
        unsafe extern "efiapi" fn(ImageHandle: EFI_HANDLE, MapKey: UINTN) -> EFI_STATUS,

    pub GetNextMonotonicCount: unsafe extern "efiapi" fn(Count: *mut UINT64) -> EFI_STATUS,

    pub Stall: unsafe extern "efiapi" fn(Microseconds: UINTN) -> EFI_STATUS,

    pub SetWatchdogTimer: unsafe extern "efiapi" fn(
        Timeout: UINTN,
        WatchdogCode: UINT64,
        DataSize: UINTN,
        WatchdogData: *mut CHAR16,
    ) -> EFI_STATUS,

    pub ConnectController: unsafe extern "efiapi" fn(
        ControllerHandle: EFI_HANDLE,
        DriverImageHandle: *mut EFI_HANDLE,
        RemainingDevicePath: *mut EFI_DEVICE_PATH_PROTOCOL,
        Recursive: BOOLEAN,
    ) -> EFI_STATUS,

    pub DisconnectController: unsafe extern "efiapi" fn(
        ControllerHandle: EFI_HANDLE,
        DriverImageHandle: EFI_HANDLE,
        ChildHandle: EFI_HANDLE,
    ) -> EFI_STATUS,

    pub OpenProtocol: unsafe extern "efiapi" fn(
        Handle: EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        Interface: *mut *mut VOID,
        AgentHandle: EFI_HANDLE,
        ControllerHandle: EFI_HANDLE,
        Attributes: UINT32,
    ) -> EFI_STATUS,

    pub CloseProtocol: unsafe extern "efiapi" fn(
        Handle: EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        AgentHandle: EFI_HANDLE,
        ControllerHandle: EFI_HANDLE,
    ) -> EFI_STATUS,

    pub OpenProtocolInformation: unsafe extern "efiapi" fn(
        Handle: EFI_HANDLE,
        Protocol: *mut EFI_GUID,
        EntryBuffer: *mut *mut EFI_OPEN_PROTOCOL_INFORMATION_ENTRY,
        EntryCount: *mut UINTN,
    ) -> EFI_STATUS,

    pub ProtocolsPerHandle: unsafe extern "efiapi" fn(
        Handle: EFI_HANDLE,
        ProtocolBuffer: *mut *mut *mut EFI_GUID,
        ProtocolBufferCount: *mut UINTN,
    ) -> EFI_STATUS,

    pub LocateHandleBuffer: unsafe extern "efiapi" fn(
        SearchType: EFI_LOCATE_SEARCH_TYPE,
        Protocol: *mut EFI_GUID,
        SearchKey: *mut VOID,
        NoHandles: *mut UINTN,
        Buffer: *mut *mut EFI_HANDLE,
    ) -> EFI_STATUS,

    //
    pub LocateProtocol: unsafe extern "efiapi" fn(
        Protocol: *mut EFI_GUID,
        Registration: *mut VOID,
        Interface: *mut *mut VOID,
    ) -> EFI_STATUS,

    pub InstallMultipleProtocolInterfaces:
        unsafe extern "efiapi" fn(Handle: *mut EFI_HANDLE, ...) -> EFI_STATUS,

    pub UninstallMultipleProtocolInterfaces:
        unsafe extern "efiapi" fn(Handle: *mut EFI_HANDLE, ...) -> EFI_STATUS,

    pub CalculateCrc32: unsafe extern "efiapi" fn(
        Data: *mut VOID,
        DataSize: UINTN,
        Crc32: *mut UINT32,
    ) -> EFI_STATUS,

    pub CopyMem:
        unsafe extern "efiapi" fn(Destination: *mut VOID, Source: *mut VOID, Length: UINTN) -> VOID,

    pub SetMem: unsafe extern "efiapi" fn(Buffer: *mut VOID, Size: UINTN, Value: UINT8) -> VOID,

    pub CreateEventEx: unsafe extern "efiapi" fn(
        Type: UINT32,
        NotifyTPL: EFI_TPL,
        NotifyFunction: EFI_EVENT_NOTIFY,
        NotifyContext: *const VOID,
        EventGroup: *const EFI_GUID,
        Event: *mut EFI_EVENT,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
    pub Type: UINT32,

    pub PhysicalStart: EFI_PHYSICAL_ADDRESS,

    pub VirtualStart: EFI_VIRTUAL_ADDRESS,

    pub NumberOfPages: UINT64,

    pub Attribute: UINT64,
}

#[repr(C)]
pub struct EFI_OPEN_PROTOCOL_INFORMATION_ENTRY {
    pub AgentHandle: EFI_HANDLE,
    pub ControllerHandle: EFI_HANDLE,
    pub Attributes: UINT32,
    pub OpenCount: UINT32,
}

pub type EFI_PHYSICAL_ADDRESS = UINT64;

pub type EFI_VIRTUAL_ADDRESS = UINT64;

pub type EFI_EVENT_NOTIFY =
    unsafe extern "efiapi" fn(Event: EFI_EVENT, Context: *mut VOID) -> EFI_STATUS;
