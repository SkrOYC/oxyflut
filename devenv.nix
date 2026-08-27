{ pkgs, ... }:
let
  # nixpkgs provides rust-bindgen 0.72.1 and rust-cbindgen 0.29.4 at the
  # pinned input revision. The latter packages the upstream v0.29.4 source;
  # no project-local cargo installation is needed.
  clangWithCc = pkgs.runCommand "oxyflut-clang-with-cc" {} ''
    mkdir -p "$out/bin"
    ln -s ${pkgs.llvmPackages.clang}/bin/clang "$out/bin/cc"
    ln -s ${pkgs.llvmPackages.clang}/bin/clang "$out/bin/clang"
    ln -s ${pkgs.llvmPackages.clang}/bin/clang++ "$out/bin/c++"
    ln -s ${pkgs.llvmPackages.clang}/bin/clang++ "$out/bin/clang++"
  '';
in
{
  packages = [
    clangWithCc
    pkgs.lld
    pkgs.binutils
    pkgs.rust-bindgen
    pkgs.rust-cbindgen
    pkgs.bun
    pkgs.git
    pkgs.jq
    pkgs.cargo-deny
    pkgs.cargo-audit
    pkgs.cargo-fuzz
    pkgs.cargo-llvm-cov
  ];

  # Rust remains managed by the host rustup installation and rust-toolchain.toml.
  env = {
    CC = "${pkgs.llvmPackages.clang}/bin/clang";
    CXX = "${pkgs.llvmPackages.clang}/bin/clang++";
    LD = "${pkgs.lld}/bin/ld.lld";
    RUSTFLAGS = "-C link-arg=-fuse-ld=lld";
  };

  scripts = {
    fmt-check.exec = "cargo +1.98.0 fmt --all --check";
    clippy-check.exec = "cargo +1.98.0 clippy --workspace --all-targets --all-features -- -D warnings";
    test-all.exec = "cargo +1.98.0 test --workspace --all-features";
    docs-check.exec = "bunx prettier@3.9.6 --prose-wrap never --check '.constitution/**/*.md'";
  };
}
