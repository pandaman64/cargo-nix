{ pkgs ? import ./nix/nixpkgs.nix { } }:
let
  sources = import ./nix/sources.nix;
  crate2nix = pkgs.callPackage sources.crate2nix { };
in pkgs.mkShell { buildInputs = [ pkgs.nixfmt crate2nix ]; }
