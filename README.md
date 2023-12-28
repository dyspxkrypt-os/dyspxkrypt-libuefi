# Dyspxkrypt LibUEFI

[![Crates.io Version](https://img.shields.io/crates/v/dyspxkrypt-libuefi?style=for-the-badge)](https://crates.io/crates/dyspxkrypt-libuefi)
[![docs.rs](https://img.shields.io/docsrs/dyspxkrypt-libuefi?style=for-the-badge)](https://docs.rs/dyspxkrypt-libuefi)
![GitHub Repo stars](https://img.shields.io/github/stars/dyspxkrypt-os/dyspxkrypt-libuefi?style=for-the-badge)
![GitHub License](https://img.shields.io/github/license/dyspxkrypt-os/dyspxkrypt-libuefi?style=for-the-badge)

## Introduction

[Unified Extensible Firmware Interface (UEFI)] is the successor to [Basic Input Output System (BIOS)], the specification
that defines the architecture of the platform firmware used for booting computing hardware and its interface for interactions
with the operating system.

[Basic Input Output System (BIOS)]: https://en.wikipedia.org/wiki/BIOS
[Unified Extensible Firmware Interface (UEFI)]: https://en.wikipedia.org/wiki/UEFI

This crate aims to provide an API that is as low-level as possible and as accurate as possible with accordance to the UEFI
specification. An absolute minimal amount of wrappers around the types will be provided when deemed necessary.

The objective is to provide a bare-metal API empowering developers to have fine control over every aspect of whatever application
they implement leveraging this library, including what to include from this library as well.

## License

The code in this repository is licensed under **Version 3 of the GNU General Public License**.

The full text of the license is available in the [`COPYING` file].

[`COPYING` file]: ./COPYING
