{
  description = "Authentication component of redcardinal.io";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustc
              cargo
              rust-analyzer
              rustfmt
              clippy
              cargo-watch
              sqlx-cli
              cargo-audit
              cargo-expand
              jq
              pgcli
            ];
            shellHook = ''
              export CARGO_HOME="$HOME/.cargo"
              export PATH="$CARGO_HOME/bin:$PATH"
              
              if [ -f config.env ]; then
                source config.env
              fi
              echo "Available Commands:"
              echo "  cargo watch -x run  - Start development server with live reload"
              echo "  cargo test         - Run tests"
              echo "  sqlx migrate       - Run database migrations" 
              echo "  sqlx prepare       - Generate database queries for compile-time checking"
            '';
          };
        });
    };
}
