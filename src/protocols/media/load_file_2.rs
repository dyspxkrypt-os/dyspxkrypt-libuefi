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

use crate::protocols::media::EFI_LOAD_FILE_PROTOCOL;
use crate::types::EFI_GUID;

pub const EFI_LOAD_FILE2_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x4006C0C1,
        0xFCB3,
        0x403E,
        [0x99, 0x6D, 0x4A, 0x6C, 0x87, 0x24, 0xE0, 0x6D],
    )
};

pub type EFI_LOAD_FILE2_PROTOCOL = EFI_LOAD_FILE_PROTOCOL;
