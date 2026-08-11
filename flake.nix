{
  description = "ghostfeed";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/e1c1b84752fb0897897380a3cae9dc7fcab91ca3";
    rust_overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane/dc7496d8ea6e526b1254b55d09b966e94673750f";
  };
  outputs = {...} @ inputs: let
    system = "x86_64-linux";
    pkgs = import inputs.nixpkgs {
      inherit system;
      overlays = [
        (import inputs.rust_overlay)
      ];
    };
    rust_pkg = pkgs.rust-bin.stable."1.97.1".default;
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = [
        pkgs.alejandra
        pkgs.pkg-config
        rust_pkg
        pkgs.openssl
      ];
      buildInputs = [];
      shellHook = '''';
    };
  };
}
