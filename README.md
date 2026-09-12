# todo-rs
Yet another todo app.

I just want to try to build my own todo app (and procrastinate).
Also wanted to try out Dioxus.

## Features
- It's just glorified wrapper around a simple sqlite database, nothing fancy.
- The web also works without JS :D

## Installation
```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    todo = {
      url = "github:Ev357/todo-rs";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    todo,
    ...
  }: {
    nixosConfigurations."«hostname»" = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        todo.nixosModules.default
        ./configuration.nix
      ];
    };
  };
}
```

### Default configuration
```nix
{
  services.todo-server = {
    enable = false;
    address = "127.0.0.1";
    port = 7630;
    openFirewall = false;
  };

  services.todo-web = {
    enable = true;
    address = "127.0.0.1";
    port = 7631;
    openFirewall = false;
    apiUrl = "http://127.0.0.1:7630";
  };
}
```

## Development
- [x] Web support.
- [ ] Cli/Tui support.
- [ ] Desktop support.
- [ ] Mobile support.
