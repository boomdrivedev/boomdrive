{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # Its nixpkgs stays its own, so its packages are the cache's.
    boomdrive.url = "github:boomdrivedev/boomdrive";
  };

  outputs =
    { nixpkgs, boomdrive, ... }:
    {
      nixosConfigurations.your-host = nixpkgs.lib.nixosSystem {
        modules = [
          ./configuration.nix
          boomdrive.nixosModules.default
          ./boomdrive.nix
        ];
      };
    };
}
