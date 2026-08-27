{ pkgs, ... }:
let
  # nixpkgs provides rust-bindgen 0.72.1 and rust-cbindgen 0.29.4 at the
  # pinned input revision. The latter packages the upstream v0.29.4 source;
  # no project-local cargo installation is needed.
  clangWithCc = pkgs.runCommand "oxyflut-clang-with-cc" {} ''
    mkdir -p "$out/bin"
    ln -s ${pkgs.llvmPackages_21.clang}/bin/clang "$out/bin/cc"
    ln -s ${pkgs.llvmPackages_21.clang}/bin/clang "$out/bin/clang"
    ln -s ${pkgs.llvmPackages_21.clang}/bin/clang++ "$out/bin/c++"
    ln -s ${pkgs.llvmPackages_21.clang}/bin/clang++ "$out/bin/clang++"
  '';
  prettier396 = pkgs.buildNpmPackage {
    pname = "prettier";
    version = "3.9.6";
    src = pkgs.fetchurl {
      url = "https://registry.npmjs.org/prettier/-/prettier-3.9.6.tgz";
      hash = "sha512-OpN0zzVdiaiAhxpuuj5efpIS4sY9j7bY6uR5mnj5yPzGkdkjNKSJeUThPb60Jw29QuAZgA4o+/iB49kFiaBX6g==";
    };
    sourceRoot = "package";
    postPatch = ''
      cat > package-lock.json <<'EOF'
      {
        "name": "prettier",
        "version": "3.9.6",
        "lockfileVersion": 3,
        "requires": true,
        "packages": {
          "": {
            "name": "prettier",
            "version": "3.9.6"
          }
        }
      }
      EOF
    '';
    forceEmptyCache = true;
    npmDepsHash = "sha256-VERkDP5Al98PLAIprM2+dIVrSsvUJ/Stozmey30AOLY=";
    dontNpmBuild = true;
    nativeBuildInputs = [ pkgs.makeWrapper ];
    installPhase = ''
      runHook preInstall
      mkdir -p "$out/lib/node_modules"
      cp --recursive . "$out/lib/node_modules/prettier"
      makeWrapper ${pkgs.nodejs}/bin/node "$out/bin/prettier" \
        --add-flags "$out/lib/node_modules/prettier/bin/prettier.cjs"
      runHook postInstall
    '';
  };
in
{
  packages = [
    clangWithCc
    pkgs.lld
    pkgs.binutils
    pkgs.rust-bindgen
    pkgs.rust-cbindgen
    prettier396
    pkgs.git
    pkgs.jq
    pkgs.cargo-deny
    pkgs.cargo-audit
    pkgs.cargo-fuzz
    pkgs.cargo-llvm-cov
  ];

  # Rust remains managed by the host rustup installation and rust-toolchain.toml.
  env = {
    CC = "${pkgs.llvmPackages_21.clang}/bin/clang";
    CXX = "${pkgs.llvmPackages_21.clang}/bin/clang++";
    LD = "${pkgs.lld}/bin/ld.lld";
    RUSTFLAGS = "-C link-arg=-fuse-ld=lld";
  };

  scripts = {
    contracts-validate.exec = "cargo +1.98.0 run -p xtask -- contracts validate";
    fmt-check.exec = "cargo +1.98.0 fmt --all --check";
    clippy-check.exec = "cargo +1.98.0 clippy --workspace --all-targets --all-features -- -D warnings";
    test-all.exec = "cargo +1.98.0 test --workspace --all-features";
    docs-check.exec = "prettier --prose-wrap never --check '.constitution/**/*.md'";
    deny-check.exec = "cargo deny check licenses bans sources";
  };
}
