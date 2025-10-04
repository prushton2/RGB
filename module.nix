{ options, config, lib, pkgs, stdenv, ... }:

let
  cfg = config.services.customrgb;
  customrgb = pkgs.callPackage ./default.nix {};
  configFile = pkgs.writeText "customrgb-config.yaml" ''
    keyboard:
      speed: ${cfg.keyboard.speed}
      left_to_right: ${toString cfg.keyboard.left_to_right}
      blend: ${cfg.keyboard.blend}
      modulo: ${cfg.keyboard.modulo}
    cooler:
      speed: ${cfg.cooler.speed}
      left_to_right: ${toString cfg.cooler.left_to_right}
      blend: ${cfg.cooler.blend}
      modulo: ${cfg.cooler.modulo}
  '';
in
{
  options.services.customrgb = {
    enable = lib.mkEnableOption "customrgb";

    package = lib.mkOption {
      type = lib.types.package;
      default = customrgb;
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

    cooler = lib.mkOption {
      default = { };
      description = "Cooler Options";
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
            default = "24.0";
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