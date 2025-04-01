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

pub mod simple_text_input;
pub mod simple_text_output;

pub use simple_text_input::EFI_SIMPLE_TEXT_INPUT_PROTOCOL;
pub use simple_text_output::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;

#[cfg(any(feature = "console-pointer", feature = "console-full"))]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "console-pointer", feature = "console-full")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "console-pointer", feature = "console-full")))
)]
pub mod absolute_pointer;
#[cfg(any(feature = "console-gop", feature = "console-full"))]
#[cfg_attr(doc, doc(cfg(any(feature = "console-gop", feature = "console-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "console-gop", feature = "console-full")))
)]
pub mod graphics_output;
#[cfg(any(feature = "console-serial", feature = "console-full"))]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "console-serial", feature = "console-full")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "console-serial", feature = "console-full")))
)]
pub mod serial_io;
#[cfg(any(feature = "console-pointer", feature = "console-full"))]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "console-pointer", feature = "console-full")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "console-pointer", feature = "console-full")))
)]
pub mod simple_pointer;
#[cfg(any(feature = "console", feature = "console-full"))]
#[cfg_attr(doc, doc(cfg(any(feature = "console", feature = "console-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "console", feature = "console-full"))))]
pub mod simple_text_input_ex;

#[cfg(any(feature = "console-pointer", feature = "console-full"))]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "console-pointer", feature = "console-full")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "console-pointer", feature = "console-full")))
)]
pub use absolute_pointer::EFI_ABSOLUTE_POINTER_PROTOCOL;
#[cfg(any(feature = "console-gop", feature = "console-full"))]
#[cfg_attr(doc, doc(cfg(any(feature = "console-gop", feature = "console-full"))))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "console-gop", feature = "console-full")))
)]
pub use graphics_output::EFI_GRAPHICS_OUTPUT_PROTOCOL;
#[cfg(any(feature = "console-serial", feature = "console-full"))]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "console-serial", feature = "console-full")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "console-serial", feature = "console-full")))
)]
pub use serial_io::EFI_SERIAL_IO_PROTOCOL;
#[cfg(any(feature = "console-pointer", feature = "console-full"))]
#[cfg_attr(
    doc,
    doc(cfg(any(feature = "console-pointer", feature = "console-full")))
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "console-pointer", feature = "console-full")))
)]
pub use simple_pointer::EFI_SIMPLE_POINTER_PROTOCOL;
#[cfg(any(feature = "console", feature = "console-full"))]
#[cfg_attr(doc, doc(cfg(any(feature = "console", feature = "console-full"))))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "console", feature = "console-full"))))]
pub use simple_text_input_ex::EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL;
