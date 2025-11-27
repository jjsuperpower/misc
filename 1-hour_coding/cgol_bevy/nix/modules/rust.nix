{ inputs, ... }:
{
  imports = [
    inputs.rust-flake.flakeModules.default
    inputs.rust-flake.flakeModules.nixpkgs
  ];
  perSystem = { config, self', pkgs, lib, ... }: {
    rust-project.crates."cgol_bevy".crane.args = {
      nativeBuildInputs = with pkgs; [
        pkg-config
      ];
      buildInputs = with pkgs; [
        # Linux dependencies for Bevy
      ]
      ++ lib.optionals (lib.strings.hasInfix "linux" stdenv.hostPlatform.system) [
        # Audio (Linux only)
        alsa-lib
        # Cross Platform 3D Graphics API
        vulkan-loader
        # Wayland support
        wayland
        wayland-protocols
        # Other dependencies
        libudev-zero
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        libxkbcommon
      ]
      # macOS dependencies
      ++ lib.optionals pkgs.stdenv.isDarwin (
        with pkgs.darwin.apple_sdk.frameworks; [
          IOKit
        ]
      );
    };
}
