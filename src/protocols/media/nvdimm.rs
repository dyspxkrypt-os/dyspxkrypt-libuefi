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

pub const EFI_NVDIMM_LABEL_PROTOCOL_GUID: EFI_GUID = unsafe {
    EFI_GUID::from_raw_parts(
        0xD40B6B80,
        0x97D5,
        0x4282,
        [0xBB, 0x1D, 0x22, 0x3A, 0x16, 0x91, 0x80, 0x58],
    )
};

#[repr(C)]
pub struct EFI_NVDIMM_LABEL_PROTOCOL {
    pub LabelStorageInformation: unsafe extern "efiapi" fn(
        This: *mut EFI_NVDIMM_LABEL_PROTOCOL,
        SizeOfLabelStorageArea: *mut UINT32,
        MaxTransferLength: *mut UINT32,
    ) -> EFI_STATUS,
    pub LabelStorageRead: unsafe extern "efiapi" fn(
        This: *const EFI_NVDIMM_LABEL_PROTOCOL,
        Offset: UINT32,
        TransferLength: UINT32,
        LabelData: *mut UINT8,
    ) -> EFI_STATUS,
    pub LabelStorageWrite: unsafe extern "efiapi" fn(
        This: *const EFI_NVDIMM_LABEL_PROTOCOL,
        Offset: UINT32,
        TransferLength: UINT32,
        LabelData: *mut UINT8,
    ) -> EFI_STATUS,
}
