# SPDX-FileCopyrightText: 2025 Matthew Mark Ibbetson
# SPDX-FileContributor: Matthew Mark Ibbetson
#
# SPDX-License-Identifier: GPL-3.0-or-later

{
  lib,
  gitSupport ? true,
  fetchFromGitHub,
  rustPlatform,
  pkg-config,
  installShellFiles,
}:

rustPlatform.buildRustPackage rec {
  pname = "dn";
  version = "0.1.2";

  src = fetchFromGitHub {
    owner = "mmibbetson";
    repo = "dn";
    rev = "v${version}";
    sha256 = "sha256-8NZtCr3amrehw679eZmSqFiaXnplx4fdLxkilm8sU9w=";
  };

  cargoHash = "sha256-3JYcdRC16BIHeBk/AePC+6XorV1MWlpeZl2AlqoQmHI=";

  nativeBuildInputs = [
    pkg-config
    installShellFiles
  ];
  buildInputs = [ ];

  outputs = [
    "out"
    "man"
  ];

  postInstall =
    ''
      installManPage man/dn.1 man/dn-new.1 man/dn-rename.1
      installShellCompletion \
        --bash completions/dn.bash \
        --fish completions/dn.fish \
        --zsh completions/_dn
    '';

  meta = with lib; {
    description = "A simple, minimal, and flexible command line utility for organising plaintext files.";
    homepage = "https://mmibbetson.github.io/software/dn";
    changelog = "https://github.com/mmibbetson/dn/CHANGELOG.md";
    license = licenses.gpl3Plus;
    mainProgram = "dn";
    maintainers = with maintainers; [
      mmibbetson
    ];
    platforms = platforms.unix ++ platforms.windows;
  };
}
