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

use crate::protocols::pci::rbio::EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_WIDTH;
use crate::tables::boot::{EFI_ALLOCATE_TYPE, EFI_MEMORY_TYPE, EFI_PHYSICAL_ADDRESS};

pub const EFI_PCI_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x4CF5B200,
        0x68B8,
        0x4CA5,
        [0x9E, 0xEc, 0xB2, 0x3E, 0x3F, 0x50, 0x02, 0x9A],
    )
};

pub const EFI_PCI_IO_ATTRIBUTE_ISA_MOTHERBOARD_IO: UINT64 = 0x0001;
pub const EFI_PCI_IO_ATTRIBUTE_ISA_IO: UINT64 = 0x0002;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_PALETTE_IO: UINT64 = 0x0004;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_MEMORY: UINT64 = 0x0008;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_IO: UINT64 = 0x0010;
pub const EFI_PCI_IO_ATTRIBUTE_IDE_PRIMARY_IO: UINT64 = 0x0020;
pub const EFI_PCI_IO_ATTRIBUTE_IDE_SECONDARY_IO: UINT64 = 0x0040;
pub const EFI_PCI_IO_ATTRIBUTE_MEMORY_WRITE_COMBINE: UINT64 = 0x0080;
pub const EFI_PCI_IO_ATTRIBUTE_IO: UINT64 = 0x0100;
pub const EFI_PCI_IO_ATTRIBUTE_MEMORY: UINT64 = 0x0200;
pub const EFI_PCI_IO_ATTRIBUTE_BUS_MASTER: UINT64 = 0x0400;
pub const EFI_PCI_IO_ATTRIBUTE_MEMORY_CACHED: UINT64 = 0x0800;
pub const EFI_PCI_IO_ATTRIBUTE_MEMORY_DISABLE: UINT64 = 0x1000;
pub const EFI_PCI_IO_ATTRIBUTE_EMBEDDED_DEVICE: UINT64 = 0x2000;
pub const EFI_PCI_IO_ATTRIBUTE_EMBEDDED_ROM: UINT64 = 0x4000;
pub const EFI_PCI_IO_ATTRIBUTE_DUAL_ADDRESS_CYCLE: UINT64 = 0x8000;
pub const EFI_PCI_IO_ATTRIBUTE_ISA_IO_16: UINT64 = 0x10000;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_PALETTE_IO_16: UINT64 = 0x20000;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_IO_16: UINT64 = 0x40000;

#[repr(C)]
pub enum EFI_PCI_IO_PROTOCOL_ATTRIBUTE_OPERATION {
    EfiPciIoAttributeOperationGet,
    EfiPciIoAttributeOperationSet,
    EfiPciIoAttributeOperationEnable,
    EfiPciIoAttributeOperationDisable,
    EfiPciIoAttributeOperationSupported,
    EfiPciIoAttributeOperationMaximum,
}

#[repr(C)]
pub enum EFI_PCI_IO_PROTOCOL_OPERATION {
    EfiPciIoOperationBusMasterRead,
    EfiPciIoOperationBusMasterWrite,
    EfiPciIoOperationBusMasterCommonBuffer,
    EfiPciIoOperationMaximum,
}

#[repr(C)]
pub enum EFI_PCI_IO_PROTOCOL_WIDTH {
    EfiPciIoWidthUint8,
    EfiPciIoWidthUint16,
    EfiPciIoWidthUint32,
    EfiPciIoWidthUint64,
    EfiPciIoWidthFifoUint8,
    EfiPciIoWidthFifoUint16,
    EfiPciIoWidthFifoUint32,
    EfiPciIoWidthFifoUint64,
    EfiPciIoWidthFillUint8,
    EfiPciIoWidthFillUint16,
    EfiPciIoWidthFillUint32,
    EfiPciIoWidthFillUint64,
    EfiPciIoWidthMaximum,
}

#[repr(C)]
pub struct EFI_PCI_IO_PROTOCOL {
    pub PollMem: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        Width: EFI_PCI_IO_PROTOCOL_WIDTH,
        BarIndex: UINT8,
        Offset: UINT64,
        Mask: UINT64,
        Value: UINT64,
        Delay: UINT64,
        Result: *mut UINT64,
    ) -> EFI_STATUS,
    pub PollIo: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        Width: EFI_PCI_IO_PROTOCOL_WIDTH,
        BarIndex: UINT8,
        Offset: UINT64,
        Mask: UINT64,
        Value: UINT64,
        Delay: UINT64,
        Result: *mut UINT64,
    ) -> EFI_STATUS,
    pub Mem: EFI_PCI_IO_PROTOCOL_ACCESS,
    pub Io: EFI_PCI_IO_PROTOCOL_ACCESS,
    pub Pci: EFI_PCI_IO_PROTOCOL_CONFIG_ACCESS,
    pub CopyMem: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        Width: EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL_WIDTH,
        DestBarIndex: UINT8,
        DestOffset: UINT64,
        SrcBarIndex: UINT8,
        SrcOffset: UINT64,
        Count: UINTN,
    ) -> EFI_STATUS,
    pub Map: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        Operation: EFI_PCI_IO_PROTOCOL_OPERATION,
        HostAddress: *mut VOID,
        NumberOfBytes: *mut UINTN,
        DeviceAddress: *mut EFI_PHYSICAL_ADDRESS,
        Mapping: *mut *mut VOID,
    ) -> EFI_STATUS,
    pub Unmap:
        unsafe extern "efiapi" fn(This: *mut EFI_PCI_IO_PROTOCOL, Mapping: *mut VOID) -> EFI_STATUS,
    pub AllocateBuffer: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        Type: EFI_ALLOCATE_TYPE,
        MemoryType: EFI_MEMORY_TYPE,
        Pages: UINTN,
        HostAddress: *mut *mut VOID,
        Attributes: UINT64,
    ) -> EFI_STATUS,
    pub FreeBuffer: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        Pages: UINTN,
        HostAddress: *mut VOID,
    ) -> EFI_STATUS,
    pub Flush: unsafe extern "efiapi" fn(This: *mut EFI_PCI_IO_PROTOCOL) -> EFI_STATUS,
    pub GetLocation: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        SegmentNumber: *mut UINTN,
        BusNumber: *mut UINTN,
        DeviceNumber: *mut UINTN,
        FunctionNumber: *mut UINTN,
    ) -> EFI_STATUS,
    pub Attributes: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        Operation: EFI_PCI_IO_PROTOCOL_ATTRIBUTE_OPERATION,
        Attributes: UINT64,
        Result: *mut UINT64,
    ) -> EFI_STATUS,
    pub GetBarAttributes: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        BarIndex: UINT8,
        Supports: *mut UINT64,
        Resources: *mut *mut VOID,
    ) -> EFI_STATUS,
    pub SetBarAttributes: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        BarIndex: UINT8,
        Offset: *mut UINT64,
        Length: *mut UINT64,
    ) -> EFI_STATUS,
    pub RomSize: UINT64,
    pub RomImage: *mut VOID,
}

#[repr(C)]
pub struct EFI_PCI_IO_PROTOCOL_ACCESS {
    pub Read: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        Width: EFI_PCI_IO_PROTOCOL_WIDTH,
        BarIndex: UINT8,
        Offset: UINT64,
        Count: UINT64,
        Buffer: *mut UINT64,
    ) -> EFI_STATUS,
    pub Write: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        Width: EFI_PCI_IO_PROTOCOL_WIDTH,
        BarIndex: UINT8,
        Offset: UINT64,
        Count: UINT64,
        Buffer: *mut UINT64,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_PCI_IO_PROTOCOL_CONFIG_ACCESS {
    pub Read: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        Width: EFI_PCI_IO_PROTOCOL_WIDTH,
        Offset: UINT64,
        Count: UINT64,
        Buffer: *mut UINT64,
    ) -> EFI_STATUS,
    pub Write: unsafe extern "efiapi" fn(
        This: *mut EFI_PCI_IO_PROTOCOL,
        Width: EFI_PCI_IO_PROTOCOL_WIDTH,
        Offset: UINT64,
        Count: UINT64,
        Buffer: *mut UINT64,
    ) -> EFI_STATUS,
}
