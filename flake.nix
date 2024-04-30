{
    inputs = {
        utils.url = "github:numtide/flake-utils";
        fenix = {
            url = "github:nix-community/fenix";
            inputs.nixpkgs.follows = "nixpkgs";
        };
        nixpkgs.url = "nixpkgs/nixos-unstable";
        pre-commit-hooks = {
            url = "github:cachix/pre-commit-hooks.nix";
            # inputs.nixpkgs.folows = "nixpkgs";
            # inputs.flake-utils.folows = "utils";
        };
    };

    outputs = { self, fenix, nixpkgs, utils, pre-commit-hooks }:
        utils.lib.eachDefaultSystem (system:
            let
                pkgs = import nixpkgs { inherit system; };
            in
            {
                checks = {
                    pre-commit-check = pre-commit-hooks.lib.${system}.run {
                        src = ./.;
                        hooks = {
                            rustfmt.enable = true;
                            clippy.enable = true;
                        };
                    };
                };
            devShells.default = pkgs.mkShell {
                inherit (self.checks.${system}.pre-commit-check) shellHook;
                nativeBuildInputs =
                    [
                        fenix.packages.${system}.complete.toolchain
                    ];
            };
        });
}
