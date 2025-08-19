{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix.url = "github:nix-community/fenix";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        toolchain =
          with fenix.packages.${system};
          combine [
            minimal.rustc
            minimal.cargo
            targets.x86_64-pc-windows-gnu.latest.rust-std
          ];

        naersk' = naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        name = cargoToml.package.name;
        version = cargoToml.package.version;
      in
      rec {
        packages.default = packages.windows;

        packages.linux = pkgs.rustPlatform.buildRustPackage {
          inherit name version;

          cargoLock.lockFile = ./Cargo.lock;
          src = self;
        };

        packages.windows = naersk'.buildPackage {
          src = self;
          strictDeps = true;
          doCheck = false;

          depsBuildBuild = with pkgs; [
            pkgsCross.mingwW64.stdenv.cc
            pkgsCross.mingwW64.windows.pthreads
          ];

          CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            git
            rustc
            rustfmt
            cargo
            rust-analyzer
          ];
        };
      }
    );
}
