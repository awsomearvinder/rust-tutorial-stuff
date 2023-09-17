{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = {nixpkgs, ...}: let
    systems = ["x86_64-linux"];
    forSystem = config_generator: builtins.foldl' (config: system: config // (config_generator system)) {} systems;
  in
    forSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          pkgs.rustc
          pkgs.cargo
          pkgs.rust-analyzer
        ];
      };
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        name = "rust-tutorial";
        version = "0.0.1";
        src = ./.;
        cargoHash = "sha256-8pMR+B5ydfSIT6wmPjrxy5oSt+elBPvY9NmuurzEg0Y=";
      };
    });
}
