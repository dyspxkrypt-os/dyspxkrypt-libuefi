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

use core::ffi::c_void;
use core::ptr::null_mut;

/// A null, mutable pointer.
pub const NULL: *mut VOID = null_mut();

/// A logical Boolean value that occupies one byte.
///
/// `TRUE` corresponds to `1` and `FALSE` corresponds to `0`. Any other values are invalid and undefined.
pub type BOOLEAN = u8;

/// A signed integer of native width.
///
/// This means 4 bytes on supported 32-bit platforms, 8 bytes on supported
/// 64-bit platforms and 16 bytes on supported 128-bit platforms.
pub type INTN = isize;

/// An unsigned integer of native width.
///
/// This means 4 bytes on supported 32-bit platforms, 8 bytes on supported
/// 64-bit platforms and 16 bytes on supported 128-bit platforms.
pub type UINTN = usize;

/// A signed 1-byte integer.
///
/// It can be used to represent integers in the range of \[`-128`, `127`].
pub type INT8 = i8;

/// An unsigned 1-byte integer.
///
/// It can be used to represent integers in the range of \[`0`, `255`].
pub type UINT8 = u8;

/// A signed 2-byte integer.
///
/// It can be used to represent integers in the range of \[`-32768`, `32767`].
pub type INT16 = i16;

/// An unsigned 2-byte integer.
///
/// It can be used to represent integers in the range of \[`0`, `65535`].
pub type UINT16 = u16;

/// A signed 4-byte integer.
///
/// It can be used to represent integers in the range of \[`-2147483648`, `2147483647`].
pub type INT32 = i32;

/// An unsigned 4-byte integer.
///
/// It can be used to represent integers in the range of \[`0`, `4294967295`].
pub type UINT32 = u32;

/// A signed 8-byte integer.
///
/// It can be used to represent integers in the range of \[`-9223372036854775808`, `9223372036854775807`].
pub type INT64 = i64;

/// An unsigned 8-byte integer.
///
/// It can be used to represent integers in the range of \[`0`, `18446744073709551615`].
pub type UINT64 = u64;

/// A signed 16-byte integer.
///
/// It can be used to represent integers in the range of \[`-170141183460469231731687303715884105728`, `170141183460469231731687303715884105727`].
pub type INT128 = i64;

/// An unsigned 16-byte integer.
///
/// It can be used to represent integers in the range of \[`0`, `340282366920938463463374607431768211455`].
pub type UINT128 = u64;

/// A 1-byte character.
///
/// Unless otherwise specified, all 1-byte or ASCII characters and strings are stored in 8-bit ASCII encoding format, using the
/// ISO-Latin-1 character set.
pub type CHAR8 = u8;

/// A 2-byte character.
///
/// Unless otherwise specified all characters and strings are stored in the UCS-2 encoding format as defined by Unicode 2.1 and
/// ISO/IEC 10646 standards.
pub type CHAR16 = u16;

/// An undeclared or unknown type.
pub type VOID = c_void;

/// A 128-bit buffer containing a globally-unique identifier value.
///
/// Unless otherwise specified, aligned on a 64-bit boundary.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EFI_GUID {
    pub Data1: UINT32,
    pub Data2: UINT16,
    pub Data3: UINT16,
    pub Data4: [UINT8; 8],
}

impl EFI_GUID {
    /// Convenience function to construct a new GUID from its raw parts.
    ///
    /// It is the caller's responsibility to ensure that the GUID constructed is not a malformed one.
    pub const unsafe fn from_raw_parts(
        Data1: UINT32,
        Data2: UINT16,
        Data3: UINT16,
        Data4: [UINT8; 8],
    ) -> Self {
        Self {
            Data1,
            Data2,
            Data3,
            Data4,
        }
    }
}

/// A status code.
pub type EFI_STATUS = UINTN;

/// A handle to a collection of related interfaces.
pub type EFI_HANDLE = *mut VOID;

/// A handle to an event structure.
pub type EFI_EVENT = *mut VOID;

/// A logical block address.
pub type EFI_LBA = UINT64;

/// A task priority level.
pub type EFI_TPL = UINTN;

/// A 32-byte buffer containing a network Media Access Control address.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EFI_MAC_ADDRESS(pub [UINT8; 32]);

/// A 4-byte buffer containing an IPv4 internet protocol address.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EFI_IPv4_ADDRESS(pub [UINT8; 4]);

/// A 16-byte buffer containing an IPv6 internet protocol address.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EFI_IPv6_ADDRESS(pub [UINT8; 16]);

/// A 16-byte buffer aligned on a 4-byte boundary, containing either an IPv4 or IPv6 internet protocol address.
#[derive(Copy, Clone)]
#[repr(C)]
pub union EFI_IP_ADDRESS {
    __: [UINT32; 4],
    /// An IPv4 internet protocol address.
    pub v4: EFI_IPv4_ADDRESS,
    /// An IPv6 internet protocol address.
    pub v6: EFI_IPv6_ADDRESS,
}
