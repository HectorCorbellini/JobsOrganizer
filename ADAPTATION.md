# Adaptation for Puppy Linux Bookworm

This document explains every file change and workflow to get our Rust organizer running on Puppy Linux (Debian Bookworm).

## 1. Cargo.toml

- **Initial**: included `rusqlite` for SQLite storage.
- **Edited**: removed `rusqlite` dependency; added pure-Rust store:
  ```toml
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  sled = "0.34"
  ```
  This eliminates C/`glibc` dependencies that failed on Bookworm.

## 2. src/main.rs

- **Imports**:
  - Removed `rusqlite::{params, Connection}` and any SQLite code.
  - Added `use sled; use serde::{Serialize,Deserialize}`.
- **Work struct**:
  ```rust
  #[derive(Serialize, Deserialize)]
  struct Work { file: String, language: String, quality: String, national: bool }
  ```
- **Main logic**:
  - Open Sled DB: `let db = sled::open(&args.db)?;`
  - On scan: serialize `Work` to JSON, `db.insert(path, vec)`.
  - After flush: iterate `db.iter()`, deserialize, then copy files.
- **Filename fix**: converted `file_name()` to owned `String` to satisfy Rust lifetimes.

## 3. Dockerfile

A multi-stage build isolates all host linker issues:

```dockerfile
# --- builder stage ---
FROM rust:1.70 as builder
WORKDIR /usr/src/organizer
COPY . .
RUN rm -f Cargo.lock      # avoid lockfile version mismatch
RUN cargo build --release

# --- runtime stage ---
FROM debian:bookworm-slim
WORKDIR /usr/src/organizer
COPY --from=builder /usr/src/organizer/target/release/organizer ./organizer
ENTRYPOINT ["/usr/src/organizer/organizer"]
```

- **Key points**:
  1. Uses official Rust image so `musl-gcc` and required libs are preconfigured.
  2. Removes `Cargo.lock` to match the builder’s Cargo version.
  3. Final image is minimal Bookworm without Rust toolchain.

## 4. Host Build Attempts (Context)

- Native `cargo build` → missing CRT files (`Scrt1.o`, `crti.o`) because Puppy Linux’s `libc6-dev` was held at an older patch level.
- Musl static target (`x86_64-unknown-linux-musl`) → still missing `libgcc_s.so.1`, `-lutil`, etc., even after symlinking from `/usr/lib/x86_64-linux-musl`.
- Conclusion: Docker is the simplest, most reproducible solution on this distro.

## 5. Usage Summary

1. **Build image**:
   ```bash
   docker build -t organizer .
   ```
2. **Run organizer**:
   ```bash
   docker run --rm \
     -v /root/EMPREND/TRABAJOS-mayo25:/data/src:ro \
     -v /root/EMPREND/Rust-Organizer/Works-COPY:/data/dest \
     organizer \
     --src /data/src \
     --dest /data/dest \
     --db organizer.db
   ```

All file classification and copying now runs reliably inside Docker on Puppy Linux Bookworm.
