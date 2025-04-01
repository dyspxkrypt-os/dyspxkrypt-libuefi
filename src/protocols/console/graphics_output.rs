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

use crate::tables::boot::EFI_PHYSICAL_ADDRESS;

pub const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x9042A9DE,
        0x23DC,
        0x4A38,
        [0x96, 0xFB, 0x7A, 0xDE, 0xD0, 0x80, 0x51, 0x6A],
    )
};

#[repr(C)]
pub enum EFI_GRAPHICS_PIXEL_FORMAT {
    PixelRedGreenBlueReserved8BitPerColor,
    PixelBlueGreenRedReserved8BitPerColor,
    PixelBitMask,
    PixelBltOnly,
    PixelFormatMax,
}

#[repr(C)]
pub enum EFI_GRAPHICS_OUTPUT_BLT_OPERATION {
    EfiBltVideoFill,
    EfiBltVideoToBltBuffer,
    EfiBltBufferToVideo,
    EfiBltVideoToVideo,
    EfiGraphicsOutputBltOperationMax,
}

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_PROTOCOL {
    pub QueryMode: unsafe extern "efiapi" fn(
        This: *mut EFI_GRAPHICS_OUTPUT_PROTOCOL,
        ModeNumber: UINT32,
        SizeOfInfo: *mut UINTN,
        Info: *mut *mut EFI_GRAPHICS_OUTPUT_MODE_INFORMATION,
    ) -> EFI_STATUS,
    pub SetMode: unsafe extern "efiapi" fn(
        This: *mut EFI_GRAPHICS_OUTPUT_PROTOCOL,
        ModeNumber: UINT32,
    ) -> EFI_STATUS,
    pub Blt: unsafe extern "efiapi" fn(
        This: *mut EFI_GRAPHICS_OUTPUT_PROTOCOL,
        BltBuffer: *mut EFI_GRAPHICS_OUTPUT_BLT_PIXEL,
        BltOperation: EFI_GRAPHICS_OUTPUT_BLT_OPERATION,
        SourceX: UINTN,
        SourceY: UINTN,
        DestinationX: UINTN,
        DestinationY: UINTN,
        Width: UINTN,
        Height: UINTN,
        Delta: UINTN,
    ) -> EFI_STATUS,
    pub Mode: *mut EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE,
}

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE {
    pub MaxMode: UINT32,
    pub Mode: UINT32,
    pub Info: *mut EFI_GRAPHICS_OUTPUT_MODE_INFORMATION,
    pub SizeOfInfo: UINTN,
    pub FrameBufferBase: EFI_PHYSICAL_ADDRESS,
    pub FrameBufferSize: UINTN,
}

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_MODE_INFORMATION {
    pub Version: UINT32,
    pub HorizontalResolution: UINT32,
    pub VerticalResolution: UINT32,
    pub PixelFormat: EFI_GRAPHICS_PIXEL_FORMAT,
    pub PixelInformation: EFI_PIXEL_BITMASK,
    pub PixelsPerScanLine: UINT32,
}

#[repr(C)]
pub struct EFI_PIXEL_BITMASK {
    pub RedMask: UINT32,
    pub GreenMask: UINT32,
    pub BlueMask: UINT32,
    pub ReservedMask: UINT32,
}

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_BLT_PIXEL {
    pub Blue: UINT8,
    pub Green: UINT8,
    pub Red: UINT8,
    #[doc(hidden)]
    Reserved: UINT8,
}

impl EFI_GRAPHICS_OUTPUT_BLT_PIXEL {
    pub fn new(Blue: UINT8, Green: UINT8, Red: UINT8) -> Self {
        Self {
            Blue,
            Green,
            Red,
            Reserved: 0,
        }
    }
}
