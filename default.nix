{
  pkgs,
  craneLib,
}:
let
  autoreconf = pkgs.writeShellScriptBin "autoreconf" "exit 0";
in
craneLib.buildPackage {
  pname = "minefetch";
  version = "0.1.0";

  src = craneLib.cleanCargoSource ./.;

  nativeBuildInputs = [
    autoreconf
    pkgs.clang
    pkgs.mold
  ];
}
