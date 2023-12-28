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

use crate::types::{EFI_GUID, EFI_STATUS, UINT32, UINTN};

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
    PixelFormatMax
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
