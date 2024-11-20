{
  description = "Devshell for oor-nmea0183";

  inputs = {
    pklcli.url = "github:MattCairns/pkl-flake";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
      in {
        formatter = pkgs.alejandra;
        devShells.default = with pkgs;
          mkShell {
            RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
            LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
            shellHook = ''
              export LD_LIBRARY_PATH=${pkgs.wayland}/lib:$LD_LIBRARY_PATH
              export LD_LIBRARY_PATH=${pkgs.libxkbcommon}/lib:$LD_LIBRARY_PATH
              export LD_LIBRARY_PATH=${pkgs.libGL}/lib:$LD_LIBRARY_PATH
            '';
            buildInputs = [
              rustup # run `rustup toolchain install stable`
              git-cliff # git cliff --unreleased --tag v0.0.1 --prepend CHANGELOG.md
            ];
          };
      }
    );
}
