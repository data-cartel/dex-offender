{
  inputs = {
    devenv.url = "github:cachix/devenv";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    foundry = {
      url = "github:shazow/foundry.nix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, foundry, devenv }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        foundry-pkg = foundry.defaultPackage.${system};
        forge = "${foundry-pkg}/bin/forge";
        channel = inputs.fenix.packages.${pkgs.system}.latest;
      in {
        devShell = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              packages = with pkgs; [
                foundry-pkg
                channel.rustc
                channel.cargo
                channel.rustfmt
                channel.clippy
                rust-analyzer
              ];

              languages.rust.enable = true;
              languages.rust.channel = "stable";

              env.FORGE = forge;

              scripts.deploy.exec = ''
                cargo run --bin deploy_levels
              '';

              scripts.test.exec = ''
                cargo test -p attack
              '';

              scripts.check.exec = ''
                cargo clippy --all-targets --all-features -- -D warnings
                cargo fmt --all -- --check
              '';

              pre-commit.hooks = {
                rustfmt.enable = true;
                clippy.enable = true;
              };
            }
          ];
        };
      });
}
