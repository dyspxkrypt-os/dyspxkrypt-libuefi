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

#[cfg(feature = "usb-controller")]
#[cfg_attr(doc, doc(cfg(any(feature = "usb-controller", feature = "usb-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "usb-controller", feature = "usb-full")))
)]
pub mod controller;
#[cfg(feature = "usb-function")]
#[cfg_attr(doc, doc(cfg(any(feature = "usb-function", feature = "usb-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "usb-function", feature = "usb-full"))))]
pub mod function;
#[cfg(feature = "usb-io")]
#[cfg_attr(doc, doc(cfg(any(feature = "usb-io", feature = "usb-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "usb-io", feature = "usb-full"))))]
pub mod io;

#[cfg(feature = "usb-controller")]
#[cfg_attr(doc, doc(cfg(any(feature = "usb-controller", feature = "usb-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "usb-controller", feature = "usb-full")))
)]
pub use controller::EFI_USB2_HC_PROTOCOL;
#[cfg(feature = "usb-function")]
#[cfg_attr(doc, doc(cfg(any(feature = "usb-function", feature = "usb-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "usb-function", feature = "usb-full"))))]
pub use function::EFI_USBFN_IO_PROTOCOL;
#[cfg(feature = "usb-io")]
#[cfg_attr(doc, doc(cfg(any(feature = "usb-io", feature = "usb-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "usb-io", feature = "usb-full"))))]
pub use io::EFI_USB_IO_PROTOCOL;
