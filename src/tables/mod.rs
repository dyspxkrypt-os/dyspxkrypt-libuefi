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

//! # EFI Tables
//!
//! This module contains definitions to various tables defined by the UEFI specification.

use crate::types::{UINT32, UINT64};

/// The EFI table header is the data structure that precedes all of the standard EFI table types.
///
/// ## Notes
///
/// - The capabilities found in the system table, runtime table and boot services table may change
/// over time. The header of these tables’s `Revision` field is incremented when new capabilities and
/// functions are added to the table. When checking for capabilities, code should verify that `Revision`
/// is **greater than or equal to** the revision level of the table at the point when the capabilities
/// were added to the UEFI specification.
///
/// - Unless otherwise specified, UEFI uses a standard CCITT32 CRC algorithm with a seed polynomial
/// value of `0x04C11DB7` for its CRC calculations.
///
/// - The size of the system table, runtime services table, and boot services table may increase over
/// time. It is very important to always use the `HeaderSize` field of the header to determine the size
/// of these tables.
#[repr(C)]
pub struct EFI_TABLE_HEADER {
    /// A 64-bit signature that identifies the type of table that follows. Unique signatures have been
    /// generated for the EFI System Table, the EFI Boot Services Table as well as the EFI Runtime Services
    /// Table.
    pub Signature: UINT64,
    /// The revision of the EFI Specification to which this table conforms. The upper 16 bits of this field
    /// contain **the major revision value**, and the lower 16 bits contain **the minor revision value**.
    /// The minor revision values are binary coded decimals and are limited to the range of \[`00`, `99`].
    ///
    /// When printed or displayed, the UEFI spec revision is referred to as `(Major revision).(Minor revision upper decimal).(Minor revision lower decimal)`
    /// or `(Major revision).(Minor revision upper decimal)` in case the `Minor revision lower decimal` is
    /// set to `0`. For example:
    ///
    /// - a specification with the revision value `((2 << 16) | (30))` would be referred to as `2.3`; and
    ///
    /// - a specification with the revision value `((2 << 16) | (31))` would be referred as `2.3.1`.
    pub Revision: UINT32,
    /// The size, in bytes, of the entire table including its header.
    pub HeaderSize: UINT32,
    /// The 32-bit CRC for the entire table. This value is computed by setting this field to 0, and computing
    /// the 32-bit CRC for `HeaderSize` bytes.
    pub CRC32: UINT32,
    /// Reserved field that must be set to `0`.
    pub Reserved: UINT32,
}
