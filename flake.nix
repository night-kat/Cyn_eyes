{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    ggezDependencies = with pkgs; [
      alsa-lib
      udev
      libX11
      libXcursor
      libXi
      libXrandr
      vulkan-loader
    ];
  in {
    devShells."x86_64-linux".default = pkgs.mkShell {
      packages = with pkgs;
        [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
          bacon
          # pkg-config
          llvmPackages.clang
          llvmPackages.lld
          llvmPackages.libclang
          cmake
          gcc
          expat
          freetype
          fontconfig
        ]
        ++ ggezDependencies;

      nativeBuildInputs = [pkgs.pkg-config];

      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath ggezDependencies;
      shellHook = ''
        export CARGO_CLIPPY_FLAGS='-- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used'
      '';
    };
  };
}
