# Changelog

## [Unreleased]
### Added
- Implemented initial native UI (`egui`) under the `ui` feature flag:
  - Two-panel layout: selectable job list and a detail view.
  - Interactive "Mark as Applied" checkbox that persists to storage.
  - Top menu bar with "File" -> "Quit".
  - "About" dialog window accessible from "File" -> "About".
- Added `reduce_size` script to clean build artifacts and reduce project size.

### Changed
- Refactored UI state management for clarity and correctness.

### Fixed
- Corrected a regular expression in `processor::classification` to properly handle keywords with punctuation (e.g., "C++"), fixing a failing test.
- Resolved a UI compilation error caused by a type mismatch when comparing job IDs.
- Cleaned up all `cargo clippy` warnings and applied `cargo fmt` formatting.

- Introduced a modular scraping infrastructure under `src/scraper/`:
  - Core traits and types for job scraping (`core.rs`)
  - Platform-specific scraper modules (LinkedIn, Indeed, StackOverflow stubs)
  - Storage abstraction using SQLite for scalable job data management
  - Basic filter module stubs (`date_filter.rs`, `keyword_filter.rs`, `remote_filter.rs`)
- Implemented a basic LinkedIn job scraper (`platforms/linkedin.rs`) with async support
- Scaffolded `IndeedScraper` and `StackOverflowScraper` stubs.
- Integrated job scraping into the main workflow, allowing jobs to be fetched from multiple platforms and stored in SQLite.
- Added `orchestrator.rs` to manage the pipeline (scraping, processing, output generation).
- Added `scheduler.rs` stub for periodic pipeline execution.
- Introduced `src/output/reports.rs` with Tera templating for `ALL_JOBS.md` generation.
- Added Tera as a dependency.
- Added new dependencies for HTTP requests, async runtime, SQLite, and error handling
- Updated Dockerfile to support new dependencies and runtime requirements

### Fixed
- Dropped initial Sled storage handle before report generation in orchestrator to prevent DB lock conflicts.
- Fixed CLI override mode to auto-create source, destination, and GOING-ON directories in `Config::from_args`, avoiding missing-directory errors.

### Changed
- Refactored `main.rs` to delegate pipeline execution to `orchestrator.rs`.
- Modified `update_all_jobs` in `main.rs` to use templating from `output::reports.rs` and fetch data from SQLite.
- Maintained compatibility with the previous file organization and classification logic
- Optimized `Dockerfile` for faster subsequent builds by improving layer caching for dependencies.
- Added `cargo fix` step in Dockerfile to auto-remove unused import warnings before the final build.

### Architecture
- Established a clear separation of concerns:
  - Scraping (data acquisition)
  - Storage (database management)
  - Processing (classification and file organization)
- Laid the foundation for future platform integrations and advanced filtering

### Notes
- The new system is designed to be extensible for additional job platforms and more advanced classification/validation in future phases.

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
