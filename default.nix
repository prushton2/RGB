{ pkgs, ... }: #lib, openssl, pkg-config, rustPlatform, ... }:

pkgs.rustPlatform.buildRustPackage rec {
  name = "customrgb-${version}";
  version = "0.1.0";
  src = ./.;

  nativeBuildInputs = [ 
    pkgs.pkg-config
  ];
  buildInputs = [ 
    pkgs.openssl
  ];

  checkPhase = "";
  cargoHash = "sha256-ts/T8Jcuc7VhoY+8Rq9isnTiaH370hrseusKU5gxJOk=";

  meta = {
    description = "My custom OpenRGB Config";
    homepage = https://github.com/prushton2/openrgb;
    license = pkgs.lib.licenses.mit;
    platforms = pkgs.lib.platforms.linux;
  };
}