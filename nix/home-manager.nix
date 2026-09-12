{
  lib,
  pkgs,
  config,
  ...
}: let
  cfg = config.programs.todo;

  tomlFormat = pkgs.formats.toml {};
in {
  options.programs.todo = {
    enable = lib.mkEnableOption "Yet another todo app.";

    package = lib.mkPackageOption pkgs "todo" {};

    settings = lib.mkOption {
      type = tomlFormat.type;
      default = {};
      example =
        lib.literalExpression
        # nix
        ''
          {
            api_url = "http://127.0.0.1:7630";
          }
        '';
      description = ''
        Configuration written to
        {file}`$XDG_CONFIG_HOME/todo/config.toml`.
      '';
    };

    enableNushellIntegration = lib.hm.shell.mkNushellIntegrationOption {inherit config;};
  };

  config = lib.mkIf cfg.enable {
    home.packages = [cfg.package];

    xdg.configFile."todo/config.toml" = lib.mkIf (cfg.settings != {}) {
      source = tomlFormat.generate "todo-settings" cfg.settings;
    };

    programs = {
      nushell.extraConfig =
        lib.mkIf cfg.enableNushellIntegration
        # nu
        ''
          source ${
            pkgs.runCommand "todo-nushell-config.nu" {} ''
              ${lib.getExe cfg.package} generate >> "$out"
            ''
          }
        '';
    };
  };
}
