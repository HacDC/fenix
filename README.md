# Franken-Fenix

[HacDC's Spaceblimp VIII](https://wiki.hacdc.org/en/Projects/spaceblimp/Space-Blimp)

![Franken Fenix Prototype PCB render](/electronics/PCB.png)

# Supported Development Platforms

In theory it shouldn't really matter what operating system your computer
uses. This may or may not be the case in practice:

* Ubuntu: works

* MacOS: unknown

* Windows Subsystem for Linux: unknown

* NixOS: Does not work (NixOS/nixpkgs#372653). Ivan is trying to find a
  workaround that we can use until the proper fix is merged
  (NixOS/nixpkgs#477552).

# Setup

1. If you haven't already done so, you must first install Nix. The official
   installation guide can be found here: https://nix.dev/install-nix

2. Any time you open a new terminal window, you will need to run
   `nix develop`.
