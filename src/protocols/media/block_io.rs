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

use crate::types::{BOOLEAN, EFI_GUID, EFI_STATUS, UINT32, UINT64};

pub const EFI_BLOCK_IO_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0x964E5B21,0x6459,0x11D2,
        [0x8E,0x39,0x00,0xA0,0xC9,0x69,0x72,0x3B]
    )
};

pub const EFI_BLOCK_IO_PROTOCOL_REVISION: UINT64 = 0x00010000;
#[cfg(feature = "media-block-v2")]
#[cfg_attr(doc, doc(cfg(feature = "media-block-v2")))]
#[cfg_attr(docsrs, doc(cfg(feature = "media-block-v2")))]
pub const EFI_BLOCK_IO_PROTOCOL_REVISION2: UINT64 = 0x00020000;
#[cfg(feature = "media-block-v3")]
#[cfg_attr(doc, doc(cfg(feature = "media-block-v3")))]
#[cfg_attr(docsrs, doc(cfg(feature = "media-block-v3")))]
pub const EFI_BLOCK_IO_PROTOCOL_REVISION3: UINT64 = (2 << 16) | 31;

/// This protocol provides control over block devices.
///
/// This section defines the Block I/O protocol. This protocol is used to abstract mass storage
/// devices to allow code running in the EFI boot services environment to access them without
/// specific knowledge of the type of device or controller that manages the device. Functions are
/// defined to read and write data at a block level from mass storage devices as well as to manage
/// such devices in the EFI boot services environment.
#[repr(C)]
pub struct EFI_BLOCK_IO_PROTOCOL {
    /// The revision to which the block IO interface adheres.
    pub Revision: UINT64,
    /// A pointer to the EFI_BLOCK_IO_MEDIA data for this device.
    pub Media: *mut EFI_BLOCK_IO_MEDIA,
    /// Resets the block device hardware.
    ///
    /// ## Parameters
    ///
    /// | Parameter                     | Description                                                                                                |
    /// | ----------------------------- | ---------------------------------------------------------------------------------------------------------- |
    /// | **IN** `This` | Indicates a pointer to the calling context. |
    /// | **IN** `ExtendedVerification` | Indicates that the driver may perform a more exhaustive verification operation of the device during reset. |
    ///
    /// ## Description
    ///
    /// The `Reset()` function resets the block device hardware.
    ///
    /// As part of the initialization process, the firmware/device will make a quick but reasonable
    /// attempt to verify that the device is functioning. If the `ExtendedVerification` flag is
    /// `TRUE` the firmware may take an extended amount of time to verify the device is operating
    /// on reset. Otherwise the reset operation is to occur as quickly as possible.
    ///
    /// The hardware verification process is not defined by this specification and is left up to
    /// the platform firmware or driver to implement.
    ///
    /// ## Status Codes Returned
    ///
    /// | Status Code        | Description                                                     |
    /// | ------------------ | --------------------------------------------------------------- |
    /// | `EFI_SUCCESS` | The block device was reset. |
    /// | `EFI_DEVICE_ERROR` | The block device is not functioning correctly and could not be reset. |
    pub Reset: unsafe extern "efiapi" fn(
        This: *mut EFI_BLOCK_IO_PROTOCOL,
        ExtendedVerification: BOOLEAN,
    ) -> EFI_STATUS,
}

#[repr(C)]
pub struct EFI_BLOCK_IO_MEDIA {
    /// The current media ID. If the media changes, this value is changed.
    pub MediaId: UINT32,
    /// `TRUE` if the media is removable; otherwise, `FALSE`.
    pub RemovableMedia: BOOLEAN,
    /// `TRUE` if there is a media currently present in the device; otherwise, `FALSE`. This field shows the media
    /// present status as of the most recent `EFI_BLOCK_IO_PROTOCOL.ReadBlocks()` or `WriteBlocks()` call.
    pub MediaPresent: BOOLEAN,
    /// `TRUE` if the `EFI_BLOCK_IO_PROTOCOL` was produced to abstract partition structures on the disk. `FALSE` if the
    /// `EFI_BLOCK_IO_PROTOCOL` was produced to abstract the logical blocks on a hardware device.
    pub LogicalPartition: BOOLEAN,
    /// `TRUE` if the media is marked read-only; otherwise, `FALSE`. This field shows the read-only status as of the
    /// most recent `EFI_BLOCK_IO_PROTOCOL.WriteBlocks()` call.
    pub ReadOnly: BOOLEAN,
    /// `TRUE` if the `WriteBlocks()` function caches write data.
    pub WriteCaching: BOOLEAN,
    /// The intrinsic block size of the device. If the media changes, then this field is updated. Returns the number of
    /// bytes per logical block. For ATA devices, this is reported in IDENTIFY DEVICE data words 117-118 (i.e., Words per
    /// Logical Sector) (see ATA8-ACS). For SCSI devices, this is reported in the READ CAPACITY (16) parameter data
    /// Logical Block Length In Bytes field (see SBC-3).
    pub BlockSize: UINT32,
    /// Supplies the alignment requirement for any buffer used in a data transfer. `IoAlign` values of 0 and 1 mean that
    /// the buffer can be placed anywhere in memory. Otherwise, `IoAlign` must be a power of 2, and the requirement is
    /// that the start address of a buffer must be evenly divisible by `IoAlign` with no remainder.
    pub IoAlign: UINT32,
    /// The last LBA on the device. If the media changes, then this field is updated. For ATA devices, this is reported
    /// in IDENTIFY DEVICE data words 60-61 (i.e., Total number of user addressable logical sectors) (see ATA8-ACS) minus
    /// one. For SCSI devices, this is reported in the READ CAPACITY (16) parameter data Returned Logical Block Address
    /// field (see SBC-3) minus one.
    pub LastBlock: EFI_LBA,
    #[cfg(feature = "media-block-v2")]
    #[cfg_attr(doc, doc(cfg(feature = "media-block-v2")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "media-block-v2")))]
    /// Only present if `EFI_BLOCK_IO_PROTOCOL.Revision` is greater than or equal to `EFI_BLOCK_IO_PROTOCOL_REVISION2`.
    /// Returns the first LBA that is aligned to a physical block boundary (See GPT overview). Note that this field
    /// follows the SCSI definition, not the ATA definition. If `LogicalPartition` is `TRUE` this value will be zero.
    pub LowestAlignedLba: EFI_LBA,
    #[cfg(feature = "media-block-v2")]
    #[cfg_attr(doc, doc(cfg(feature = "media-block-v2")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "media-block-v2")))]
    /// Only present if `EFI_BLOCK_IO_PROTOCOL.Revision` is greater than or equal to `EFI_BLOCK_IO_PROTOCOL_REVISION2`.
    /// Returns the number of logical blocks per physical block (See GPT overview) . Unlike the ATA and SCSI fields that
    /// provide the information for this field, this field does not contain an exponential value. A value of 0 means
    /// there is either one logical block per physical block, or there are more than one physical block per logical
    /// block. If `LogicalPartition` is TRUE this value will be zero.
    pub LogicalBlocksPerPhysicalBlock: UINT32,
    #[cfg(feature = "media-block-v3")]
    #[cfg_attr(doc, doc(cfg(feature = "media-block-v3")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "media-block-v3")))]
    /// Only present if `EFI_BLOCK_IO_PROTOCOL.Revision` is greater than or equal to `EFI_BLOCK_IO_PROTOCOL_REVISION3`.
    /// Returns the optimal transfer length granularity as a number of logical blocks (See GPT overview). A value of 0
    /// means there is no reported optimal transfer length granularity. If `LogicalPartition` is `TRUE` this value will
    /// be zero.
    pub OptimalTransferLengthGranularity: UINT32,
}

pub type EFI_LBA = UINT64;
