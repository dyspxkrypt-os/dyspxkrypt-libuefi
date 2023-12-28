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

use crate::tables::boot::EFI_PHYSICAL_ADDRESS;
use crate::types::{EFI_GUID, EFI_STATUS, UINT32, UINT8, UINTN};

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
    /// A pixel is 32-bits and byte zero represents red, byte one represents green, byte two represents
    /// blue, and byte three is reserved. This is the definition for the physical frame buffer. The
    /// byte values for the red, green, and blue components represent the color intensity. This color
    /// intensity value range from a minimum intensity of `0` to maximum intensity of `255`.
    PixelRedGreenBlueReserved8BitPerColor,
    /// A pixel is 32-bits and byte zero represents blue, byte one represents green, byte two represents
    /// red, and byte three is reserved. This is the definition for the physical frame buffer. The byte
    /// values for the red, green, and blue components represent the color intensity. This color intensity
    /// value range from a minimum intensity of 0 to maximum intensity of 255.
    PixelBlueGreenRedReserved8BitPerColor,
    /// The pixel definition of the physical frame buffer is defined by `EFI_PIXEL_BITMASK`.
    PixelBitMask,
    /// This mode does not support a physical frame buffer.
    PixelBltOnly,
    /// Valid `EFI_GRAPHICS_PIXEL_FORMAT` enum values are less than this value.
    PixelFormatMax,
}

#[repr(C)]
pub enum EFI_GRAPHICS_OUTPUT_BLT_OPERATION {
    /// Write data from the `BltBuffer` pixel `(0, 0)` directly to every pixel of the video display
    /// rectangle `(DestinationX, DestinationY)` `(DestinationX + Width, DestinationY + Height)`. Only
    /// one pixel will be used from the `BltBuffer`. `Delta` is NOT used.
    EfiBltVideoFill,
    /// Read data from the video display rectangle `(SourceX, SourceY)` `(SourceX + Width, SourceY + Height)`
    /// and place it in the `BltBuffer` rectangle `(DestinationX, DestinationY)` `(DestinationX + Width, DestinationY + Height)`.
    /// If `DestinationX` or `DestinationY` is not zero then `Delta` must be set to the length in bytes
    /// of a row in the `BltBuffer`.
    EfiBltVideoToBltBuffer,
    /// Write data from the `BltBuffer` rectangle `(SourceX, SourceY)` `(SourceX + Width, SourceY + Height)`
    /// directly to the video display rectangle `(DestinationX, DestinationY)` `(DestinationX + Width, DestinationY + Height)`.
    /// If `SourceX` or `SourceY` is not zero then `Delta` must be set to the length in bytes of a
    /// row in the `BltBuffer`.
    EfiBltBufferToVideo,
    /// Copy from the video display rectangle `(SourceX, SourceY)` `(SourceX + Width, SourceY + Height)`
    /// to the video display rectangle `(DestinationX, DestinationY)` `(DestinationX + Width, DestinationY + Height)`.
    /// The `BltBuffer` and `Delta` are not used in this mode. There is no limitation on the overlapping
    /// of the source and destination rectangles.
    EfiBltVideoToVideo,
    EfiGraphicsOutputBltOperationMax,
}

/// Provides a basic abstraction to set video modes and copy pixels to and from the graphics
/// controller’s frame buffer. The linear address of the hardware frame buffer is also exposed so
/// software can write directly to the video hardware.
#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_PROTOCOL {
    /// Returns information for an available graphics mode that the graphics device and the set of
    /// active video output devices supports.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_ABSOLUTE_POINTER_PROTOCOL` instance. |
    /// | **IN** `ModeNumber` | The mode number to return information on. The current mode and valid modes are read-only values in the Mode structure of the `EFI_GRAPHICS_OUTPUT_PROTOCOL`. |
    /// | **OUT** `SizeOfInfo` | A pointer to the size, in bytes, of the Info buffer. |
    /// | **OUT** `Info` | A pointer to a callee allocated buffer that returns information about `ModeNumber`. |
    ///
    /// ## Description
    ///
    /// The `QueryMode()` function returns information for an available graphics mode that the graphics
    /// device and the set of active video output devices supports. If `ModeNumber` is not between `0`
    /// and `MaxMode - 1`, then `EFI_INVALID_PARAMETER` is returned. `MaxMode` is available from the `Mode`
    /// structure of the `EFI_GRAPHICS_OUTPUT_PROTOCOL`.
    ///
    /// The size of the `Info` structure should never be assumed and the value of `SizeOfInfo` is the
    /// only valid way to know the size of `Info`.
    ///
    /// If the `EFI_GRAPHICS_OUTPUT_PROTOCOL` is installed on the handle that represents a single
    /// video output device, then the set of modes returned by this service is the subset of modes
    /// supported by both the graphics controller and the video output device.
    ///
    /// If the `EFI_GRAPHICS_OUTPUT_PROTOCOL` is installed on the handle that represents a combination
    /// of video output devices, then the set of modes returned by this service is the subset of modes
    /// supported by the graphics controller and the all of the video output devices represented by
    /// the handle.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | Valid mode information was returned. |
    /// | `EFI_DEVICE_ERROR` | A hardware error occurred trying to retrieve the video mode. |
    /// | `EFI_INVALID_PARAMETER` | `ModeNumber` is not valid. |
    pub QueryMode: unsafe extern "efiapi" fn(
        This: *mut EFI_GRAPHICS_OUTPUT_PROTOCOL,
        ModeNumber: UINT32,
        SizeOfInfo: *mut UINTN,
        Info: *mut *mut EFI_GRAPHICS_OUTPUT_MODE_INFORMATION,
    ) -> EFI_STATUS,
    /// Set the video device into the specified mode and clears the visible portions of the output
    /// display to black.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_ABSOLUTE_POINTER_PROTOCOL` instance. |
    /// | **IN** `ModeNumber` | Abstraction that defines the current video mode. |
    ///
    /// ## Description
    ///
    /// This `SetMode()` function sets the graphics device and the set of active video output devices
    /// to the video mode specified by `ModeNumber`. If `ModeNumber` is not supported `EFI_UNSUPPORTED`
    /// is returned.
    ///
    /// If a device error occurs while attempting to set the video mode, then `EFI_DEVICE_ERROR` is
    /// returned. Otherwise, the graphics device is set to the requested geometry, the set of active
    /// output devices are set to the requested geometry, the visible portion of the hardware frame
    /// buffer is cleared to black, and `EFI_SUCCESS` is returned.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The graphics mode specified by `ModeNumber` was selected. |
    /// | `EFI_DEVICE_ERROR` | The device had an error and could not complete the request. |
    /// | `EFI_UNSUPPORTED` | `ModeNumber` is not supported by this device. |
    pub SetMode: unsafe extern "efiapi" fn(
        This: *mut EFI_GRAPHICS_OUTPUT_PROTOCOL,
        ModeNumber: UINT32,
    ) -> EFI_STATUS,
    /// Blt a rectangle of pixels on the graphics screen. Blt stands for Block Transfer.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | A pointer to the `EFI_ABSOLUTE_POINTER_PROTOCOL` instance. |
    /// | **IN** `BltBuffer` | The data to transfer to the graphics screen. Size is at least `Width*Height*sizeof(EFI_GRAPHICS_OUTPUT_BLT_PIXEL)`. |
    /// | **IN** `BltOperation` | The operation to perform when copying `BltBuffer` on to the graphics screen. |
    /// | **IN** `SourceX`, `SourceY` | The X and Y coordinates of the source for the `BltOperation`. The origin of the screen is `0, 0` and that is the upper left-hand corner of the screen. `SourceY` The Y coordinate of the source for the `BltOperation`. |
    /// | **IN** `DestinationX`, `DestinationY` | The X and Y coordinates of the destination for the `BltOperation`. The origin of the screen is `0, 0` and that is the upper left-hand corner of the screen. |
    /// | **IN** `Width` | The width of a rectangle in the blt rectangle in pixels. Each pixel is represented by an `EFI_GRAPHICS_OUTPUT_BLT_PIXEL` element. |
    /// | **IN** `Height` | The height of a rectangle in the blt rectangle in pixels. Each pixel is represented by an `EFI_GRAPHICS_OUTPUT_BLT_PIXEL` element. |
    /// | **IN** `Delta` | Not used for `EfiBltVideoFill` or the `EfiBltVideoToVideo` operation. If a `Delta` of zero is used, the entire `BltBuffer` is being operated on. If a sub-rectangle of the `BltBuffer` is being used then `Delta` represents the number of bytes in a row of the `BltBuffer`. |
    ///
    /// ## Description
    ///
    /// The `Blt()` function is used to draw the `BltBuffer` rectangle onto the video screen.
    ///
    /// The `BltBuffer` represents a rectangle of `Height` by `Width` pixels that will be drawn on
    /// the graphics screen using the operation specified by `BltOperation`. The `Delta` value can be
    /// used to enable the `BltOperation` to be performed on a sub-rectangle of the `BltBuffer`.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | `BltBuffer` was drawn to the graphics screen. |
    /// | `EFI_INVALID_PARAMETER` | `BltOperation` is not valid. |
    /// | `EFI_DEVICE_ERROR` | The device had an error and could not complete the request. |
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
    /// Pointer to `EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE` data.
    pub Mode: *mut EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE,
}

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE {
    /// The number of modes supported by `QueryMode()` and `SetMode()`.
    pub MaxMode: UINT32,
    /// Current `Mode` of the graphics device. Valid mode numbers are `0` to `MaxMode-1`.
    pub Mode: UINT32,
    /// Pointer to read-only `EFI_GRAPHICS_OUTPUT_MODE_INFORMATION` data.
    pub Info: *mut EFI_GRAPHICS_OUTPUT_MODE_INFORMATION,
    /// Size of `Info` structure in bytes. Future versions of this specification may increase the
    /// size of the `EFI_GRAPHICS_OUTPUT_MODE_INFORMATION` data.
    pub SizeOfInfo: UINTN,
    /// Base address of graphics linear frame buffer. `Info` contains information required to allow
    /// software to draw directly to the frame buffer without using `Blt()`. Offset zero in
    /// `FrameBufferBase` represents the upper left pixel of the display.
    pub FrameBufferBase: EFI_PHYSICAL_ADDRESS,
    /// Amount of frame buffer needed to support the active mode as defined by
    /// `PixelsPerScanLine` x `VerticalResolution` x `PixelElementSize`.
    pub FrameBufferSize: UINTN,
}

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_MODE_INFORMATION {
    /// The version of this data structure. A value of zero represents the `EFI_GRAPHICS_OUTPUT_MODE_INFORMATION`
    /// structure as defined in this specification. Future version of this specification may extend
    /// this data structure in a backwards compatible way and increase the value of `Version`.
    pub Version: UINT32,
    /// The size of video screen in pixels in the X dimension.
    pub HorizontalResolution: UINT32,
    /// The size of video screen in pixels in the Y dimension.
    pub VerticalResolution: UINT32,
    /// Enumeration that defines the physical format of the pixel. A value of `PixelBltOnly` implies
    /// that a linear frame buffer is not available for this mode.
    pub PixelFormat: EFI_GRAPHICS_PIXEL_FORMAT,
    /// This bit-mask is only valid if `PixelFormat` is set to `PixelPixelBitMask`. A bit being set
    /// defines what bits are used for what purpose such as `Red`, `Green`, `Blue`, or `Reserved`.
    pub PixelInformation: EFI_PIXEL_BITMASK,
    /// Defines the number of pixel elements per video memory line. For performance reasons, or due
    /// to hardware restrictions, scan lines may be padded to an amount of memory alignment. These
    /// padding pixel elements are outside the area covered by `HorizontalResolution` and are not
    /// visible. For direct frame buffer access, this number is used as a span between starts of
    /// pixel lines in video memory. Based on the size of an individual pixel element and
    /// `PixelsPerScanline`, the offset in video memory from pixel element `(x, y)` to pixel element
    /// `(x, y+1)` has to be calculated as `sizeof(PixelElement) * PixelsPerScanLine`, not
    /// `sizeof(PixelElement) * HorizontalResolution`, though in many cases those values can coincide.
    /// This value can depend on video hardware and mode resolution. GOP implementation is responsible
    /// for providing accurate value for this field.
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
    /// Construct a new `EFI_GRAPHICS_OUTPUT_BLT_PIXEL`.
    pub fn new(Blue: UINT8, Green: UINT8, Red: UINT8) -> Self {
        Self {
            Blue,
            Green,
            Red,
            Reserved: 0,
        }
    }
}
