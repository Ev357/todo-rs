{
  self,
  config,
  lib,
  pkgs,
  ...
}: let
  cfg = config.services.todo-web;
in {
  options.services.todo-web = {
    enable = lib.mkEnableOption "Web server for todo-rs";

    package = lib.mkPackageOption self.packages.${pkgs.stdenv.hostPlatform.system} "todo-web" {};

    address = lib.mkOption {
      type = lib.types.str;
      default = "127.0.0.1";
      description = "IP address on which todo-web will listen.";
    };

    port = lib.mkOption {
      type = lib.types.port;
      default = 7631;
      description = "Port on which todo-web will listen.";
    };

    openFirewall = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = "Whether to open the firewall for the todo-web port.";
    };
  };

  config = lib.mkIf cfg.enable {
    networking.firewall.allowedTCPPorts = lib.mkIf cfg.openFirewall [cfg.port];

    systemd.services.todo-web = {
      description = "Todo Web Application";
      after = ["network.target"];
      wantedBy = ["multi-user.target"];

      environment = {
        PORT = toString cfg.port;
        IP = cfg.address;
      };

      serviceConfig = {
        ExecStart = lib.getExe cfg.package;
        DynamicUser = true;
        Restart = "on-failure";
        RestartSec = "5s";

        CapabilityBoundingSet = "";
        NoNewPrivileges = true;
        LockPersonality = true;
        MemoryDenyWriteExecute = true;

        ProtectSystem = "strict";
        ProtectHome = true;
        PrivateTmp = true;
        ProtectProc = "invisible";

        ProtectClock = true;
        ProtectControlGroups = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        PrivateDevices = true;

        PrivateMounts = true;
        PrivateUsers = true;
        PrivateIPC = true;
        RemoveIPC = true;
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;

        RestrictAddressFamilies = ["AF_INET" "AF_INET6" "AF_UNIX"];
        SocketBindDeny = "any";
        SocketBindAllow = ["tcp:${toString cfg.port}"];

        SystemCallArchitectures = "native";
        SystemCallErrorNumber = "EPERM";
        SystemCallFilter = ["@system-service" "~@privileged"];
      };
    };
  };
}
