# The drive on NixOS: the family's cache trusted, the programs the later
# steps run, and the account me's drive home kept under crew, crew's
# drive home as setup gives it on a Mac. The set's module gives Flume the
# drive's stream and crew's source. Put your own names for you,
# your-host, your-github-name and your-repository.
{ pkgs, ... }:
{
  nix.settings = {
    extra-substituters = [ "https://boomdrive.cachix.org" ];
    extra-trusted-public-keys = [
      "boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag="
    ];
  };

  environment.systemPackages = [
    pkgs.git
    pkgs.gh
    pkgs.nodejs
  ];

  services.boomdrive = {
    enable = true;
    user = "you";
    accounts.me.drives.home.repositories.your-github-name = [ "your-repository" ];
    settings.control_plane.type = "crew";
  };

  services.crew = {
    enable = true;
    host = "your-host";
    settings = {
      hosts.your-host = {
        ssh = "your-host";
        kind = "nixos";
      };
      drives.home = {
        blueprints = [ "your-github-name/your-repository" ];
        state = "your-github-name/drive-state";
        machine = "your-host";
        session = "home";
        folder = "/home/you/Code";
        repos = [ ];
        accounts = {
          design = "me";
          planner = "me";
          reviewer = "me";
          ops = "me";
          operator = "me";
        };
        runs = {
          design = "standing";
          planner = "standing";
          reviewer = "standing";
          ops = "standing";
          operator = "standing";
        };
        review = {
          after_changes = 5;
          after_hours = 4;
        };
      };
    };
  };
}
