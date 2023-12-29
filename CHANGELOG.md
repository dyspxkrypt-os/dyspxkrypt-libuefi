# Dyspxkrypt LibUEFI Unreleased

## Added

## Changed

## Fixed

## Removed

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
