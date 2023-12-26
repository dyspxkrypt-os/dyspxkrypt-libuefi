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

use crate::types::{EFI_GUID, VOID};

/// Contains a set of GUID/pointer pairs comprised of `ConfigurationTable` field in the EFI System Table.
#[repr(C)]
pub struct EFI_CONFIGURATION_TABLE {
    /// The 128-bit GUID value that uniquely identifies the system configuration table.
    pub VendorGuid: EFI_GUID,
    /// A pointer to the table associated with `VendorGuid`. Type of the memory that is used to store
    /// the table as well as whether this pointer is a physical address or a virtual address during
    /// runtime (whether or not a particular address reported in the table gets fixed up when a call
    /// to `SetVirtualAddressMap()` is made) is determined by the `VendorGuid`. Unless otherwise
    /// specified, memory type of the table buffer is defined by the guidelines set forth in the Calling
    /// Conventions section in Chapter 2. It is the responsibility of the specification defining the
    /// `VendorTable` to specify additional memory type requirements (if any) and whether to convert
    /// the addresses reported in the table. Any required address conversion is a responsibility
    /// of the driver that publishes corresponding configuration table.
    ///
    /// Whether this pointer is a physical address or a virtual address during runtime is determined
    /// by the `VendorGuid`. The `VendorGuid` associated with a given `VendorTable` pointer defines
    /// whether or not a particular address reported in the table gets fixed up when a call to
    /// `SetVirtualAddressMap()` is made. It is the responsibility of the specification defining the
    /// `VendorTable` to specify whether to convert the addresses reported in the table.
    pub VendorTable: *mut VOID,
}
