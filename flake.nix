{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";

    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      flake-parts,
      self,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      perSystem =
        { pkgs, ... }:
        {
          devShells.default = pkgs.mkShell {
            name = "minefetch";

            buildInputs = with pkgs; [
              cargo
              clang
              mold
            ];

            RUST_BACKTRACE = 1;
          };

          packages = rec {
            minefetch = pkgs.callPackage ./. { inherit self; };
            default = minefetch;
          };
        };
    };

  description = "Ping Minecraft servers from your terminal";
}
