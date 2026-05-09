{
  pkgs,
  craneLib,
}:
craneLib.buildPackage {
  pname = "minefetch";
  version = "0.1.0";

  src = craneLib.cleanCargoSource ./.;

  nativeBuildInputs = with pkgs; [
    clang
    mold
  ];

  buildInputs = with pkgs; [
    libsixel
  ];
}
