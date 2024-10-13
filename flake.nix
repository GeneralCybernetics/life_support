{
  description = "LifeSupport devShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, fenix, flake-utils, ... }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (fenix.overlay) ];
        };
        toolchain = fenix.packages.${system}.fromToolchainFile
        {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-KmwhJWKRbcFnauS/A2/laoZlRSrEJ6jnob3ZWVpFoiI=";
        };
      in
      {
        devShells.default = pkgs.mkShell {
        nativeBuildInputs = [
          toolchain
          pkgs.probe-rs
          pkgs.rust-analyzer
          pkgs.gdb 
        ];

          # shellHook = ''
          #   export MEMORY_X_PATH=${pkgs.stdenvNoCC.mkDerivation {
          #     name = "memory-x";
          #     src = ./memory.x;
          #     installPhase = "mkdir -p $out && cp -pm 644 $src $out/memory.x";
          #     unpackPhase = "";
          #   }}/memory.x
          # '';
        };
      }
      
    );
}
