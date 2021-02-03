# A derivation for the wrapper shell script
# taken from: https://nixos.wiki/wiki/Nix_Cookbook
{ pkgs ? import nix/nixpkgs.nix { } }:
let
  sources = import nix/sources.nix;
  crate2nix = pkgs.callPackage sources.crate2nix { };
  cargoNix = pkgs.callPackage (import ./Cargo.nix) { };
in cargoNix.rootCrate.build.overrideAttrs
(oldAttrs: {
  buildInputs = (oldAttrs.buildInputs or []) ++ [
    pkgs.makeWrapper
  ];
  postInstall = (oldAttrs.postInstall or "") + ''
    wrapProgram $out/bin/cargo-nix \
      --prefix PATH : ${crate2nix}/bin
  '';
})
