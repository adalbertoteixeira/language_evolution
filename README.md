# Language Evolution

Script to monitor JavaScript to TypeScript migration.

## Release binaries

To upload latest build binaries to Github:

```
docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:latest cargo build --release --target aarch64-unknown-linux-gnu
gh release upload v0.2.0 target/aarch64-unknown-linux-gnu/release/language_evolution --clobber
```

## Install
```
cd ~ && \
apt-get update && apt-get install ripgrep && \
curl -LO https://github.com/adalbertoteixeira/language_evolution/releases/download/v0.2.0/language_evolution && \
chmod +x ./language_evolution
```

## Usage

```

export FOLDERS_TO_MATCH
./language_evolution -f "web,api" -p "/tmp/ben"
```
