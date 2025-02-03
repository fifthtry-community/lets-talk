{
  description = "lets-talk";

  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = { self, nixpkgs, rust-overlay }:
    let
      systems = [ "x86_64-linux" "aarch64-darwin" ];
      forAllSystems = f:
        nixpkgs.lib.genAttrs systems
          (system:
            f (import nixpkgs {
              inherit system;
              overlays = [ (import rust-overlay) ];
            }));
    in
    {
      devShells = forAllSystems (pkgs:
        let
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              pkg-config
            ] ++ lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Foundation ];

            buildInputs = with pkgs; [
              toolchain
              rust-analyzer-unwrapped
              cargo-expand

              wasm-pack # for www/ide
              nodejs_20 # for www/ide

              rlwrap
              sqlite
            ];

            shellHook = ''
              source scripts/auto.sh
            '';


            RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          };
        });

      formatter = forAllSystems (pkgs: pkgs.nixpkgs-fmt);
    };
}
