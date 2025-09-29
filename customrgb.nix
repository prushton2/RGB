{ config, lib, pkgs, ... }:

let
  cfg = config.services.customrgb;
  configFile = pkgs.writeText "config.yaml" ''
    SPEED=${cfg.keyboard.speed}
    LEFT_TO_RIGHT=${cfg.keyboard.left_to_right}
    BLEND=${cfg.keyboard.blend}
    MODULO=${cfg.keyboard.modulo}
  '';
in
{
  options.services.customrgb = {
    enable = lib.mkEnableOption "customrgb";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.customrgb;
      description = "Package";
    };

  };

  config = lib.mkIf cfg.enable {
    systemd.services.customrgb = {
      description = "Custom RGB";
      after = [ "openrgb.service" ];
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        ExecStart = "${cfg.package}/bin/customrgb";
        Restart = "always";
      };
    };
  };
}