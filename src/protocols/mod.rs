/*
 * Dyspxkrypt LibUEFI: Raw bindings of UEFI that conforms to the definitions of the UEFI Specification.
 * Copyright (C) 2023-2025 HTGAzureX1212.
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

#[cfg(feature = "acpi")]
#[cfg_attr(doc, doc(cfg(feature = "acpi")))]
#[cfg_attr(docsrs, doc(cfg(feature = "acpi")))]
pub mod acpi;
#[cfg(feature = "compress")]
#[cfg_attr(doc, doc(cfg(feature = "compress")))]
#[cfg_attr(docsrs, doc(cfg(feature = "compress")))]
pub mod compress;
pub mod console;
#[cfg(any(
    feature = "debug-debug",
    feature = "debug-port",
    feature = "debug-full"
))]
#[cfg_attr(
    doc,
    doc(cfg(any(
        feature = "debug-debug",
        feature = "debug-port",
        feature = "debug-full"
    )))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "debug-debug",
        feature = "debug-port",
        feature = "debug-full"
    )))
)]
pub mod debug;
pub mod device_path;
#[cfg(any(
    feature = "driver-adapter",
    feature = "driver-binding",
    feature = "driver-busspec",
    feature = "driver-comp",
    feature = "driver-conf",
    feature = "driver-adapter",
    feature = "driver-diagnostic",
    feature = "driver-family-override",
    feature = "driver-full",
    feature = "driver-health",
    feature = "driver-override",
    feature = "driver-supported-efi"
))]
#[cfg_attr(
    doc,
    doc(cfg(any(
        feature = "driver-adapter",
        feature = "driver-binding",
        feature = "driver-busspec",
        feature = "driver-comp",
        feature = "driver-conf",
        feature = "driver-adapter",
        feature = "driver-diagnostic",
        feature = "driver-family-override",
        feature = "driver-full",
        feature = "driver-health",
        feature = "driver-override",
        feature = "driver-supported-efi"
    )))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "driver-adapter",
        feature = "driver-binding",
        feature = "driver-busspec",
        feature = "driver-comp",
        feature = "driver-conf",
        feature = "driver-adapter",
        feature = "driver-diagnostic",
        feature = "driver-family-override",
        feature = "driver-full",
        feature = "driver-health",
        feature = "driver-override",
        feature = "driver-supported-efi"
    )))
)]
pub mod driver;
#[cfg(feature = "image")]
#[cfg_attr(doc, doc(cfg(any(feature = "image", feature = "image-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "image", feature = "image-full"))))]
pub mod image;
#[cfg(feature = "iscsi")]
#[cfg_attr(doc, doc(cfg(feature = "iscsi")))]
#[cfg_attr(docsrs, doc(cfg(feature = "iscsi")))]
pub mod iscsi;
#[cfg(any(
    feature = "media-ata",
    feature = "media-block",
    feature = "media-block-2",
    feature = "media-block-v2",
    feature = "media-block-v3",
    feature = "media-crypt",
    feature = "media-crypt-aes-cbc",
    feature = "media-crypt-aes-xts",
    feature = "media-disk",
    feature = "media-disk-2",
    feature = "media-erase",
    feature = "media-file",
    feature = "media-file-v2",
    feature = "media-fs",
    feature = "media-nvdimm",
    feature = "media-nvme",
    feature = "media-partition",
    feature = "media-ramdisk",
    feature = "media-sd-mmc",
    feature = "media-storesec",
    feature = "media-tape",
    feature = "media-ufs",
    feature = "media-full"
))]
#[cfg_attr(
    doc,
    doc(cfg(any(
        feature = "media-ata",
        feature = "media-block",
        feature = "media-block-2",
        feature = "media-block-v2",
        feature = "media-block-v3",
        feature = "media-crypt",
        feature = "media-crypt-aes-cbc",
        feature = "media-crypt-aes-xts",
        feature = "media-disk",
        feature = "media-disk-2",
        feature = "media-erase",
        feature = "media-file",
        feature = "media-file-v2",
        feature = "media-fs",
        feature = "media-nvdimm",
        feature = "media-nvme",
        feature = "media-partition",
        feature = "media-ramdisk",
        feature = "media-sd-mmc",
        feature = "media-storesec",
        feature = "media-tape",
        feature = "media-ufs",
        feature = "media-full"
    )))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "media-ata",
        feature = "media-block",
        feature = "media-block-2",
        feature = "media-block-v2",
        feature = "media-block-v3",
        feature = "media-crypt",
        feature = "media-crypt-aes-cbc",
        feature = "media-crypt-aes-xts",
        feature = "media-disk",
        feature = "media-disk-2",
        feature = "media-erase",
        feature = "media-file",
        feature = "media-file-v2",
        feature = "media-fs",
        feature = "media-nvdimm",
        feature = "media-nvme",
        feature = "media-partition",
        feature = "media-ramdisk",
        feature = "media-sd-mmc",
        feature = "media-storesec",
        feature = "media-tape",
        feature = "media-ufs",
        feature = "media-full"
    )))
)]
pub mod media;
#[cfg(any(
    feature = "network-boot-integ",
    feature = "network-http-callback",
    feature = "network-interface-id",
    feature = "network-simple",
    feature = "network-pxe",
    feature = "network-pxe-callback",
    feature = "network-full"
))]
#[cfg_attr(
    doc,
    doc(cfg(any(
        feature = "network-boot-integ",
        feature = "network-http-callback",
        feature = "network-interface-id",
        feature = "network-simple",
        feature = "network-pxe",
        feature = "network-pxe-callback",
        feature = "network-full"
    )))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "network-boot-integ",
        feature = "network-http-callback",
        feature = "network-interface-id",
        feature = "network-simple",
        feature = "network-pxe",
        feature = "network-pxe-callback",
        feature = "network-full"
    )))
)]
pub mod network;
#[cfg(any(feature = "pci-full", feature = "pci-io", feature = "pci-rbio"))]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "pci-full", feature = "pci-io", feature = "pci-rbio")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "pci-full", feature = "pci-io", feature = "pci-rbio")))
)]
pub mod pci;
#[cfg(any(feature = "scsi-full", feature = "scsi-io", feature = "scsi-passthru"))]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "scsi-full", feature = "scsi-io", feature = "scsi-passthru")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "scsi-full", feature = "scsi-io", feature = "scsi-passthru")))
)]
pub mod scsi;
#[cfg(any(
    feature = "string-full",
    feature = "string-collation",
    feature = "string-regex"
))]
#[cfg_attr(
    doc,
    doc(cfg(any(
        feature = "string-full",
        feature = "string-collation",
        feature = "string-regex"
    )))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "string-full",
        feature = "string-collation",
        feature = "string-regex"
    )))
)]
pub mod string;
#[cfg(any(
    feature = "usb-full",
    feature = "usb-controller",
    feature = "usb-function",
    feature = "usb-io"
))]
#[cfg_attr(
    doc,
    doc(cfg(any(
        feature = "usb-full",
        feature = "usb-controller",
        feature = "usb-function",
        feature = "usb-io"
    )))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "usb-full",
        feature = "usb-controller",
        feature = "usb-function",
        feature = "usb-io"
    )))
)]
pub mod usb;
