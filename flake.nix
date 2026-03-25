{
  description = "bive — Rust TUI client for AI agents";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "aarch64-darwin"
        "x86_64-linux"
        "aarch64-linux"
      ];
      perSystem = system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          inherit pkgs;
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.gcc
          ];
          devPackages = [
            pkgs.clippy
            pkgs.rustfmt
            pkgs.git
            pkgs.ripgrep
            pkgs.lefthook
            pkgs.taplo
            pkgs.typos
            pkgs.cargo-modules
            pkgs.cargo-audit
            pkgs.samply
          ];
        };
    in
    {
      overlays.default = final: prev: {
        bive = final.rustPlatform.buildRustPackage {
          pname = "bive";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          # Some tests run during the flake check are using git
          nativeCheckInputs = [ final.git ];
          # Nix sandbox sets HOME=/homeless-shelter which doesn't exist,
          # This causes add_workspace tests to reject tilde-expanded paths.
          preCheck = ''
            export HOME=$(mktemp -d)
          '';
        };
      };

      # Useful to build / install in the docker container
      packages = forAllSystems (
        system:
        let
          s = perSystem system;
        in
        {
          devtools = s.pkgs.buildEnv {
            name = "bive-devtools";
            paths = s.devPackages;
          };
        }
      );

      devShells = forAllSystems (
        system:
        let
          s = perSystem system;
        in
        {
          default = s.pkgs.mkShell {
            packages = s.buildInputs ++ s.devPackages;
          };
        }
      );
    };
}
