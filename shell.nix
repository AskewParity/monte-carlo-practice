let
  pkgs = import <nixpkgs> {};
in pkgs.mkShell rec {
    name = "monte-carlo";

    buildInputs = with pkgs; [
      gnumake

      cargo
      rustc

      texliveFull
      tdf
    ];
}

