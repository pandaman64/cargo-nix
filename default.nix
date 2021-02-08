{ system ? builtins.currentSystem, sources ? import nix/sources.nix
, nixpkgs ? sources.nixpkgs, nixpkgsMozilla ? sources.nixpkgs-mozilla
, cargo2nix ? sources.cargo2nix, crate2nix ? sources.crate2nix }:
let
  rustOverlayPath = "${nixpkgsMozilla}/rust-overlay.nix";
  rustOverlay = import rustOverlayPath;

  cargo2nixOverlayPath = "${cargo2nix}/overlay";
  cargo2nixOverlay = import cargo2nixOverlayPath;

  pkgs = import nixpkgs {
    inherit system;
    overlays = [ rustOverlay cargo2nixOverlay ];
  };

  cargo2nixBin = (pkgs.callPackage cargo2nix {}).package;
  crate2nixBin = pkgs.callPackage sources.crate2nix { };

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
      --prefix PATH : ${pkgs.lib.makeBinPath [ cargo2nixBin crate2nixBin ]} \
      --set RUST_OVERLAY_PATH ${rustOverlayPath} \
      --set CARGO2NIX_OVERLAY_PATH ${cargo2nixOverlayPath}
  '';
}
