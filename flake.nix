{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
  };

  outputs = { self, nixpkgs, devenv, systems, ... }@inputs:
    let forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in {
      devShells = forEachSystem (system:
        let pkgs = nixpkgs.legacyPackages.${system};
        in {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [{
              # https://devenv.sh/reference/options/
              packages = [ ];

              difftastic.enable = true;

              scripts.fullcheck.exec = ''
                forge build
                forge bind -b ./bindings --crate-name bindings --overwrite
                cargo test -p solutions -- --nocapture
              '';

              # enterShell = ''
              #   export PATH="$PATH:$PWD/.devenv/profile/bin"
              # '';

              languages.nix.enable = true;

              pre-commit.hooks.nixfmt.enable = true;
              pre-commit.hooks.rebuild-contracts-and-check-solutions = {
                enable = true;

                name = "Re-generating contract bindings and running tests";
                entry = "fullcheck";
                files = "\\.(rs|sol|toml)$";

                pass_filenames = false;
                raw = { verbose = true; };
              };
            }];
          };
        });
    };
}
