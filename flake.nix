{
  description = "SoulBoard development environment with cross-compilation support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        fenixPkgs = fenix.packages.${system};
        
        # Rust toolchain with cross-compilation targets
        rustToolchain = fenixPkgs.stable.toolchain;
        windowsRustToolchain = fenixPkgs.combine [
          fenixPkgs.stable.toolchain
          fenixPkgs.targets.x86_64-pc-windows-gnu.stable.rust-std
        ];
        
        # Linux x86_64 native development
        linuxDevShell = pkgs.mkShell {
          name = "soulboard-linux-dev";
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            libxkbcommon
            wayland
            libxcb
            libxcb-util
            libxcb-keysyms
            libxcb-render-util
            libxcb-wm
            libx11
            libxrandr
            libxinerama
            libxi
            libxext
            libxcursor
            libGL
            vulkan-loader
          ];

          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
              pkgs.libxkbcommon
              pkgs.wayland
              pkgs.libxcb
              pkgs.libxcb-util
              pkgs.libx11
              pkgs.libGL
              pkgs.vulkan-loader
            ]}:$LD_LIBRARY_PATH
          '';
        };
        
        # Windows x86_64 cross-compilation (from Linux)
        windowsDevShell = pkgs.mkShell {
          name = "soulboard-windows-dev";
          buildInputs = with pkgs; [
            windowsRustToolchain
            pkgsCross.mingwW64.stdenv.cc
            pkgsCross.mingwW64.windows.pthreads
            pkg-config
          ];

          shellHook = ''
            export CARGO_BUILD_TARGET=x86_64-pc-windows-gnu
            export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
            export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
          '';
        };
      in {
        devShells = {
          default = linuxDevShell;
          linux = linuxDevShell;
          windows = windowsDevShell;
        };
      });
}
