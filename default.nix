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
  version = "0.1.3";

  src = fetchFromGitHub {
    owner = "mmibbetson";
    repo = "dn";
    rev = "v${version}";
    sha256 = "sha256-D7ccY8Vq8grUwjojPzVlpBPmyU1iUwt41gYAs4rOzaI=";
  };

  cargoHash = "sha256-+QHCYQ/0Kl4cp1nyq2nCFMKNz37p8IUhq7LdF62DVRk=";

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
