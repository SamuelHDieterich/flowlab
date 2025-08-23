{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain =
          pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        mingw = pkgs.pkgsCross.mingwW64;
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      in with pkgs; {

        # nix develop
        devShells.default = mkShell {
          buildInputs = [
            pkg-config
            rustToolchain
            # Cross-compilation
            ## cargo build --target x86_64-pc-windows-gnu
            mingw.buildPackages.gcc
            mingw.buildPackages.stdenv.cc
          ];
          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS =
            "-L native=${mingw.windows.pthreads}/lib";
        };

        # nix build
        packages.default = (makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        }).buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkg-config ];
        };

      });
}
