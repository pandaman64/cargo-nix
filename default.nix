{ system ? builtins.currentSystem, sources ? import nix/sources.nix
, nixpkgs ? sources.nixpkgs, nixpkgsMozilla ? sources.nixpkgs-mozilla
, cargo2nix ? sources.cargo2nix, crate2nix ? sources.crate2nix }:
let
  rustOverlay = import "${nixpkgsMozilla}/rust-overlay.nix";
  cargo2nixOverlay = import "${cargo2nix}/overlay";
  pkgs = import nixpkgs {
    inherit system;
    overlays = [ rustOverlay cargo2nixOverlay ];
  };
  crate2nix = pkgs.callPackage sources.crate2nix { };
  rustPkgs = pkgs.rustBuilder.makePackageSet' {
    rustChannel = "stable";
    packageFun = import ./Cargo.nix;
  };

  cargoNix = rustPkgs.workspace.cargo-nix { };
in pkgs.stdenv.mkDerivation {
  name = "cargo-nix";
  unpackPhase = "true";
  buildInputs = [ pkgs.makeWrapper ];
  installPhase = ''
    makeWrapper ${cargoNix}/bin/cargo-nix $out/bin/cargo-nix \
      --prefix PATH : ${crate2nix}/bin
  '';
}
