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
//! This library is licensed under **Version 3 of the GNU Public License**.
//!
//! The full text of the license is available in the [`COPYING` file].
//!
//! [`COPYING` file]: ./COPYING
