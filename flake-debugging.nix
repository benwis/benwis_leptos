{
  description = "Build the Benwis Blog!";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
    
    cargo-leptos-git = {
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

          rustTarget = pkgs.rust-bin.nightly."2023-06-30".default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
            targets = [ "wasm32-unknown-unknown" ];
          };


          #rustTarget = pkgs.rust-bin.nightly."2023-06-01".default.override{
          #  extensions = [ "rust-src" "rust-analyzer" ];
          #  targets = [ "wasm32-unknown-unknown" ];
          #};
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
          sqlFilter = path: _type: builtins.match ".*sql$" path != null;
          cssFilter = path: _type: builtins.match ".*css$" path != null;
          ttfFilter = path: _type: builtins.match ".*ttf$" path != null;
          woff2Filter = path: _type: builtins.match ".*woff2$" path != null;
          webpFilter = path: _type: builtins.match ".*webp$" path != null;
          jpegFilter = path: _type: builtins.match ".*jpeg$" path != null;
          pngFilter = path: _type: builtins.match ".*png$" path != null;
          icoFilter = path: _type: builtins.match ".*ico$" path != null;
          pemFilter = path: _type: builtins.match ".*pem$" path !=null;
          soFilter = path: _type: builtins.match ".*so$" path !=null;
          protoOrCargo = path: type:
            (protoFilter path type) || (craneLib.filterCargoSources path type) || (sqlxFilter path type) || (sqlFilter path type) || (soFilter path type) || (pemFilter path type) ||(cssFilter path type) || (woff2Filter path type) || (ttfFilter path type) || (webpFilter path type) || (icoFilter path type) || (jpegFilter path type) || (pngFilter path type);
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
            cargo-leptos-git
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
          cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
            cargoExtraArgs = " --target x86_64-unknown-linux-gnu";
            doCheck = false;
             # Needed to enable build-std inside Crane
            cargoVendorDir = craneLib.vendorMultipleCargoDeps {
              inherit (craneLib.findCargoFiles src) cargoConfigs;
              cargoLockList = [
                ./Cargo.lock

                # Unfortunately this approach requires IFD (import-from-derivation)
                # otherwise Nix will refuse to read the Cargo.lock from our toolchain
                # (unless we build with `--impure`).
                #
                # Another way around this is to manually copy the rustlib `Cargo.lock`
                # to the repo and import it with `./path/to/rustlib/Cargo.lock` which
                # will avoid IFD entirely but will require manually keeping the file
                # up to date!
                "${rustTarget.passthru.availableComponents.rust-src}/lib/rustlib/src/rust/Cargo.lock"
              ];
            };
          });

          # Build the actual crate itself, reusing the dependency
          # artifacts from above.
          benwis_leptos = craneLib.buildPackage (commonArgs // {
            pname = "benwis_leptos";

          # Needed to enable build-std inside Crane
          cargoVendorDir = craneLib.vendorMultipleCargoDeps {
            inherit (craneLib.findCargoFiles src) cargoConfigs;
            cargoLockList = [
              ./Cargo.lock

              # Unfortunately this approach requires IFD (import-from-derivation)
              # otherwise Nix will refuse to read the Cargo.lock from our toolchain
              # (unless we build with `--impure`).
              #
              # Another way around this is to manually copy the rustlib `Cargo.lock`
              # to the repo and import it with `./path/to/rustlib/Cargo.lock` which
              # will avoid IFD entirely but will require manually keeping the file
              # up to date!
              "${rustTarget.passthru.availableComponents.rust-src}/lib/rustlib/src/rust/Cargo.lock"
            ];
          };

            buildPhaseCargoCommand = "cargo leptos build --release -vvv";
            installPhaseCommand = ''
            mkdir -p $out/bin
            cp target/server/x86_64-unknown-linux-gnu/release/benwis_leptos $out/bin/
            cp -r target/site $out/bin/
            '';
            # Prevent cargo test and nextest from duplicating tests
            doCheck = false;
            #cargoExtraArgs: "-Z build-std --target "
            inherit cargoArtifacts;
            # ALL CAPITAL derivations will get forwarded to mkDerivation and will set the env var during build
            SQLX_OFFLINE = "true";
            LEPTOS_BIN_TARGET_TRIPLE = "x86_64-unknown-linux-gnu"; # Adding this allows -Zbuild-std to work and shave 100kb off the WASM
            LEPTOS_BIN_PROFILE_RELEASE = "release";
            LEPTOS_LIB_PROFILE_RELEASE ="release-wasm-size";
          });
          
          cargo-leptos-git = pkgs.rustPlatform.buildRustPackage rec {
            pname = "cargo-leptos";
            version = "0.1.11";
            buildFeatures = ["no_downloads"]; # cargo-leptos will try to download Ruby and other things without this feature

            src = inputs.cargo-leptos-git;

            cargoSha256 = "sha256-9lKD9AZ5if/pxurtWNH0V+aUPs4pdEFIJIWbiGgz4cs=";

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
          flyConfig = ./fly.toml;

          # Deploy the image to Fly with our own bash script
          flyDeploy = pkgs.writeShellScriptBin "flyDeploy" ''
            OUT_PATH=$(nix build --print-out-paths .#container)
            HASH=$(echo $OUT_PATH | grep -Po "(?<=store\/)(.*?)(?=-)")
            ${pkgs.skopeo}/bin/skopeo --insecure-policy --debug copy docker-archive:"$OUT_PATH" docker://registry.fly.io/$FLY_PROJECT_NAME:$HASH --dest-creds x:"$FLY_AUTH_TOKEN" --format v2s2
            ${pkgs.flyctl}/bin/flyctl deploy -i registry.fly.io/$FLY_PROJECT_NAME:$HASH -c ${flyConfig} --remote-only --strategy immediate
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
            fromImage= pkgs.dockerTools.pullImage{
                imageName="ubuntu";
                imageDigest="sha256:266cbca633bcb1356b9be281efe251782d30830cee0c0cf6a3c952a3a655f040";
                sha256="sha256-hcOTa/pDj0wtaVjLY+2MpColSA8fOfifKN//ydS+CK8=";
            };
            #tag = "latest";
            created = "now";
            copyToRoot = pkgs.buildEnv {
              name="image-root";
              paths = [ pkgs.cacert
                (pkgs.writeShellScriptBin "runRust" ''
                    #!${pkgs.runtimeShell}
                    LD_PRELOAD=libbytehound.so ${benwis_leptos}/bin/benwis_leptos
                    '')
                    pkgs.bashInteractive
                    pkgs.heaptrack
                    pkgs.glibc_multi
                    pkgs.gdb
                    pkgs.coreutils
                    pkgs.dockerTools.binSh
                    pkgs.dockerTools.caCertificates
                    ./.
              ];
              pathsToLink = [ "/bin" "/db" "/migrations /bytehound" ];
            };
            config = {
              Env = [ "LD_LIBRARY_PATH=/bytehound" "MEMORY_PROFILER_LOG=warn" "MEMORY_PROFILER_OUTPUT=/bench/memory-profiling_%e_%t_%p.dat" "LEPTOS_ENVIRONMENT=prod_no_trace" "RUST_LOG=warn" "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt" "LEPTOS_OUTPUT_NAME=benwis_leptos" "LEPTOS_SITE_ADDR=0.0.0.0:3000" "LEPTOS_SITE_ROOT=${benwis_leptos}/bin/site" ];

              ExposedPorts = {
                "3000/tcp" = { };
              };
              runasRoot = ''
              #!${pkgs.runtimeShell}
              ${pkgs.dockerTools.shadowSetup}
              mkdir -p /bench
              cp /bytehound/libbytehound.so /lib/x86_64-linux-gnu/
              chmod u+s /lib/x86_64-linux-gnu/libbytehound.so
              '';

              Cmd = [ "runRust" ];
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
              dig
              sqlx-cli
              wasm-pack
              pkg-config
              binaryen
              flamegraph
              tokio-console
              oha
              nodejs
              hey
              drill
              nodePackages.tailwindcss
              heaptrack
              gdb
              cargo-leptos-git
              protobuf
              skopeo
              flyctl
            ];
            RUST_SRC_PATH = "${rustTarget}/lib/rustlib/src/rust/library";
          };
        });
}
