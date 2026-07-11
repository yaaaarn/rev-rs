{ lib
, rustPlatform
}:
rustPlatform.buildRustPackage {
  pname = "rev-rs";
  version = "0.1.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  meta = with lib; {
    description = "A blazingly fast, memory-safe rewrite of the Unix rev command";
    homepage = "https://github.com/yaaaarn/rev-rs";
    license = licenses.mit;
    platforms = platforms.unix;
    mainProgram = "rev";
  };
}
