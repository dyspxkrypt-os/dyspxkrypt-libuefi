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

use crate::types::{UINT32, UINT64};

/// The signature of the EFI System Table.
pub const EFI_SYSTEM_TABLE_SIGNATURE: UINT64 = 0x5453595320494249;

pub const EFI_2_100_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 100;
pub const EFI_2_90_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 90;
pub const EFI_2_80_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 80;
pub const EFI_2_70_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 70;
pub const EFI_2_60_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 60;
pub const EFI_2_50_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 50;
pub const EFI_2_40_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 40;
pub const EFI_2_30_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 30;
pub const EFI_2_20_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 20;
pub const EFI_2_10_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 10;
pub const EFI_2_00_SYSTEM_TABLE_REVISION: UINT32 = 2 << 16;
pub const EFI_1_10_SYSTEM_TABLE_REVISION: UINT32 = (1 << 16) | 10;
pub const EFI_1_02_SYSTEM_TABLE_REVISION: UINT32 = (1 << 16) | 2;

pub const EFI_SPECIFICATION_VERSION: UINT32 = EFI_SYSTEM_TABLE_REVISION;
pub const EFI_SYSTEM_TABLE_REVISION: UINT32 = EFI_2_100_SYSTEM_TABLE_REVISION;

/// The EFI System Table containing pointers to the runtime and boot services tables.
#[repr(C)]
pub struct EFI_SYSTEM_TABLE {}
