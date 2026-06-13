{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  buildInput = with pkgs; [
    rustc
    cargo
    rust-analyzer
  ];
}
