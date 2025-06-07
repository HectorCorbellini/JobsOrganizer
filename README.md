# Rust Job Files Organizer

A command-line Rust application (containerized with Docker) that organizes job and project text files into categorized directories based on programming language, document quality, and nationality.

## Features

- Scans all files in a source directory (recursively).
- Classifies files by:
  - **Language**: detects `java` (case-insensitive) vs. `other`.
  - **Quality**: content length > 1000 chars ⇒ `good`, else `low`.
  - **Nationality**: checks for the substring `Uruguay` ⇒ `national`.
- Uses [sled](https://crates.io/crates/sled) (pure Rust) for an embedded key-value store; no native C dependencies.
- Preserves original directory structure under each category.
- Provides a Docker multi-stage build for reproducible results on minimal Linux hosts.

## Prerequisites

- **Docker** installed and running.
- (Optional) Rust toolchain for native builds.

## Project Structure

```
Organizer-/
├── Cargo.toml        # Rust package manifest
├── src/main.rs       # Application logic
├── Dockerfile        # Multi-stage Docker build
├── README.md         # This file
└── ADAPTATION.md     # Details on adapting to Puppy Linux Bookworm environment
```

## Building and Running

### Docker

1. Build the Docker image:
   ```bash
   docker build -t organizer .
   ```

2. Run the organizer, mounting source and destination:
   ```bash
   docker run --rm \
     -v /path/to/source:/data/src:ro \
     -v /path/to/output:/data/dest \
     organizer \
     --src /data/src \
     --dest /data/dest \
     --db organizer.db
   ```

- **`--src`**: root of your job/project files.
- **`--dest`**: directory where organized output will be written.
- **`--db`**: path to the sled database file (internal use).

### Native (if Rust toolchain available)

1. Install dependencies in `Cargo.toml` via `cargo`.
2. Build in release mode:
   ```bash
   cargo build --release
   ```
3. Run the binary:
   ```bash
   ./target/release/organizer --src /path/to/source --dest /path/to/output --db organizer.db
   ```

> **Note**: Native builds on minimal distros may fail due to missing `glibc-dev` or musl linking issues. Docker is recommended.

## Classification Output

Under the destination directory, files are organized:

```
<dest>/
├── Works in java language/
│   ├── National (uruguayan) works/...     # Uruguay + Java
│   ├── Good quality works/...             # Java + length>1000
│   └── Low quality works/...              # Java + length≤1000
└── Works in other languages/
    ├── National (uruguayan) works/...     # Uruguay + non-Java
    ├── Good quality works/...             # non-Java + length>1000
    └── Low quality works/...              # non-Java + length≤1000
```

Subdirectories mirror the original relative paths of each file.

## Customization

- Adjust the Java detection regex in `main.rs`.
- Tweak quality thresholds or keywords as needed.

## GitHub Repository Setup

To clone and push to this repository using SSH authentication:

1. **Generate an SSH key** (if you don't have one):
   ```bash
   ssh-keygen -t ed25519 -C "your_email@example.com"
   ```
   - Press Enter to accept the default file location
   - Optionally set a passphrase for added security

2. **Add your SSH key to the SSH agent**:
   ```bash
   eval "$(ssh-agent -s)"
   ssh-add ~/.ssh/id_ed25519
   ```

3. **Add the public key to GitHub**:
   - Copy your public key:
     ```bash
     cat ~/.ssh/id_ed25519.pub
     ```
   - Go to [GitHub SSH Keys](https://github.com/settings/keys)
   - Click "New SSH key"
   - Paste your public key and save

4. **Clone the repository**:
   ```bash
   git clone git@github.com:HectorCorbellini/JobsOrganizer.git
   ```

5. **Push changes**:
   ```bash
   git add .
   git commit -m "Your commit message"
   git push origin main
   ```

## License

MIT © Your Name
