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

#[cfg(feature = "network-interface-id")]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "network-interface-id", feature = "network-full")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "network-interface-id", feature = "network-full")))
)]
pub mod interface_id;
#[cfg(feature = "network-simple")]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "network-simple", feature = "network-full")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "network-simple", feature = "network-full")))
)]
pub mod simple;

#[cfg(feature = "network-interface-id")]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "network-interface-id", feature = "network-full")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "network-interface-id", feature = "network-full")))
)]
pub use interface_id::EFI_NETWORK_INTERFACE_IDENTIFIER_PROTOCOL;
#[cfg(feature = "network-simple")]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "network-simple", feature = "network-full")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "network-simple", feature = "network-full")))
)]
pub use simple::EFI_SIMPLE_NETWORK_PROTOCOL;
