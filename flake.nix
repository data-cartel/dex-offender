{
  inputs = {
    devenv.url = "github:cachix/devenv";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    foundry = {
      url =
        "github:shazow/foundry.nix/monthly"; # Use monthly branch for permanent releases
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, foundry, devenv }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in {
        devShell = devenv.lib.mkShell {
          inherit inputs pkgs;

          modules = [{
            packages = with pkgs;
              [ solc gcc foundry.defaultPackage.${system} go-ethereum tldr ]
              ++ lib.optionals stdenv.isDarwin (with darwin.apple_sdk; [
                libiconv
                frameworks.Security
                frameworks.CoreFoundation
              ]);

            # https://devenv.sh/languages/
            languages.nix.enable = true;
            languages.rust = {
              enable = true;
              channel = "stable";
              toolchain = {
                rustfmt = inputs.fenix.packages.${pkgs.system}.latest.rustfmt;
                clippy = inputs.fenix.packages.${pkgs.system}.latest.clippy;
              };
            };

            scripts.gen.exec = ''
              forge fmt
              forge bind -b ./bindings --crate-name bindings --overwrite
              cargo fmt
              cargo clippy
              cargo build --workspace --all-features --all-targets
              cargo run --bin deploy_levels
            '';

            scripts.bind.exec = ''
              forge fmt
              forge bind -b ./bindings --crate-name bindings --overwrite
            '';

            difftastic.enable = true;

            # https://devenv.sh/pre-commit-hooks/
            pre-commit.hooks = {
              nixfmt.enable = true;
              forge-fmt = {
                enable = true;

                name = "Format Solidity code";
                entry = "forge fmt";

                pass_filenames = false;
                raw.verbose = true;
              };
              bind-contracts = {
                enable = true;

                name = "Generate Rust bindings for Solidity contracts";
                entry =
                  "forge bind -b ./bindings --crate-name bindings --overwrite";

                pass_filenames = false;
                raw.verbose = true;
              };
              cargo-fmt = {
                enable = true;

                name = "Format Rust code";
                entry = "cargo fmt";

                pass_filenames = false;
                raw.verbose = true;
              };
              # cargo-build = {
              #   enable = true;

              #   name = "Compile Rust code";
              #   entry = "cargo build --workspace --all-features --all-targets";

              #   pass_filenames = false;
              #   raw.verbose = true;
              # };
            };
          }];
        };
      });
}
