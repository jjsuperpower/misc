{ inputs, ... }:
{
  perSystem = { config, self', pkgs, lib, ... }: {
    devShells.default = pkgs.mkShell {
      name = "cgof_bevy-shell";
      inputsFrom = [
        self'.devShells.rust
        config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
      ];
      buildInputs = with pkgs; [
        pkg-config
      ]
      # https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md
      ++ lib.optionals (lib.strings.hasInfix "linux" stdenv.hostPlatform.system) [
        # for Linux
        # Audio (Linux only)
        alsa-lib
        # Cross Platform 3D Graphics API
        vulkan-loader
        # For debugging around vulkan
        vulkan-tools
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
      ];
      packages = with pkgs; [
        fish
        just
        nixd # Nix language server
        bacon
      ];
      LD_LIBRARY_PATH = lib.makeLibraryPath (with pkgs; [
        vulkan-loader
        xorg.libX11
        xorg.libXi
        xorg.libXcursor
        libxkbcommon
        wayland
      ]);

        RUST_LOG = "info";

        shellHook = ''
          export fish_greeting=""
          exec ${pkgs.fish}/bin/fish
          export RUST_LOG="info"
        '';
      };
    };
}
