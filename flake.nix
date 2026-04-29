{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
    lazytimerPkg = pkgs.rustPlatform.buildRustPackage {
      pname = "lazytimer";
      inherit ((fromTOML (builtins.readFile ./Cargo.toml)).package) version;
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
    };
  in {
    packages.x86_64-linux.default = lazytimerPkg;
    homeModules.lazytimer = {
      lib,
      config,
      ...
    }: {
      options.programs.lazytimer = {
        enable = lib.mkEnableOption "lazytimer";
        settings = lib.mkOption {
          type = lib.types.attrs;
          default = {};
          description = "Configuration written to $XDG_CONFIG_HOME/lazytimer/config.toml";
        };
      };
      config = lib.mkIf config.programs.lazytimer.enable {
        home.packages = [lazytimerPkg];
        xdg.configFile."lazytimer/config.toml".source =
          (pkgs.formats.toml {}).generate "lazytimer-config" config.programs.lazytimer.settings;
      };
    };
  };
}
