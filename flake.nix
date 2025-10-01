{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-compat }: {
    packages.x86_64-linux.customrgb = (
      import nixpkgs {
        currentSystem = "x86_64-linux";
        localSystem = "x86_64-linux";
      }).pkgs.callPackage ./default.nix { };
    packages.x86_64-linux.default = self.packages.x86_64-linux.customrgb;

    nixosModules.default = { config, pkgs, ... }: import ./module.nix { inherit pkgs; };
  };
}