# Changelog
All notable changes to this Quantis Software Release project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [24.4.2] - 2024-05-10

### Changed
- Updated versions of all packages to latest version (see above).

### Added
- Code Quality: `release`
- Quantis Libs updated for compatibility up to 'Ubuntu 22.04 LTS' & 'AlmaLinux 9.3'
- Quantis Libs updated to QT5 Linux
- Quantis Libs are delivered in RPM, DEB and sources package
- Quantis Pcie driver supports of Linux kernel up to 6.1
- Quantis Pcie driver supports of Linux kernel 5.x
- Quantis Pcie driver are delivered in RPM, DEB and sources package

### Removed
- 32 bits is not build nor shipped anymore.


## [23.1.33-preview] - 2023-10-20

### Added
- Code Quality: `private preview`
- Quantis Pcie driver tarball source files for kernel module versioned.
- Quantis Pcie driver supports of Linux kernel 5.x
- Quantis Pcie driver are delivered in RPM, DEB and sources package

## [20.2.4]

### Added
- Microsoft-certified Windows driver.

### Changed
- Updated versions of all packages to 20.2.4 (01/April/2022).

## [20.2.3]

### Added
- Support for new generation pcie-chip.
- Initial version of Linux driver, Windows driver, and User Manual.

### Changed
- Updated versions of all packages to 20.2.3 (20/April/2020).
- Use same version (MAJOR.MINOR.PATCH) for all softwares.
- Removed sources related to non-supported OS.
- Updated UDEV file according to new OS rules.
- Added UDEV support for pcie-chip.
- Updated Quantis library Windows project to Visual Studio 2019.
- Added Quantis library support for the new pcie-chip.

## [08/March/2018]

### Changed
- Updated versions of all packages and documents (08/March/2018).
- Added references to the extractor algorithm in Documentation v3.1.

## [21/July/2017]

### Changed
- Updated versions of all packages and documents.
- Improved support for Linux kernel > 4 in Quantis-PCI Unix 2.9.
- Updated layout and added Windows 10 and Windows Server 2016 support in Documentation v3.0.

## [8/November/2013]

### Added
- Support for Windows 8.1 in Quantis-PCI Windows 5.1.
- Added Quantis-PCIe-16Mbit/s throughput issue fix in Quantis-PCI Windows 5.1.
- Added Quantis-PCIe-16Mbit/s throughput issue fix in Quantis-PCI Unix 2.7.

## [29/April/2013]

### Added
- Added extraction capability in console line mode in EasyQuantis v2.1.
- Added QuantisUsbGetModulesDataRate bug fix in Quantis Library v2.12.
- Added update of driver certificate due to expiration in Quantis-USB Windows 2.1 and Quantis-PCI Windows 5.0.
- Added Quantis-PCI Unix 2.6 with IOCTL GET_PCI_BUS_DEVICE_ID for IDQ internal purpose and fixed compilation errors with linux kernel >= 3.8.0.

## [12/September/2012]

### Added
- Added randomness extraction capabilities in EasyQuantis v2.0.
- Added QuantisExtensions library with randomness extraction capability in Quantis Library v2.10.
- Added support for Solaris and FreeBSD in Quantis Library v2.9.

## [12/July/2011]

### Fixed
- Fixed missing dll in the EasyQuantis Setup installation in EasyQuantis 1.4.
- Fixed error control with Quantis-USB devices in Quantis Library v2.8.

## [20/May/2011]

### Changed
- Corrected minor mistakes and rephrased a few sentences in Documentation v2.6.

### Added
- Added support for Quantis USB on Mac OSX in Quantis Library v2.7.

## [19/April/2011]

### Changed
- Updated EasyQuantis installation description on Linux in Documentation v2.6.

### Fixed
- Fixed compilation error on RedHat/CentOS distributions in Unix PCI Driver v2.3.

## [9/December/2011]

### Added
- Implemented the C++11 random_device interface to use Quantis in Quantis Library v2.9.
- Quantis can now be used on Solaris and FreeBSD in Quantis Library v2.9.

## [12/Jan/2011]

### Added
- Added details on the QuantisGetManufacturer method in Documentation v2.5.
- Display Manufacturer's name when displaying Quantis USB devices info in EasyQuantis v1.4.

## [08/Oct/2010]

### Fixed
- Fixed 'kobject_add failed for Quantis PCI/PCIe RNG driver (-13)' module crash on module load in Unix PCI Driver v2.4.

## [20/Sep/2010]

### Added
- Added instruction to install Quantis on Red Had Enterprise Linux and CentOS distributions in Documentation v2.4.
- Added FreeBSD support and Solaris support in Unix PCI Driver v2.3.
- Added missing signature certificate in Microsoft Windows USB Driver 2.1.

## [29/Jun/2010]

### Added
- Added paths for FreeBSD in FindJNI.cmake in Quantis Library v2.4.

## [28/Jun/2010]

### Changed
- Improved EasyQuantis installation procedure under Linux in Documentation v2.3.
- Displaying Manufacturer's name when displaying Quantis USB devices info in EasyQuantis v1.4.

## [27/May/2010]

### Fixed
- Fixed compilation on FreeBSD and Solaris in EasyQuantis v1.2 and Quantis Library v2.3.

## [25/May/2010]

### Added
- Added FreeBSD support and Solaris support in Unix PCI Driver v2.1.

## [30/Apr/2010]

### Added
- Added compiled version for 64-bit Linux systems in Quantis Library v2.2.
- Added EasyQuantis command line section in Documentation v2.2.

## [26/Apr/2010]

### Changed
- Updated EasyQuantis installation procedure under Linux in Documentation v2.1.

### Fixed
- Fixed wrong text message during number generation in EasyQuantis v1.1.

## [09/Apr/2010]

### Added
- Baseline for this changeLog.

