{ lib, rustPlatform, openssl, pkg-config}:

rustPlatform.buildRustPackage rec {
  name = "customrgb-${version}";
  version = "0.1.0";
  src = ./.;

  buildInputs = [ 
    openssl
    pkg-config
  ];

  checkPhase = "";
  cargoHash = "sha256-1hiKrFPEdkcSo/OwTkXKZeC3BfOdpsTEpo4i7dizje4=";

  meta = {
    description = "My custom OpenRGB Config";
    homepage = https://github.com/prushton2/openrgb;
    license = lib.licenses.mit;
    platforms = lib.platforms.linux;
  };
}