# Readiness fixtures

Readiness lock fixtures use the staged toolchain manifest at `qualification/tools/native-contract-toolchain.json`.

For tools whose manifest entry has `pathRoot: rustup-home`, fixtures retain the manifest-relative `executablePath`, such as `toolchains/1.98.0-x86_64-unknown-linux-gnu/bin/rustfmt`. The readiness test loader resolves that path through the manifest on the test host, using `RUSTUP_HOME` or `$HOME/.rustup`. This matches the lock verifier.

The fixtures retain absolute executable paths for Nix store tools. Nix store paths are immutable and don't depend on a developer home directory.

The convention applies to the complete synthetic fixture and every ready or production readiness lock fixture. Tests compare the remaining tool metadata and Nix store paths exactly after they normalize the Rustup prefix.
