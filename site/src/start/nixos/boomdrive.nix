# The drive on NixOS: the family's cache trusted, the programs the later
# steps run, and the account me's drive home kept under crew. Put your
# own names for you, your-host, your-github-name and your-repository.
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
    settings.partitions = [
      {
        label = "home";
        partition = "home";
        blueprints = [ "your-github-name/your-repository" ];
        state = "your-github-name/drive-state";
        machine = "your-host";
        session = "home";
        review = {
          every = 5;
          hours = 4;
        };
        flume = {
          design = false;
          planner = false;
          reviewer = false;
          ops = false;
          operator = false;
        };
      }
    ];
  };
}
