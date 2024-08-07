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

          modules = [{
            packages = with pkgs;
              [ solc gcc foundry-pkg go-ethereum cargo-watch ]
              ++ lib.optionals stdenv.isDarwin (with darwin.apple_sdk; [
                libiconv
                frameworks.Security
                frameworks.CoreFoundation
                frameworks.SystemConfiguration
              ]);

            dotenv.enable = true;
            difftastic.enable = true;

            # https://devenv.sh/languages/
            languages.nix.enable = true;
            languages.rust = {
              enable = true;
              toolchain = channel.toolchain;
            };

            scripts.bind-attack.exec = ''
              ${forge} install
              ${forge} fmt
              ${forge} bind -b ./attack/src/abi --module --force --overwrite
              pre-commit run rustfmt -a > /dev/null || true
            '';

            scripts.bind-ctf.exec = ''
              ${forge} install --root ctf
              ${forge} fmt --root ctf
              ${forge} bind --root ctf -b ./ctf/src/abi --module --skip-cargo-toml --force --overwrite
              pre-commit run rustfmt -a > /dev/null || true
            '';

            scripts.deploy-levels.exec = ''
              if [ -f state.json ]; then
                rm -v state.json
              fi
              cargo run --bin deploy_levels
            '';

            # https://devenv.sh/pre-commit-hooks/
            pre-commit.hooks = {
              nixfmt = {
                enable = true;
                fail_fast = true;
                package = pkgs.nixfmt-classic;
              };
              rustfmt = {
                enable = true;
                packageOverrides = { inherit (channel) cargo rustfmt; };
              };
              # bind-attack-contracts = {
              #   enable = true;
              #   name = "Bind attack contracts";
              #   description =
              #     "Build attack/contracts/ contracts and generate Rust ABI bindings";
              #   files = "attack/contracts/.*.sol$";
              #   entry = ".devenv/profile/bin/bind-attack";
              #   pass_filenames = false;
              #   verbose = true;
              # };
              # bind-ctf-contracts = {
              #   enable = true;
              #   name = "Bind CTF contracts";
              #   description =
              #     "Compile CTF smart contracts and generate Rust ABI bindings";
              #   files = "ctf/contracts/.*.sol$";
              #   entry = ".devenv/profile/bin/bind-ctfs";
              #   pass_filenames = false;
              #   verbose = true;
              # };
              # deploy-levels = {
              #   enable = true;
              #   name = "Deploy levels";
              #   files = "ctf/src/.*.rs";
              #   entry = ".devenv/profile/bin/deploy-levels";
              #   pass_filenames = false;
              #   verbose = true;
              # };
            };
          }];
        };
      });
}
