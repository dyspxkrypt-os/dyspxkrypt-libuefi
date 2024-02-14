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

#[cfg(feature = "pci-rbio")]
#[cfg_attr(doc, doc(cfg(any(feature = "pci-rbio", feature = "pci-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "pci-rbio", feature = "pci-full"))))]
pub mod rbio;

#[cfg(feature = "pci-rbio")]
#[cfg_attr(doc, doc(cfg(any(feature = "pci-rbio", feature = "pci-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "pci-rbio", feature = "pci-full"))))]
pub use rbio::EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL;
