{
  description = "brelok";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachSystem ["x86_64-linux" "aarch64-linux" "aarch64-darwin"]
      (system:
        let pkgs = import nixpkgs {inherit system;}; 
        in {
          devShell.default = pkgs.mkShell {
            buildInputs = with pkgs; [
            nushell
            ];
          };
        }
      );
}
