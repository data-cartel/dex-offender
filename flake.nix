{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
  };

  outputs = { self, nixpkgs, devenv, systems, ... } @ inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      devShells = forEachSystem
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            default = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                {
                  # https://devenv.sh/reference/options/
                  packages = [];

                  difftastic.enable = true;

                  scripts.bind.exec = let
                    contractsPath = "$(git rev-parse --show-toplevel)/contracts";
                  in ''
                    forge build --root ${contractsPath}
                    forge bind \
                      --bindings-path ./bindings \
                      --root ${contractsPath} \
                      --crate-name bindings \
                      --overwrite
                  '';

                  scripts.test.exec = ''
                    cargo test -p dex_offender -- --nocapture
                  '';

                  scripts.watch-test.exec = ''
                    cargo watch -x 'test -p dex_offender -- --nocapture'
                  '';

                  enterShell = ''
                    export PATH="$PATH:$PWD/.devenv/profile/bin"
                  '';
                }
              ];
            };
          });
    };
}
