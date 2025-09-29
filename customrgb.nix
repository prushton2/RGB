{ config, lib, pkgs, ... }:

let
  cfg = config.services.customrgb;
  configFile = pkgs.writeText "customrgb-config.yaml" ''
    SPEED: ${cfg.keyboard.speed}
    LEFT_TO_RIGHT: ${toString cfg.keyboard.left_to_right}
    BLEND: ${cfg.keyboard.blend}
    MODULO: ${cfg.keyboard.modulo}
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

    keyboard = lib.mkOption {
      default = { };
      description = "Keyboard Options";
      type = lib.types.submodule {
        options = {
          speed = lib.mkOption {
            type = lib.types.str;
            default = "0.25";
            description = "Speed of rainbow effect";
          };

          left_to_right = lib.mkOption {
            type = lib.types.bool;
            default = true;
            description = "Direction of rainbow effect";
          };

          blend = lib.mkOption {
            type = lib.types.str;
            default = "8.0";
            description = "Blend of rainbow effect";
          };

          modulo = lib.mkOption {
            type = lib.types.str;
            default = "48.0";
            description = "difference between two identical states";
          };

        };
      };
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.customrgb = {
      description = "Custom RGB";
      after = [ "openrgb.service" ];
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        ExecStart = "${cfg.package}/bin/customrgb ${configFile}";
        Restart = "always";
      };
    };
  };
}