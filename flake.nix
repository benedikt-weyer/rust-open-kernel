{
  description = "Open Kernel Rust compiler and std-port development shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
    in {
      devShells = systems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in {
          default = pkgs.mkShell {
            packages = with pkgs; [
              bash
              clang
              cmake
              coreutils
              curl
              file
              git
              gnumake
              ninja
              pkg-config
              python3
              which
            ];

            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.stdenv.cc.cc.lib
              pkgs.curl
              pkgs.libxml2
              pkgs.openssl
              pkgs.xz
              pkgs.zlib
            ];

            RUSTC_ICE = 0;
          };
        });
    };
}
