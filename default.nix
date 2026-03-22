{
  pkgs,
  rustPlatform,
  self,
  lib,
}:
rustPlatform.buildRustPackage {
  pname = "minefetch";
  version = builtins.toString (self.shortRev or self.dirtyShortRev or self.lastModified);

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = with pkgs; [
    clang
    mold
  ];

  buildInputs = with pkgs; [
    libsixel
  ];
}
