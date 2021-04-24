# upver

Manager for [SemVer](https://semver.org/)

Support
- Cargo.toml (Cargo)
- package.json (npm, yarn)


## Installation
```
brew install --HEAD fuyutarow/tap/upver
```

clean uninstall
```
brew uninstall fuyutarow/tap/upver
brew untap fuyutarow/tap
```


## Usage
semver: X.Y.Z (Major.Minor.Patch)

- Increment Major version: X+1.y.z
  ```
  upver up -x Cargo.toml
  ```
- Increment Patch version: x.y.Z+1 and replace new file
  ```
  upver up -z -r Cargo.toml
  ```
- Set pre and build: x.y.z -> x.y.z-alpha+beta
  ```
  upver up --pre alpha --build beta Cargo.toml
  ```


## Development
```
cargo install cargo-make
cargo make hot
```
