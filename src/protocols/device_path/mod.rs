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

#[cfg(any(feature = "dev-path-full"))]
#[cfg_attr(doc, doc(cfg(feature = "dev-path-full")))]
#[cfg_attr(docsrs, doc(cfg(feature = "dev-path-full")))]
pub mod device_path_util;

#[cfg(any(feature = "dev-path-full"))]
#[cfg_attr(doc, doc(cfg(feature = "dev-path-full")))]
#[cfg_attr(docsrs, doc(cfg(feature = "dev-path-full")))]
pub use device_path_util::EFI_DEVICE_PATH_UTILITIES_PROTOCOL;

use crate::types::{EFI_GUID, UINT8};

pub const EFI_DEVICE_PATH_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x09576E91,
        0x6D3F,
        0x11D2,
        [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    )
};

#[repr(C)]
pub struct EFI_DEVICE_PATH_PROTOCOL {
    pub Type: UINT8,
    pub SubType: UINT8,
    pub Length: [UINT8; 2],
}
