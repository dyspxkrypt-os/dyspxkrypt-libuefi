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

use crate::protocols::media::block::EFI_LBA;
use crate::types::{CHAR16, EFI_GUID, UINT64, UINT8};

#[repr(C)]
pub struct EFI_PARTITION_ENTRY {
    pub PartitionTypeGUID: EFI_GUID,
    pub UniquePartitionGUID: EFI_GUID,
    pub StartingLBA: EFI_LBA,
    pub EndingLBA: EFI_LBA,
    pub Attributes: UINT64,
    pub PartitionName: [CHAR16; 36],
}

#[repr(C)]
pub struct MBR_PARTITION_RECORD {
    pub BootIndicator: UINT8,
    pub StartHead: UINT8,
    pub StartSector: UINT8,
    pub StartTrack: UINT8,
    pub OSIndicator: UINT8,
    pub EndHead: UINT8,
    pub EndSector: UINT8,
    pub EndTrack: UINT8,
    pub StartingLBA: [UINT8; 4],
    pub SizeInLBA: [UINT8; 4],
}
