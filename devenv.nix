# https://devenv.sh/getting-started
{pkgs, ...}: {
  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = ["wasm32-unknown-unknown"];
  };

  # https://devenv.sh/packages/
  packages = with pkgs; [
    cargo-audit
    cargo-auditable
    cargo-insta # https://github.com/mitsuhiko/insta
    cargo-machete
    cargo-mutants
    cargo-outdated
    cargo-semver-checks
    cargo-tarpaulin
    codespell
    git
    just
    onefetch
  ];

  # https://devenv.sh/pre-commit-hooks/
  # https://devenv.sh/reference/options/#pre-commit
  pre-commit.hooks = {
    # Files
    check-symlinks.enable = true;
    # Rust
    rustfmt.enable = true;
    # Nix
    alejandra.enable = true;
    deadnix.enable = true;
    statix.enable = true;
    # Shell
    shellcheck.enable = true;
    shfmt.enable = true; # alternative: beautysh
    # Misc. formats
    check-json.enable = true;
    check-toml.enable = true;
    check-yaml.enable = true;
    denofmt.enable = true;
    markdownlint.enable = true; # alternative: mdl
    # Security
    # ripsecrets.enable = true;
    detect-private-keys.enable = true;
    # Hyperlinks
    # lychee.enable = true;
    check-vcs-permalinks.enable = true;
    # EditorConfig
    editorconfig-checker.enable = true; # https://EditorConfig.org
    end-of-file-fixer.enable = true;
    trim-trailing-whitespace.enable = true;
    # Git
    commitizen.enable = true; # https://www.conventionalcommits.org/en/v1.0.0/#summary
    check-merge-conflicts.enable = true;
    check-added-large-files.enable = true;
  };

  enterShell = ''
    # set -x  # for debugging

    onefetch --no-color-palette --no-art --no-title
    echo "Run 'just' often, to prevent CI failures."
    echo ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  '';
}
