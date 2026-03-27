{
  description = "Spaceblimp firmware";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.11-small";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        config.allowUnfreePredicate = pkg: builtins.elem (nixpkgs.lib.getName pkg) [ "vscode" ];
      };
    in
    let
      devshell = pkgs.mkShell {
        buildInputs = [
          pkgs.rustup
          pkgs.espup
          pkgs.espflash
          pkgs.esptool
          pkgs.vscode
        ];
        shellHook = ''
          if ! [ -f espidf.sh ]; then
            echo "Installing dependencies using rustup; this might take a while..."
            if ! espup install -f espidf.sh -t all; then
              echo "ERROR: Failed to install dependencies"
              exit 1
            fi
          fi
          if ! source espidf.sh; then
            echo "ERROR: The espidf.sh script failed"
            exit 1
          fi
        '';
      };
    in
    {
      devShells.x86_64-linux.default = devshell;

      # TODO: actually build our project
      packages.x86_64-linux.default = devshell;
    };
}
