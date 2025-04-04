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

//! # Dyspxkrypt LibUEFI
//!
//! ![Crates.io Version](https://img.shields.io/crates/v/dyspxkrupt-libuefi?style=for-the-badge)
//! ![docs.rs](https://img.shields.io/docsrs/dyspxkrupt-libuefi?style=for-the-badge)
//! ![GitHub Repo stars](https://img.shields.io/github/stars/dyspxkrypt-os/dyspxkrypt-libuefi?style=for-the-badge)
//! ![GitHub License](https://img.shields.io/github/license/dyspxkrypt-os/dyspxkrypt-libuefi?style=for-the-badge)
//!
//! ## Introduction
//!
//! [Unified Extensible Firmware Interface (UEFI)] is the successor to [Basic Input Output System (BIOS)], the specification
//! that defines the architecture of the platform firmware used for booting computing hardware and its interface for interactions
//! with the operating system.
//!
//! [Basic Input Output System (BIOS)]: https://en.wikipedia.org/wiki/BIOS
//! [Unified Extensible Firmware Interface (UEFI)]: https://en.wikipedia.org/wiki/UEFI
//!
//! This crate aims to provide an API that is as low-level as possible and as accurate as possible with accordance to the UEFI
//! specification. An absolute minimal amount of wrappers around the types will be provided when deemed necessary.
//!
//! The objective is to provide a bare-metal API empowering developers to have fine control over every aspect of whatever application
//! they implement leveraging this library, including what to include from this library as well.
//!
//! ## License
//!
//! This library is licensed under **Version 3 of the GNU General Public License**.
//!
//! The full text of the license is available in the [`COPYING` file].
//!
//! [`COPYING` file]: ./COPYING

#![no_std]
#![allow(clippy::missing_safety_doc)]
#![allow(internal_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![cfg_attr(doc, feature(doc_cfg))]
#![feature(extended_varargs_abi_support)]
#![feature(prelude_import)]

extern crate alloc;

pub mod prelude;

#[cfg(feature = "partition")]
#[cfg_attr(doc, doc(cfg(feature = "partition")))]
#[cfg_attr(docsrs, doc(cfg(feature = "partition")))]
pub mod partition;
pub mod protocols;
pub mod tables;
pub mod types;

#[allow(unused_imports)]
#[prelude_import]
pub(crate) use prelude::*;
