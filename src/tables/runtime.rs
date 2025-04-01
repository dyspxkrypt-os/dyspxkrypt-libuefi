/*
 * Dyspxkrypt LibUEFI: Raw bindings of UEFI that conforms to the definitions of the UEFI Specification.
 * Copyright (C) 2023-2025 HTGAzureX1212.
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

#[repr(C)]
pub struct EFI_RUNTIME_SERVICES {
    pub Hdr: EFI_TABLE_HEADER,
    pub GetTime: unsafe extern "efiapi" fn(
        Time: *mut EFI_TIME,
        Capabilities: *mut EFI_TIME_CAPABILITIES,
    ) -> EFI_STATUS,
    pub SetTime: unsafe extern "efiapi" fn(Time: *mut EFI_TIME) -> EFI_STATUS,
    pub GetWakeupTime: unsafe extern "efiapi" fn(
        Enabled: *mut BOOLEAN,
        Pending: *mut BOOLEAN,
        Time: *mut EFI_TIME,
    ) -> EFI_STATUS,
    pub SetWakeupTime:
        unsafe extern "efiapi" fn(Enable: BOOLEAN, Time: *mut EFI_TIME) -> EFI_STATUS,
    pub SetVirtualAddressMap: unsafe extern "efiapi" fn(
        MemoryMapSize: UINTN,
        DescriptorSize: UINTN,
        DescriptorVersion: UINT32,
        VirtualMap: *mut EFI_MEMORY_DESCRIPTOR,
    ) -> EFI_STATUS,
    pub ConvertPointer:
        unsafe extern "efiapi" fn(DebugDisposition: UINTN, Address: *mut *mut VOID) -> EFI_STATUS,
    pub GetVariable: unsafe extern "efiapi" fn(
        VariableName: *mut CHAR16,
        VendorGuid: *mut EFI_GUID,
        Attributes: *mut UINT32,
        DataSize: *mut UINTN,
        Data: *mut VOID,
    ) -> EFI_STATUS,
    pub GetNextVariableName: unsafe extern "efiapi" fn(
        VariableNameSize: *mut UINTN,
        VariableName: *mut CHAR16,
        VendorGuid: *mut EFI_GUID,
    ) -> EFI_STATUS,
    pub SetVariable: unsafe extern "efiapi" fn(
        VariableName: *mut CHAR16,
        VendorGuid: *mut EFI_GUID,
        Attributes: UINT32,
        DataSize: UINTN,
        Data: *mut VOID,
    ) -> EFI_STATUS,
    pub GetNextHighMonotonicCount: unsafe extern "efiapi" fn(HighCount: *mut UINT32) -> EFI_STATUS,
    pub ResetSystem: unsafe extern "efiapi" fn(
        ResetType: EFI_RESET_TYPE,
        ResetStatus: EFI_STATUS,
        DataSize: UINTN,
        ResetData: *mut VOID,
    ),
    pub UpdateCapsule: unsafe extern "efiapi" fn(
        CapsuleHeaderArray: *mut *mut EFI_CAPSULE_HEADER,
        CapsuleCount: UINTN,
        ScatterGatherList: EFI_PHYSICAL_ADDRESS,
    ) -> EFI_STATUS,
    pub QueryCapsuleCapabilities: unsafe extern "efiapi" fn(
        CapsuleHeaderArray: *mut *mut EFI_CAPSULE_HEADER,
        CapsuleCount: UINTN,
        MaximumCapsuleSize: *mut UINT64,
        ResetType: *mut EFI_RESET_TYPE,
    ) -> EFI_STATUS,
    pub QueryVariableInfo: unsafe extern "efiapi" fn(
        Attributes: UINT32,
        MaximumVariableStorageSize: *mut UINT64,
        RemainingVariableStorageSize: *mut UINT64,
        MaximumVariableSize: *mut UINT64,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_TIME {
    pub Year: UINT16,
    pub Month: UINT8,
    pub Day: UINT8,
    pub Hour: UINT8,
    pub Minute: UINT8,
    pub Second: UINT8,
    #[doc(hidden)]
    Pad1: UINT8,
    pub Nanosecond: UINT32,
    pub TimeZone: INT16,
    pub Daylight: UINT8,
    #[doc(hidden)]
    Pad2: UINT8,
}

#[repr(C)]
pub struct EFI_TIME_CAPABILITIES {
    pub Resolution: UINT32,
    pub Accuracy: UINT32,
    pub SetsToZero: BOOLEAN,
}

#[repr(C)]
pub struct EFI_VARIABLE_AUTHENTICATION_3_CERT_ID {
    pub Type: UINT8,
    pub IdSize: UINT32,
}

#[repr(C)]
pub struct EFI_CAPSULE_BLOCK_DESCRIPTOR {
    pub Length: UINT64,
    pub Union: EFI_CAPSULE_BLOCK_DESCRIPTOR_INNER,
}

#[repr(C)]
pub struct EFI_CAPSULE_HEADER {
    pub CapsuleGuid: EFI_GUID,
    pub HeaderSize: UINT32,
    pub Flags: UINT32,
    pub CapsuleImageSize: UINT32,
}

#[repr(C)]
pub union EFI_CAPSULE_BLOCK_DESCRIPTOR_INNER {
    pub DataBlock: EFI_PHYSICAL_ADDRESS,
    pub ContinuationPointer: EFI_PHYSICAL_ADDRESS,
}
