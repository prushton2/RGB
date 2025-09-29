with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
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
    license = lib.licenses.mit;
    platforms = lib.platforms.linux;
  };
}