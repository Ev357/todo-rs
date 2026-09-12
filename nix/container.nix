{
  boot.isContainer = true;

  system.stateVersion = "26.05";

  services.todo-server = {
    enable = true;
    openFirewall = true;
  };

  services.todo-web = {
    enable = true;
    openFirewall = true;
  };
}
