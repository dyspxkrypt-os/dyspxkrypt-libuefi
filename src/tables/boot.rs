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

pub const EFI_MEMORY_UC: UINT64 = 0x0000000000000001;
pub const EFI_MEMORY_WC: UINT64 = 0x0000000000000002;
pub const EFI_MEMORY_WT: UINT64 = 0x0000000000000004;
pub const EFI_MEMORY_WB: UINT64 = 0x0000000000000008;
pub const EFI_MEMORY_UCE: UINT64 = 0x0000000000000010;
pub const EFI_MEMORY_WP: UINT64 = 0x0000000000001000;
pub const EFI_MEMORY_RP: UINT64 = 0x0000000000002000;
pub const EFI_MEMORY_XP: UINT64 = 0x0000000000004000;
pub const EFI_MEMORY_NV: UINT64 = 0x0000000000008000;
pub const EFI_MEMORY_MORE_RELIABLE: UINT64 = 0x0000000000010000;
pub const EFI_MEMORY_RO: UINT64 = 0x0000000000020000;
pub const EFI_MEMORY_SP: UINT64 = 0x0000000000040000;
pub const EFI_MEMORY_CPU_CRYPTO: UINT64 = 0x0000000000080000;
pub const EFI_MEMORY_RUNTIME: UINT64 = 0x8000000000000000;
pub const EFI_MEMORY_ISA_VALID: UINT64 = 0x4000000000000000;
pub const EFI_MEMORY_ISA_MASK: UINT64 = 0x0FFFF00000000000;

/// The EFI Boot Services containing a table header and pointers to all of the boot services.
#[repr(C)]
pub struct EFI_BOOT_SERVICES;

/// A descriptor for a memory map.
#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
    /// Type of the memory region.
    pub Type: UINT32,
    /// Physical address of the first byte in the memory region. `PhysicalStart` must be aligned on a
    /// 4 KiB boundary, and must not be above `0xFFFFFFFFFFFFF000`.
    pub PhysicalStart: EFI_PHYSICAL_ADDRESS,
    /// Virtual address of the first byte in the memory region. `VirtualStart` must be aligned on a
    /// 4 KiB boundary, and must not be above `0xFFFFFFFFFFFFF000`.
    pub VirtualStart: EFI_VIRTUAL_ADDRESS,
    /// Number of 4 KiB pages in the memory region. `NumberOfPages` must not be `0`, and must not be
    /// any value that would represent a memory page with a start address, either physical or virtual,
    /// above `0xFFFFFFFFFFFFF000`.
    pub NumberOfPages: UINT64,
    /// Attributes of the memory region that describe the bit mask of capabilities for that memory region,
    /// and not necessarily the current settings for that memory region.
    pub Attribute: UINT64,
}

/// A physical address.
pub type EFI_PHYSICAL_ADDRESS = UINT64;
/// A virtual address.
pub type EFI_VIRTUAL_ADDRESS = UINT64;
