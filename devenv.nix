{pkgs, ...}: {
  packages = with pkgs; [ nixd ]; # zed
  languages.nix.enable = true;
  languages.rust.enable = true;
}
