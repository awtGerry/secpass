{
  lib,
  stdenv,
  rustPlatform,
  fetchFromGitHub,
}:

rustPlatform.buildRustPackage rec {
  pname = "secpass";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "awtGerry";
    repo = "secpass";
    rev = "v0.1.0";
  }
