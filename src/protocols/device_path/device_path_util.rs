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

use crate::protocols::device_path::EFI_DEVICE_PATH_PROTOCOL;
use crate::types::{BOOLEAN, EFI_GUID, UINT16, UINT8, UINTN};

pub const EFI_DEVICE_PATH_UTILITIES_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x0379BE4E,
        0xD706,
        0x437D,
        [0xB0, 0x37, 0xED, 0xB8, 0x2F, 0xB7, 0x72, 0xA4],
    )
};

/// Creates and manipulates device paths and device nodes.
///
/// The `EFI_DEVICE_PATH_UTILITIES_PROTOCOL` provides common utilities for creating a manipulating
/// device paths and device nodes.
#[repr(C)]
pub struct EFI_DEVICE_PATH_UTILITIES_PROTOCOL {
    /// Returns the size of the device path, in bytes.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `DevicePath` | Points to the start of the EFI device path. |
    ///
    /// ## Description
    ///
    /// This function returns the size of the specified device path, in bytes, including the end-of-path
    /// tag. If `DevicePath` is `NULL` then zero is returned.
    pub GetDevicePathSize: unsafe extern "efiapi" fn(
        DevicePath: *const EFI_DEVICE_PATH_PROTOCOL,
    ) -> UINTN,
    /// Create a duplicate of the specified path.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `DevicePath` | Points to the start of the EFI device path. |
    ///
    /// ## Description
    ///
    /// This function creates a duplicate of the specified device path. The memory is allocated from
    /// EFI boot services memory. It is the responsibility of the caller to free the memory allocated.
    /// If `DevicePath` is `NULL` then `NULL` will be returned and no memory will be allocated.
    ///
    /// ## Returns
    ///
    /// This function returns a pointer to the duplicate device path or `NULL` if there was
    /// insufficient memory.
    pub DuplicateDevicePath: unsafe extern "efiapi" fn(
        DevicePath: *const EFI_DEVICE_PATH_UTILITIES_PROTOCOL,
    ) -> *mut EFI_DEVICE_PATH_UTILITIES_PROTOCOL,
    /// Create a new path by appending the second device path to the first.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `Src1` | Points to the first device path. |
    /// | **IN** `Src2` | Points to the second device path. |
    ///
    /// ## Description
    ///
    /// This function creates a new device path by appending a copy of the second device path to a
    /// copy of the first device path in a newly allocated buffer. Only the end-of-device-path device
    /// node from the second device path is retained. If `Src1` is `NULL` and `Src2` is non-`NULL`,
    /// then a duplicate of `Src2` is returned. If `Src1` is non-`NULL` and `Src2` is `NULL`, then a
    /// duplicate of `Src1` is returned. If `Src1` and `Src2` are both `NULL`, then a copy of an
    /// end-of-device-path is returned.
    ///
    /// The memory is allocated from EFI boot services memory. It is the responsibility of the caller
    /// to free the memory allocated.
    ///
    /// ## Returns
    ///
    /// This function returns a pointer to the newly created device path or `NULL` if memory could not
    /// be allocated.
    pub AppendDevicePath: unsafe extern "efiapi" fn(
        Src1: *const EFI_DEVICE_PATH_PROTOCOL,
        Src2: *const EFI_DEVICE_PATH_PROTOCOL,
    ) -> *mut EFI_DEVICE_PATH_PROTOCOL,
    /// Creates a new path by appending the device node to the device path.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `DevicePath` | Points to the device path. |
    /// | **IN** `DeviceNode` | Points to the device node. |
    ///
    /// ## Description
    ///
    /// This function creates a new device path by appending a copy of the second device path to a
    /// copy of the first device path in a newly allocated buffer. Only the end-of-device-path device
    /// node from the second device path is retained. If `DevicePath` is `NULL` and `DeviceNode` is
    /// non-`NULL`, then a duplicate of `DevicePath` is returned. If `DevicePath` is non-`NULL` and
    /// `DeviceNode` is `NULL`, then a duplicate of `DevicePath` is returned. If `DevicePath` and
    /// `DeviceNode` are both `NULL`, then a copy of an end-of-device-path is returned.
    ///
    /// The memory is allocated from EFI boot services memory. It is the responsibility of the caller
    /// to free the memory allocated.
    ///
    /// ## Returns
    ///
    /// This function returns a pointer to the newly created device path or `NULL` if memory could not
    /// be allocated.
    pub AppendDeviceNode: unsafe extern "efiapi" fn(
        DevicePath: *const EFI_DEVICE_PATH_PROTOCOL,
        DeviceNode: *const EFI_DEVICE_PATH_PROTOCOL,
    ) -> *mut EFI_DEVICE_PATH_PROTOCOL,
    /// Creates a new path by appending the specified device path instance to the specified device
    /// path.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `DevicePath` | Points to the device path. If `NULL`, then ignored. |
    /// | **IN** `DevicePathInstance` | Points to the device path instance. |
    ///
    /// ## Description
    ///
    /// This function creates a new device path by appending a copy of the specified device path instance
    /// to a copy of the specified device path in an allocated buffer. The end-of-device-path device
    /// node is moved after the end of the appended device node and a new end-of-device-path-instance
    /// node is inserted between. If `DevicePath` is `NULL`, then a copy of `DevicePathInstance` is
    /// returned instead.
    ///
    /// The memory is allocated from EFI boot services memory. It is the responsibility of the caller
    /// to free the memory allocated.
    ///
    /// ## Returns
    ///
    /// This function returns a pointer to the newly created device path or `NULL` if `DevicePathInstance`
    /// is `NULL` or there was insufficient memory.
    pub AppendDevicePathInstance: unsafe extern "efiapi" fn(
        DevicePath: *const EFI_DEVICE_PATH_PROTOCOL,
        DevicePathInstance: *const EFI_DEVICE_PATH_PROTOCOL,
    ) -> *mut EFI_DEVICE_PATH_PROTOCOL,
    /// Creates a copy of the current device path instance and returns pointer to the next device path
    /// instance.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN OUT** `DevicePathInstance` | On input, this holds the pointer to the current device path instance. On output, this holds the pointer to the next device path instance or `NULL` if there are no more device path instances in the device path. |
    /// | **OUT** `DevicePathInstanceSize` | On output, this holds the size of the device path instance, in bytes or zero, if `DevicePathInstance` is `NULL`. If `NULL`, then the instance size is not output. |
    ///
    /// ## Description
    ///
    /// This function creates a copy of the current device path instance. It also updates `DevicePathInstance`
    /// to point to the next device path instance in the device path (or `NULL` if no more) and updates
    /// `DevicePathInstanceSize` to hold the size of the device path instance copy.
    ///
    /// The memory is allocated from EFI boot services memory. It is the responsibility of the caller
    /// to free the memory allocated.
    ///
    /// ## Returns
    ///
    /// This function returns a pointer to the copy of the current device path instance or `NULL` if
    /// `DevicePathInstance` was `NULL` on entry or there was insufficient memory.
    pub GetNextDevicePathInstance: unsafe extern "efiapi" fn(
        DevicePathInstance: *mut *mut EFI_DEVICE_PATH_PROTOCOL,
        DevicePathInstanceSize: *mut UINTN,
    ) -> *mut EFI_DEVICE_PATH_PROTOCOL,
    /// Creates a device node.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `NodeType` | `NodeType` is the device node type (`EFI_DEVICE_PATH_PROTOCOL.Type`) for the new device node. |
    /// | **IN** `NodeSubType` | `NodeSubType` is the device node sub-type (`EFI_DEVICE_PATH_PROTOCOL.SubType`) for the new device node. |
    /// | **IN** `NodeLength` | `NodeLength` is the length of the device node (`EFI_DEVICE_PATH_PROTOCOL.Length`) for the new device node. |
    ///
    /// ## Description
    ///
    /// This function creates a new device node in a newly allocated buffer.
    ///
    /// The memory is allocated from EFI boot services memory. It is the responsibility of the caller
    /// to free the memory allocated.
    ///
    /// ## Returns
    ///
    /// This function returns a pointer to the created device node or `NULL` if `NodeLength` is less
    /// than the size of the header or there was insufficient memory.
    pub CreateDeviceNode: unsafe extern "efiapi" fn(
        NodeType: UINT8,
        NodeSubType: UINT8,
        NodeLength: UINT16,
    ) -> *mut EFI_DEVICE_PATH_PROTOCOL,
    /// Returns whether a device path is multi-instance.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `DevicePath` | Points to the device path. If `NULL`, then ignored. |
    ///
    /// ## Description
    ///
    /// This function returns whether the specified device path has multiple path instances.
    ///
    /// ## Returns
    ///
    /// This function returns `TRUE` if the device path has more than one instance or `FALSE` if it
    /// is empty or contains only a single instance.
    pub IsDevicePathMultiInstance: unsafe extern "efiapi" fn(
        DevicePath: *const EFI_DEVICE_PATH_PROTOCOL,
    ) -> BOOLEAN,
}
