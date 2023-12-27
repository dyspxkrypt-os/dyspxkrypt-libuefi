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

pub mod simple_text_input;
pub mod simple_text_output;

pub use simple_text_input::EFI_SIMPLE_TEXT_INPUT_PROTOCOL;
pub use simple_text_output::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;

cfg_if::cfg_if! {
    if #[cfg(feature = "console-full")] {
        pub mod simple_text_input_ex;
        pub mod simple_pointer;

        pub use simple_text_input_ex::EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL;
        pub use simple_pointer::EFI_SIMPLE_POINTER_PROTOCOL;
    } else {
    }
}
