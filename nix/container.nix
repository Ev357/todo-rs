{
  boot.isNspawnContainer = true;

  system.stateVersion = "26.05";

  services.todo-server = {
    enable = true;
    address = "0.0.0.0";
    openFirewall = true;
  };

  services.todo-web = {
    enable = true;
    address = "0.0.0.0";
    openFirewall = true;
  };
}
