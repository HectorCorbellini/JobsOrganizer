# Changelog

## [1.1.0] - 2025-06-06

### Fixed
- Resolved build dependency issues by updating to Rust 1.82 in Dockerfile
- Fixed Docker build process by removing problematic vendoring steps
- Addressed file locking issues with the Sled database by using temporary directory

### Changed
- Simplified Dockerfile for better maintainability and reliability
- Updated README.md with corrected build and run instructions
- Improved error handling for file operations

### Added
- New CHANGELOG.md to track project changes
- Better logging for debugging file processing
- Support for more robust file organization based on content analysis

## [1.0.0] - 2025-05-25

### Initial Release
- Basic file organization functionality
- Support for Java and non-Java file categorization
- Quality-based file classification
- National (Uruguayan) content detection

[1.1.0]: https://github.com/HectorCorbellini/JobsOrganizer/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/HectorCorbellini/JobsOrganizer/releases/tag/v1.0.0
