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

pub const EFI_PCI_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x4CF5B200,
        0x68B8,
        0x4CA5,
        [0x9E, 0xEc, 0xB2, 0x3E, 0x3F, 0x50, 0x02, 0x9A],
    )
};

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
}
