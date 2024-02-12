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

#[cfg(feature = "media-ata")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-ata", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-ata", feature = "media-full"))))]
pub mod ata;
#[cfg(feature = "media-block")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-block", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-block", feature = "media-full"))))]
pub mod block;
#[cfg(feature = "media-block-2")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-block-2", feature = "media-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "media-block-2", feature = "media-full")))
)]
pub mod block_2;
#[cfg(feature = "media-crypt")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-crypt", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-crypt", feature = "media-full"))))]
pub mod crypt;
#[cfg(feature = "media-disk")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-disk", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-disk", feature = "media-full"))))]
pub mod disk;
#[cfg(feature = "media-disk-2")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-disk-2", feature = "media-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "media-disk-2", feature = "media-full")))
)]
pub mod disk_2;
#[cfg(feature = "media-erase")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-erase", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-erase", feature = "media-full"))))]
pub mod erase;
#[cfg(feature = "media-file")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-file", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-file", feature = "media-full"))))]
pub mod file;
#[cfg(feature = "media-file")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-file", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-file", feature = "media-full"))))]
pub mod load_file;
#[cfg(feature = "media-file")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-file", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-file", feature = "media-full"))))]
pub mod load_file_2;
#[cfg(feature = "media-nvme")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-nvme", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-nvme", feature = "media-full"))))]
pub mod nvme;
#[cfg(feature = "media-partition")]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "media-partition", feature = "media-full")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "media-partition", feature = "media-full")))
)]
pub mod partition;
#[cfg(feature = "media-ramdisk")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-ramdisk", feature = "media-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "media-ramdisk", feature = "media-full")))
)]
pub mod ramdisk;
#[cfg(feature = "media-sd-mmc")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-sd-mmc", feature = "media-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "media-sd-mmc", feature = "media-full")))
)]
pub mod sdmmc;
#[cfg(feature = "media-fs")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-fs", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-fs", feature = "media-full"))))]
pub mod simple_filesystem;
#[cfg(feature = "media-storesec")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-storesec", feature = "media-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "media-storesec", feature = "media-full")))
)]
pub mod storesec;
#[cfg(feature = "media-tape")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-tape", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-tape", feature = "media-full"))))]
pub mod tape;

#[cfg(feature = "media-ata")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-ata", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-ata", feature = "media-full"))))]
pub use ata::EFI_ATA_PASS_THRU_PROTOCOL;
#[cfg(feature = "media-block")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-block", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-block", feature = "media-full"))))]
pub use block::EFI_BLOCK_IO_PROTOCOL;
#[cfg(feature = "media-block-2")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-block-2", feature = "media-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "media-block-2", feature = "media-full")))
)]
pub use block_2::EFI_BLOCK_IO2_PROTOCOL;
#[cfg(feature = "media-crypt")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-crypt", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-crypt", feature = "media-full"))))]
pub use crypt::EFI_BLOCK_IO_CRYPTO_PROTOCOL;
#[cfg(feature = "media-disk")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-disk", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-disk", feature = "media-full"))))]
pub use disk::EFI_DISK_IO_PROTOCOL;
#[cfg(feature = "media-disk-2")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-disk-2", feature = "media-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "media-disk-2", feature = "media-full")))
)]
pub use disk_2::EFI_DISK_IO2_PROTOCOL;
#[cfg(feature = "media-erase")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-erase", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-erase", feature = "media-full"))))]
pub use erase::EFI_ERASE_BLOCK_PROTOCOL;
#[cfg(feature = "media-file")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-file", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-file", feature = "media-full"))))]
pub use file::EFI_FILE_PROTOCOL;
#[cfg(feature = "media-file")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-file", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-file", feature = "media-full"))))]
pub use load_file::EFI_LOAD_FILE_PROTOCOL;
#[cfg(feature = "media-file")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-file", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-file", feature = "media-full"))))]
pub use load_file_2::EFI_LOAD_FILE2_PROTOCOL;
#[cfg(feature = "media-nvme")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-nvme", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-nvme", feature = "media-full"))))]
pub use nvme::EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL;
#[cfg(feature = "media-sd-mmc")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-sd-mmc", feature = "media-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "media-sd-mmc", feature = "media-full")))
)]
pub use sdmmc::EFI_SD_MMC_PASS_THRU_PROTOCOL;
#[cfg(feature = "media-fs")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-fs", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-fs", feature = "media-full"))))]
pub use simple_filesystem::EFI_SIMPLE_FILE_SYSTEM_PROTOCOL;
#[cfg(feature = "media-storesec")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-storesec", feature = "media-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "media-storesec", feature = "media-full")))
)]
pub use storesec::EFI_STORAGE_SECURITY_COMMAND_PROTOCOL;
#[cfg(feature = "media-tape")]
#[cfg_attr(doc, doc(cfg(any(feature = "media-tape", feature = "media-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "media-tape", feature = "media-full"))))]
pub use tape::EFI_TAPE_IO_PROTOCOL;
