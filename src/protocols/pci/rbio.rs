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

use crate::tables::boot::{EFI_ALLOCATE_TYPE, EFI_MEMORY_TYPE, EFI_PHYSICAL_ADDRESS};

pub const EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x2F707EBB,
        0x4A1A,
        0x11D4,
        [0x9A, 0x38, 0x00, 0x90, 0x27, 0x3F, 0xC1, 0x4D],
    )
};

pub const EFI_PCI_ATTRIBUTE_ISA_MOTHERBOARD_IO: UINT64 = 0x0001;
pub const EFI_PCI_ATTRIBUTE_ISA_IO: UINT64 = 0x0002;
pub const EFI_PCI_ATTRIBUTE_VGA_PALETTE_IO: UINT64 = 0x0004;
pub const EFI_PCI_ATTRIBUTE_VGA_MEMORY: UINT64 = 0x0008;
pub const EFI_PCI_ATTRIBUTE_VGA_IO: UINT64 = 0x0010;
pub const EFI_PCI_ATTRIBUTE_IDE_PRIMARY_IO: UINT64 = 0x0020;
pub const EFI_PCI_ATTRIBUTE_IDE_SECONDARY_IO: UINT64 = 0x0040;
pub const EFI_PCI_ATTRIBUTE_MEMORY_WRITE_COMBINE: UINT64 = 0x0080;
pub const EFI_PCI_ATTRIBUTE_MEMORY_CACHED: UINT64 = 0x0800;
pub const EFI_PCI_ATTRIBUTE_MEMORY_DISABLE: UINT64 = 0x1000;
pub const EFI_PCI_ATTRIBUTE_DUAL_ADDRESS_CYCLE: UINT64 = 0x8000;
pub const EFI_PCI_ATTRIBUTE_ISA_IO_16: UINT64 = 0x10000;
pub const EFI_PCI_ATTRIBUTE_VGA_PALETTE_IO_16: UINT64 = 0x20000;
pub const EFI_PCI_ATTRIBUTE_VGA_IO_16: UINT64 = 0x40000;

#[repr(C)]
pub enum EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_OPERATION {
    EfiPciOperationBusMasterRead,
    EfiPciOperationBusMasterWrite,
    EfiPciOperationBusMasterCommonBuffer,
    EfiPciOperationBusMasterRead64,
    EfiPciOperationBusMasterWrite64,
    EfiPciOperationBusMasterCommonBuffer64,
    EfiPciOperationMaximum,
}

#[repr(C)]
pub enum EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_WIDTH {
    EfiPciWidthUint8,
    EfiPciWidthUint16,
    EfiPciWidthUint32,
    EfiPciWidthUint64,
    EfiPciWidthFifoUint8,
    EfiPciWidthFifoUint16,
    EfiPciWidthFifoUint32,
    EfiPciWidthFifoUint64,
    EfiPciWidthFillUint8,
    EfiPciWidthFillUint16,
    EfiPciWidthFillUint32,
    EfiPciWidthFillUint64,
    EfiPciWidthMaximum,
}

#[repr(C)]
pub struct EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL {
    pub ParentHandle: EFI_HANDLE,
    pub PollMem: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL,
        Width: EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_WIDTH,
        Address: UINT64,
        Mask: UINT64,
        Value: UINT64,
        Delay: UINT64,
        Result: *mut UINT64,
    ) -> EFI_STATUS,
    pub PollIo: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL,
        Width: EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_WIDTH,
        Address: UINT64,
        Mask: UINT64,
        Value: UINT64,
        Delay: UINT64,
        Result: *mut UINT64,
    ) -> EFI_STATUS,
    pub Mem: EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_ACCESS,
    pub Io: EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_ACCESS,
    pub Pci: EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_ACCESS,
    pub CopyMem: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL,
        Width: EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_WIDTH,
        DestAddress: UINT64,
        SrcAddress: UINT64,
        Count: UINTN,
    ) -> EFI_STATUS,
    pub Map: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL,
        Operation: EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_OPERATION,
        HostAddress: *mut VOID,
        NumberOfBytes: *mut UINTN,
        DeviceAddress: *mut EFI_PHYSICAL_ADDRESS,
        Mapping: *mut *mut VOID,
    ) -> EFI_STATUS,
    pub Unmap: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL,
        Mapping: *mut VOID,
    ) -> EFI_STATUS,
    pub AllocateBuffer: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL,
        Type: EFI_ALLOCATE_TYPE,
        MemoryType: EFI_MEMORY_TYPE,
        Pages: UINTN,
        HostAddress: *mut *mut VOID,
        Attributes: UINT64,
    ) -> EFI_STATUS,
    pub FreeBuffer: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL,
        Pages: UINTN,
        HostAddress: *mut *mut VOID,
    ) -> EFI_STATUS,
    pub Flush: unsafe extern "efiapi" fn(This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL) -> EFI_STATUS,
    pub GetAttributes: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL,
        Supports: *mut UINT64,
        Attributes: *mut UINT64,
    ) -> EFI_STATUS,
    pub SetAttributes: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL,
        Attributes: UINT64,
        ResourceBase: *mut UINT64,
        ResourceLength: *mut UINT64,
    ) -> EFI_STATUS,
    pub Configuration: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL,
        Resources: *mut *mut VOID,
    ) -> EFI_STATUS,
    pub Segment: UINT32,
}

#[repr(C)]
pub struct EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_ACCESS {
    pub Read: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL,
        Width: EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_WIDTH,
        Address: UINT64,
        Count: UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
    pub Write: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL,
        Width: EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_WIDTH,
        Address: UINT64,
        Count: UINTN,
        Buffer: *mut VOID,
    ) -> EFI_STATUS,
}
