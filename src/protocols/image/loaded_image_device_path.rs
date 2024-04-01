/*
 * Dyspxkrypt LibUEFI: Raw bindings of UEFI that conforms to the definitions of the UEFI Specification.
 * Copyright (C) 2023-2024 HTGAzureX1212.
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

pub const EFI_LOADED_IMAGE_DEVICE_PATH_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xBC62157E,
        0x3E33,
        0x4FEC,
        [0x99, 0x20, 0x2D, 0x3B, 0x36, 0xD7, 0x50, 0xDF],
    )
};

pub type EFI_LOADED_IMAGE_DEVICE_PATH_PROTOCOL = EFI_DEVICE_PATH_PROTOCOL;
