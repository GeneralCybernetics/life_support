{
  description = "LifeSupport devShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, fenix, flake-utils, ... }: {
    defaultPackage = flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (fenix.overlay) ];
        };
        toolchain = pkgs.fromToolchainFile ./rust-toolchain.toml;
      in
      pkgs.mkShell {
        nativeBuildInputs = [
          toolchain
          pkgs.probe-rs
          pkgs.rust-analyzer
          pkgs.gdb-multiarch 
        ];

        shellHook = ''
          export MEMORY_X_PATH=${pkgs.stdenvNoCC.mkDerivation {
            name = "memory-x";
            src = ./path/to/memory.x;
            installPhase = "install -Dm644 $src $out/memory.x";
          }}/memory.x
        '';
      }
    );
  };
}
