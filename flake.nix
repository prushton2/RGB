{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }@inputs: {
    defaultPackage = let 
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in  
      pkgs.rustPlatform.buildRustPackage rec {
        name = "custom-rgb-${version}";
        version = "0.1.0";
        src = ./.;

        buildInputs = with pkgs; [ 
          openssl
          pkg-config
        ];

        checkPhase = "";
        cargoHash = "sha256-aljNMSWxjcUbznEFyuECIWawXy9hfesETzS8XjNWYPo=";

        meta = {
          description = "My custom OpenRGB Config";
          homepage = https://github.com/prushton2/openrgb;
          license = pkgs.lib.licenses.mit;
          platforms = pkgs.lib.platforms.linux;
        };
      };
  };
}