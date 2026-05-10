{
  inputs = {
    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";

    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      crane,
      fenix,
      flake-parts,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      perSystem =
        {
          pkgs,
          self',
          system,
          ...
        }:
        let
          craneLib = (crane.mkLib pkgs).overrideToolchain fenix.packages.${system}.stable.toolchain;
        in
        {
          devShells.default = craneLib.devShell {
            name = "minefetch";

            inputsFrom = [ self'.packages.default ];
            buildInputs = with pkgs; [
              taplo
            ];

            RUST_BACKTRACE = 1;
          };

          packages = rec {
            default = minefetch;
            minefetch = pkgs.callPackage ./. { inherit craneLib; };
          };

          formatter = pkgs.nixfmt;
        };
    };

  description = "Ping Minecraft servers from your terminal";
}
