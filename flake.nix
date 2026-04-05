{
  description = "SoulBoard development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
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
      });
}
