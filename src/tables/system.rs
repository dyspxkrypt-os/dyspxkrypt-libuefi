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

use crate::protocols::console::{EFI_SIMPLE_TEXT_INPUT_PROTOCOL, EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL};
use crate::tables::boot::EFI_BOOT_SERVICES;
use crate::tables::configuration::EFI_CONFIGURATION_TABLE;
use crate::tables::runtime::EFI_RUNTIME_SERVICES;
use crate::tables::EFI_TABLE_HEADER;
use crate::types::{CHAR16, EFI_HANDLE, UINT32, UINT64, UINTN};

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
pub struct EFI_SYSTEM_TABLE {
    /// The table header for the EFI System Table. This header contains the `EFI_SYSTEM_TABLE_SIGNATURE`
    /// and `EFI_SYSTEM_TABLE_REVISION` values along with the size of the `EFI_SYSTEM_TABLE` structure and
    /// a 32-bit CRC to verify that the contents of the EFI System Table are valid.
    pub Hdr: EFI_TABLE_HEADER,
    /// A pointer to a null-terminated string that identifies the vendor that produces the system firmware
    /// for the platform.
    pub FirmwareVendor: *mut CHAR16,
    /// A firmware vendor specific value that identifies the revision of the system firmware for the platform.
    pub FirmwareRevision: UINT32,
    /// The handle for the active console input device. This handle must support `EFI_SIMPLE_TEXT_INPUT_PROTOCOL`
    /// and `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`. If there is no active console, these protocols must still
    /// be present.
    pub ConsoleInHandle: EFI_HANDLE,
    /// A pointer to the `EFI_SIMPLE_TEXT_INPUT_PROTOCOL` interface that is associated with `ConsoleInHandle`.
    pub ConIn: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    /// The handle for the active console output device. This handle must support the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
    ///
    /// If there is no active console, this protocol must still be present.
    pub ConsoleOutHandle: EFI_HANDLE,
    /// A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` interface that is associated with `ConsoleOutHandle`.
    pub ConOut: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    /// The handle for the active standard error console device. This handle must support the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`.
    /// If there is no active console, this protocol must still be present.
    pub StandardErrorHandle: EFI_HANDLE,
    /// A pointer to the `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL` interface that is associated with `StandardErrorHandle`.
    pub StdErr: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    /// A pointer to the EFI Runtime Services Table.
    pub RuntimeServices: *mut EFI_RUNTIME_SERVICES,
    /// A pointer to the EFI Boot Services Table.
    pub BootServices: *mut EFI_BOOT_SERVICES,
    /// The number of system configuration tables in the buffer `ConfigurationTable`.
    pub NumberOfTableEntries: UINTN,
    /// A pointer to the system configuration tables. The number of entries in the table is `NumberOfTableEntries`.
    pub ConfigurationTable: *mut EFI_CONFIGURATION_TABLE,
}
