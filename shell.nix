{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo-nextest
    cargo-tarpaulin
    rustup
    stdenv
  ];
}
