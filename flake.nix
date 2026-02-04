{
  description = "Spaceblimp firmware";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-25.11-small";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
    in
    let
      devshell = pkgs.mkShell {
        buildInputs = [
          pkgs.rustup
          pkgs.espup
          pkgs.espflash
        ];
        shellHook = ''
          if ! [ -f espidf.sh ]; then
            echo "Installing dependencies using rustup; this might take a while..."
            # The purpose of `--log-level warn` is to hide the info log message
            # where `espup` tells the user to `source espidf.sh`.
            if ! espup install -f espidf.sh -t all --log-level warn; then
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
