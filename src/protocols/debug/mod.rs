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

#[cfg(feature = "debug-debug")]
#[cfg_attr(doc, doc(cfg(any(feature = "debug-debug", feature = "debug-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "debug-debug", feature = "debug-full"))))]
pub mod debug;
#[cfg(feature = "debug-port")]
#[cfg_attr(doc, doc(cfg(any(feature = "debug-port", feature = "debug-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "debug-port", feature = "debug-full"))))]
pub mod port;

#[cfg(feature = "debug-debug")]
#[cfg_attr(doc, doc(cfg(any(feature = "debug-debug", feature = "debug-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "debug-debug", feature = "debug-full"))))]
pub use debug::EFI_DEBUG_SUPPORT_PROTOCOL;
#[cfg(feature = "debug-port")]
#[cfg_attr(doc, doc(cfg(any(feature = "debug-port", feature = "debug-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "debug-port", feature = "debug-full"))))]
pub use port::EFI_DEBUGPORT_PROTOCOL;
