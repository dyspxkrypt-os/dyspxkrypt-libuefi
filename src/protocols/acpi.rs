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

pub const EFI_ACPI_TABLE_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xFFE06BDD,
        0x6107,
        0x46A6,
        [0x7B, 0xB2, 0x5A, 0x9C, 0x7E, 0xC5, 0x27, 0x5C],
    )
};

#[repr(C)]
pub struct EFI_ACPI_TABLE_PROTOCOL {
    pub InstallAcpiTable: unsafe extern "efiapi" fn(
        This: *mut EFI_ACPI_TABLE_PROTOCOL,
        AcpiTableBuffer: *mut VOID,
        AcpiTableBufferSize: UINTN,
        TableKey: *mut UINTN,
    ) -> EFI_STATUS,
    pub UninstallAcpiTable: unsafe extern "efiapi" fn(
        This: *mut EFI_ACPI_TABLE_PROTOCOL,
        TableKey: UINTN,
    ) -> EFI_STATUS,
}
