{ pkgs ? import nix/nixpkgs.nix { } }:
let
  sources = import nix/sources.nix;
  crate2nix = pkgs.callPackage sources.crate2nix { };
in pkgs.mkShell {
  buildInputs = [ pkgs.nixfmt pkgs.niv crate2nix ]
    ++ (pkgs.lib.optionals pkgs.stdenv.isDarwin [
      pkgs.darwin.apple_sdk.frameworks.Security
      pkgs.darwin.apple_sdk.frameworks.CoreServices
      pkgs.darwin.apple_sdk.frameworks.CoreFoundation
      pkgs.darwin.apple_sdk.frameworks.Foundation
      pkgs.darwin.apple_sdk.frameworks.AppKit
    ]);
}
