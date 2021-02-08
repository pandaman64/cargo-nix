{ pkgs ? import nix/nixpkgs.nix { } }:
let
  sources = import nix/sources.nix;
  cargo2nix = pkgs.callPackage sources.cargo2nix { };
in pkgs.mkShell { buildInputs = [ pkgs.nixfmt cargo2nix.package ]; }
