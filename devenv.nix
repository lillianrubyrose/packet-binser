{pkgs, ...}: {
  packages = with pkgs; [ nixd cargo-expand ]; # zed
  languages.nix.enable = true;
  languages.rust.enable = true;
}
