{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    bacon
    cargo-nextest
    cargo-tarpaulin
    rustup
    stdenv
  ];
}
