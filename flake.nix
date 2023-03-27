{
  description = "Build the Benwis Blog!";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
    
    cargo-leptos = {
      #url= "github:leptos-rs/cargo-leptos/v1.7";
      url = "github:benwis/cargo-leptos";
      flake = false;
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, advisory-db, rust-overlay, ... } @inputs:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          rustTarget = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
            targets = [ "wasm32-unknown-unknown" ];
          });

          # NB: we don't need to overlay our custom toolchain for the *entire*
          # pkgs (which would require rebuidling anything else which uses rust).
          # Instead, we just want to update the scope that crane will use by appendings
          inherit (pkgs) lib;
          # our specific toolchain there.
          craneLib = (crane.mkLib pkgs).overrideToolchain rustTarget;
          #craneLib = crane.lib.${system};
          # Only keeps markdown files
          protoFilter = path: _type: builtins.match ".*proto$" path != null;
          sqlxFilter = path: _type: builtins.match ".*json$" path != null;
          protoOrCargo = path: type:
            (protoFilter path type) || (craneLib.filterCargoSources path type) || (sqlxFilter path type);
          # other attributes omitted

          # Include more types of files in our bundle
          src = lib.cleanSourceWith {
            src = ./.; # The original, unfiltered source
            filter = protoOrCargo;
          };
          #    src = craneLib.cleanCargoSource ./.;

          # Common arguments can be set here
          commonArgs = {
            inherit src;
          buildInputs = [
            # Add additional build inputs here
            cargo-leptos
            pkgs.pkg-config
            pkgs.openssl
            pkgs.protobuf
            pkgs.binaryen
            pkgs.cargo-generate
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];
        };


          # Build *just* the cargo dependencies, so we can reuse
          # all of that work (e.g. via cachix) when running in CI
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          # Build the actual crate itself, reusing the dependency
          # artifacts from above.
          benwis_leptos = craneLib.buildPackage (commonArgs // {
            pname = "benwis_leptos";
            buildPhaseCargoCommand = "cargo leptos build --release";
            installPhaseCommand = ''
            mkdir -p $out/bin
            cp target/server/release/benwis_leptos $out/bin/
            cp -r target/site $out/bin/
            '';
            # Prevent cargo test and nextest from duplicating tests
            doCheck = false;
            inherit cargoArtifacts;
            # ALL CAPITAL derivations will get forwarded to mkDerivation and will set the env var during build
            SQLX_OFFLINE = "true";
            APP_ENVIRONMENT = "production";
          });
          cargo-leptos = pkgs.rustPlatform.buildRustPackage rec {
            pname = "cargo-leptos";
            #version = "0.1.7";
            version = "0.1.8.1";
            buildFeatures = ["no_downloads"]; # cargo-leptos will try to download Ruby and other things without this feature

            src = inputs.cargo-leptos; 

            cargoSha256 = "sha256-FBtbVli9qJQYsd6aLiizy9qup8E0VOVxkmYX6K09aO0=";

            nativeBuildInputs = [pkgs.pkg-config pkgs.openssl];

            buildInputs = with pkgs;
              [openssl pkg-config]
              ++ lib.optionals stdenv.isDarwin [
              Security
            ];

            doCheck = false; # integration tests depend on changing cargo config

            meta = with lib; {
            description = "A build tool for the Leptos web framework";
            homepage = "https://github.com/leptos-rs/cargo-leptos";
            changelog = "https://github.com/leptos-rs/cargo-leptos/blob/v${version}/CHANGELOG.md";
            license = with licenses; [mit];
            maintainers = with maintainers; [benwis];
          };
      };
          # Deploy the image to Fly with our own bash script
          flyDeploy = pkgs.writeShellScriptBin "flyDeploy" ''
            OUT_PATH=$(nix build --print-out-paths .#container)
            HASH=$(echo $OUT_PATH | grep -Po "(?<=store\/)(.*?)(?=-)")
            ${pkgs.skopeo}/bin/skopeo --insecure-policy --debug copy docker-archive:"$OUT_PATH" docker://registry.fly.io/$FLY_PROJECT_NAME:$HASH --dest-creds x:"$FLY_AUTH_TOKEN" --format v2s2
            ${pkgs.flyctl}/bin/flyctl deploy -i registry.fly.io/$FLY_PROJECT_NAME:$HASH --remote-only
          '';
        in
        {
          checks = {
            # Build the crate as part of `nix flake check` for convenience
            inherit benwis_leptos;

            # Run clippy (and deny all warnings) on the crate source,
            # again, resuing the dependency artifacts from above.
            #
            # Note that this is done as a separate derivation so that
            # we can block the CI if there are issues here, but not
            # prevent downstream consumers from building our crate by itself.
            benwis_leptos-clippy = craneLib.cargoClippy (commonArgs // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

            benwis_leptos-doc = craneLib.cargoDoc (commonArgs //{
              inherit cargoArtifacts;
            });

            # Check formatting
            benwis_leptos-fmt = craneLib.cargoFmt {
              inherit src;
            };

            # Audit dependencies
            benwis_leptos-audit = craneLib.cargoAudit {
              inherit src advisory-db;
            };

            # Run tests with cargo-nextest
            # Consider setting `doCheck = false` on `benwis_leptos` if you do not want
            # the tests to run twice
            # benwis_leptos-nextest = craneLib.cargoNextest {
            #  inherit cargoArtifacts src buildInputs;
            #  partitions = 1;
            #  partitionType = "count";
            #};
          } // lib.optionalAttrs (system == "x86_64-linux") {
            # NB: cargo-tarpaulin only supports x86_64 systems
            # Check code coverage (note: this will not upload coverage anywhere)
            #benwis_leptos-coverage = craneLib.cargoTarpaulin {
            #  inherit cargoArtifacts src;
            #};

          };

          packages.default = benwis_leptos;

          apps.default = flake-utils.lib.mkApp {
            drv = benwis_leptos;
          };

          # Create an option to build a docker image from this package 
          packages.container = pkgs.dockerTools.buildImage {
            name = "benwis_leptos";
            #tag = "latest";
            created = "now";
            copyToRoot = pkgs.buildEnv {
              name = "image-root";
              paths = [ pkgs.cacert ./.  ];
              pathsToLink = [ "/bin" "/configuration" "/keys" "/migrations" "App.db" ];
            };
            config = {
              Env = [ "PATH=${benwis_leptos}/bin" "APP_ENVIRONMENT=production" ];

              ExposedPorts = {
                "8080/tcp" = { };
              };

              Cmd = [ "${benwis_leptos}/bin/benwis_leptos" ];
            };

          };

          apps.flyDeploy = flake-utils.lib.mkApp {
            drv = flyDeploy;
          };
          devShells.default = pkgs.mkShell {
            inputsFrom = builtins.attrValues self.checks;

            # Extra inputs can be added here
            nativeBuildInputs = with pkgs; [
              rustTarget
              openssl
              mysql80
              dive
              sqlx-cli
              wasm-pack
              pkg-config
              binaryen
              nodejs
              nodePackages.tailwindcss
              cargo-leptos
              protobuf
              skopeo
              flyctl
            ];
            RUST_SRC_PATH = "${rustTarget}/lib/rustlib/src/rust/library";
          };
        });
}
