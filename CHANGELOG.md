# Dyspxkrypt LibUEFI Unreleased

## Added

## Changed

## Fixed

## Removed

# Dyspxkrypt LibUEFI 0.4.0 (12-02-2024)

## Added

- custom prelude
- EFI NVME pass thru protocol and its associating constants and structures (`EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL`) (behind `media-nvme` or `media-full` feature flags)
- EFI SD MMC pass thru protocol and its associating constants and structures (`EFI_SD_MMC_PASS_THRU_PROTOCOL`) (behind `media-sd-mmc` or `media-full` feature flags)
- EFI ramdisk protocol and its associating constants and structures (`EFI_RAM_DISK_PROTOCOL`) (behind `media-ramdisk` or `media-full` feature flags)
- EFI partition info protocol and its associating constants and structures (`EFI_PARTITION_INFO_PROTOCOL`) (behind `media-partition` or `media-full` feature flags)
- EFI NVDIMM protocol and its associating constants and structures (`EFI_NVDIMM_LABEL_PROTOCOL`) (behind `media-nvdimm` or `media-full` feature flags)
- EFI UFS device config protocol and its associating constants and structures (`EFI_UFS_DEVICE_CONFIG_PROTOCOL`) (behind `media-ufs` or `media-full` feature flags)
- EFI partition infos (behind `partition` feature flag)

# Dyspxkrypt LibUEFI 0.3.1 (09-02-2024)

## Added

- EFI block I/O 2 protocol and its associating constants and structures (`EFI_BLOCK_IO2_PROTOCOL`) (behind `media-block-2` or `media-full` feature flags)
- EFI inline cryptographic interface protocol and its associating constants and structures (`EFI_BLOCK_IO_CRYPTO_PROTOCOL`) (behind `media-crypt` or `media-full` feature flags)
- EFI erase blocks protocol and its associating constants and structures (`EFI_ERASE_BLOCKS_PROTOCOL`) (behind `media-erase` or `media-full` feature flags)
- EFI ATA pass thru protocol and its associating constants and structures (`EFI_ATA_PASS_THRU_PROTOCOL`) (behind `media-ata` or `media-full` feature flags)
- EFI storage security command protocol and its associating constants and structures (`EFI_STORAGE_SECURITY_COMMAND_PROTOCOL`) (behind `media-storesec` or `media-full` feature flags)

# Dyspxkrypt LibUEFI 0.3.0 (09-02-2024)

## Added

- EFI tape I/O protocol and its associating constants and structures (`EFI_TAPE_IO_PROTOCOL`) (behind `media-tape` or `media-full` feature flags)
- EFI disk I/O protocol and its associating constants and structures (`EFI_DISK_IO_PROTOCOL`) (behind `media-disk` or `media-full` feature flags)
- EFI disk I/O v2 protocol and its associating constants and structures (`EFI_DISK_IO_2_PROTOCOL`) (behind `media-disk-v2` or `media-full` feature flags)
- EFI block I/O protocol and its associating constants and structures (`EFI_BLOCK_IO_PROTOCOL`) (behind `media-block` or `media-full` feature flags)

# Dyspxkrypt LibUEFI 0.2.0 (29-12-2023)

Second release.

## Added

- EFI loaded image protocol and its associating constants and structures (`EFI_LOADED_IMAGE_PROTOCOL`) (behind `image` or `image-full` feature flags)
- EFI loaded image device path protocol and its associating constants and structures (`EFI_LOADED_IMAGE_DEVICE_PATH_PROTOCOL`) (behind `image-full` feature flag)
- EFI absolute pointer protocol and its associating constants and structures (`EFI_ABSOLUTE_POINTER_PROTOCOL`) (behind `console-pointer` or `console-full` feature flags)
- EFI graphics output protocol and its associating constants and structures (`EFI_GRAPHICS_OUTPUT_PROTOCOL`) (behind `console-gop` or `console-full` feature flags)
- EFI serial I/O protocol and its associating constants and structures (`EFI_SERIAL_IO_PROTOCOL`) (behind `console-serial` or `console-full` feature flags)
- EFI simple pointer protocol and its associating constants and structures (`EFI_SIMPLE_POINTER_PROTOCOL`) (behind `console-pointer` or `console-full` feature flags)
- EFI simple text input ex protocol and its associating constants and structures (`EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`)  (behind `console-full` or `console-full` feature flags)
- EFI device path utilities protocol and its associating constants and structures (`EFI_DEVICE_PATH_UTILITIES_PROTOCOL`)  (behind `dev-path-full` feature flag)
- EFI file protocol and its associating constants and structures (`EFI_FILE_PROTOCOL`)  (behind `media-file`, `media-file-v2` or `media-full` feature flags)
- EFI load file protocol and its associating constants and structures (`EFI_LOAD_FILE_PROTOCOL`)  (behind `media-file`, `media-file-v2` or `media-full` feature flags)
- EFI load file 2 protocol and its associating constants and structures (`EFI_LOAD_FILE2_PROTOCOL`)  (behind `media-file`, `media-file-v2` or `media-full` feature flags)
- EFI simple filesystem protocol and its associating constants and structures (`EFI_SIMPLE_FILE_SYSTEM_PROTOCOL`)  (behind `media-fs` or `media-full` feature flags)

# Dyspxkrypt LibUEFI 0.1.0 (27-12-2023)

Initial release.

## Added

- fundamental data types (e.g. `BOOLEAN`, `INTN`, `UINTN`, etc.)
- EFI table header data structure (`EFI_TABLE_HEADER`)
- EFI system table and its associating constants (`EFI_SYSTEM_TABLE`)
- EFI simple text input protocol and its associating constants and structures (`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`)
- EFI simple text output protocol and its associating constants and structures (`EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`)
- EFI device path protocol and its associating constants and structures (`EFI_DEVICE_PATH_PROTOCOL`)
- EFI configuration table and its associating constants a structures (`EFI_CONFIGURATION_TABLE`)
- EFI boot services table and its associating constants a structures (`EFI_BOOT_SERVICES`)
- EFI runtime services table and its associating constants a structures (`EFI_RUNTIME_SERVICES`)
