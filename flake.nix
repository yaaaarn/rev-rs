{
  description = "A blazingly fast(🦀), memory-safe rewrite of the Unix rev command, written in Rust(🦀🚀)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rev-rs = pkgs.callPackage ./package.nix { };
      in
      {
        packages = {
          default = rev-rs;
          inherit rev-rs;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ rev-rs ];
          packages = with pkgs; [
            cargo
            rustc
            rust-analyzer
          ];
        };
      }
    );
}
