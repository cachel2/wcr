This is a minimal and super tiny line counter in rust.

build with
```bash
git clone https://github.com/cachel2/wcr.git
cd wcr 
cargo build -r
```

The compiled binary will be available at `./target/release/wcr`.

## Usage

### 1. Count lines in a single file
```bash
./target/release/wcr src/main.rs
```

### 2. Count lines in multiple files
```bash
./target/release/wcr src/main.rs Cargo.toml
```

### 3. Read from standard input (Pipes)
```bash
cat Cargo.toml | ./target/release/wcr
```
