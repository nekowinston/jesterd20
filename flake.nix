{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {flake-parts, ...} @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        pkgs,
        self',
        inputs',
        system,
        ...
      }: let
        toolchain = inputs'.fenix.packages.stable;
        rustPlatform = pkgs.makeRustPlatform {
          inherit (toolchain) cargo rustc;
        };
        inherit (pkgs) lib;
        inherit (pkgs.stdenv) isDarwin;
        inherit (pkgs.darwin.apple_sdk_11_0.frameworks) AppKit CoreServices OpenGL;
        commonBuildInputs =
          []
          ++ lib.optionals isDarwin [
            pkgs.libiconv
            AppKit
            CoreServices
            OpenGL
          ];
      in {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [inputs.fenix.overlays.default];
        };

        devShells.default = pkgs.mkShell {
          buildInputs =
            [
              (pkgs.fenix.complete.withComponents [
                "cargo"
                "clippy"
                "rust-analyzer"
                "rust-src"
                "rustc"
                "rustfmt"
              ])
            ]
            ++ commonBuildInputs;
          env.RUST_SRC_PATH = "${toolchain.rust-src}/lib/rustlib/src/rust/library";
        };

        packages.default = let
          name = "jesterd20";
          path = ./.;
        in
          rustPlatform.buildRustPackage {
            inherit name path;
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.nix-gitignore.gitignoreSource [] (builtins.path {inherit name path;});
            buildInputs = commonBuildInputs;
          };

        formatter = pkgs.alejandra;
      };
    };

  nixConfig = {
    extra-substituters = [
      "https://nekowinston.cachix.org"
      "https://nix-community.cachix.org/"
    ];
    extra-trusted-public-keys = [
      "nekowinston.cachix.org-1:lucpmaO+JwtoZj16HCO1p1fOv68s/RL1gumpVzRHRDs="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };
}
