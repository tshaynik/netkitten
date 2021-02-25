{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.evcxr
    pkgs.rust-analyzer
  ];
}
