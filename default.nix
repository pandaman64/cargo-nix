# A derivation for the wrapper shell script
# taken from: https://nixos.wiki/wiki/Nix_Cookbook
{ pkgs ? import nix/nixpkgs.nix { } }:
let
  sources = import nix/sources.nix;
  # CR pandaman: the upstream seems to produce buildRustCrate deprecation warning.
  # resolve this CR if the upstream fixes it.
  crate2nix = pkgs.callPackage sources.crate2nix { };
  # buildRustCrate seems deprecated
  cargoNix = pkgs.callPackage (import ./Cargo.nix) { buildRustCrate = null; };
in cargoNix.rootCrate.build.overrideAttrs (oldAttrs: {
  buildInputs = (oldAttrs.buildInputs or [ ]) ++ [ pkgs.makeWrapper ]
    ++ (pkgs.lib.optionals pkgs.stdenv.isDarwin [
      pkgs.darwin.apple_sdk.frameworks.Security
      pkgs.darwin.apple_sdk.frameworks.CoreServices
      pkgs.darwin.apple_sdk.frameworks.CoreFoundation
      pkgs.darwin.apple_sdk.frameworks.Foundation
      pkgs.darwin.apple_sdk.frameworks.AppKit
    ]);
  postInstall = (oldAttrs.postInstall or "") + ''
    wrapProgram $out/bin/cargo-nix \
      --prefix PATH : ${crate2nix}/bin
  '';
})
