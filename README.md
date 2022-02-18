# Language Evolution

Script to monitor JavaScript to TypeScript migration.

## Release binaries

To build the binaries:

```
docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:latest cargo build --release --target x86_64-unknown-linux-gnu

# or 

docker run -it --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:latest /bin/bash

rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

If build fails , remove `target/*` before building.

To upload latest build binaries to Github:

```
gh release upload v0.2.0 target/x86_64-unknown-linux-gnu/release/language_evolution --clobber
```

## Limitations
Only one release version _OR_ date are allowed.

## Requirements
- sed / gnu-sed on MacOs
- [xsv](https://github.com/BurntSushi/xsv)

## Install
```
cd ~ && \
apt-get update && apt-get install ripgrep && \
curl -LO https://github.com/adalbertoteixeira/language_evolution/releases/download/v0.2.0/language_evolution && \
chmod +x ./language_evolution
```

## Usage

```
./language_evolution -h 
```

*Basic example with release version*
```
export FOLDERS_TO_MATCH
./language_evolution -f "web,api" -p "/tmp/ben"
```
