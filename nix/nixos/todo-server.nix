{
  config,
  lib,
  pkgs,
  ...
}: let
  cfg = config.services.todo-server;
in {
  options.services.todo-server = {
    enable = lib.mkEnableOption "Server for todo-rs";

    package = lib.mkPackageOption pkgs "todo-server" {};

    address = lib.mkOption {
      type = lib.types.str;
      default = "127.0.0.1";
      description = "IP address on which todo-server will listen.";
    };

    port = lib.mkOption {
      type = lib.types.port;
      default = 7630;
      description = "Port on which todo-server will listen.";
    };

    openFirewall = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = "Whether to open the firewall for the todo-server port.";
    };

    dataDir = lib.mkOption {
      type = lib.types.str;
      default = "/var/lib/todo-server";
      description = "Directory to store SQLite database files and state for todo-server.";
    };
  };

  config = lib.mkIf cfg.enable {
    networking.firewall.allowedTCPPorts = lib.mkIf cfg.openFirewall [cfg.port];

    systemd.services.todo-server = {
      description = "Todo API Server";
      after = ["network.target"];
      wantedBy = ["multi-user.target"];

      environment = {
        PORT = toString cfg.port;
        ADDRESS = cfg.address;
        DATABASE_URL = "sqlite://${cfg.dataDir}/todo.db";
      };

      serviceConfig = {
        ExecStart = lib.getExe cfg.package;
        WorkingDirectory = cfg.dataDir;
        StateDirectory = "todo-server";
        DynamicUser = true;
        Restart = "on-failure";
        RestartSec = "5s";
        UMask = "0077";

        CapabilityBoundingSet = "";
        NoNewPrivileges = true;
        LockPersonality = true;
        MemoryDenyWriteExecute = true;

        ProtectSystem = "strict";
        ProtectHome = true;
        PrivateTmp = true;
        ProtectProc = "invisible";
        ReadWritePaths = [cfg.dataDir];

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
