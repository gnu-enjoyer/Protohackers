with import <nixpkgs> {};

pkgsStatic.stdenv.mkDerivation {
  name = "static-shell";

  nativeBuildInputs = [ cmake ];

  shellHook =
    ''
      cmake . && cmake --build .
      cp 00-echo-server/echo-server /out
    '';

}
