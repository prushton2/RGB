{
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: {
    packages.x86_64-linux = 
      let
        system = "x86_64-linux";
        pkgs = import nixpkgs { inherit system; };
        lib = pkgs.lib;
        rustPlatform = pkgs.rustPlatform;
        openssl = pkgs.openssl;
        pkg-config = pkgs.pkg-config;
        myPackage = import ./default.nix { inherit lib rustPlatform openssl pkg-config; };
      in 
      {
        default = myPackage;
      };

    nixosModules.default = import ./customrgb.nix;
  };
}