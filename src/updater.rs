//! Minimal updater helper using CrabNebula's `cargo-packager-updater`.
//! Replace `UPDATER_PUBKEY` with the minisign public key that matches the
//! private key configured in the release pipeline.

use anyhow::Result;
use cargo_packager_updater::{check_update, Config};

/// Minisign public key that verifies update signatures.
/// TODO: replace with the real public key (e.g. `RWQ...`).
// const UPDATER_PUBKEY: &str = "REPLACE_WITH_MINISIGN_PUBLIC_KEY";
const UPDATER_PUBKEY: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEFGOTlFM0UzMTZCNUQzMTUKUldRVjA3VVc0K09acnhYRFU3My9wK1BuV2ZOeElNT1M4RWIvb1ZDSktrTHU5bkJmMUthZkdKUWoK";

/// Checks GitHub Releases for an available update and prints the result.
///
/// This queries the `latest` release endpoint using target/arch-specific
/// manifests uploaded by the release workflow (e.g. `latest-windows-x86_64.json`).
pub fn check_for_update_and_print() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION").parse()?;
    let config = Config {
        endpoints: vec![ "https://github.com/moly-ai/moly/releases/latest/download/latest-{{target}}-{{arch}}.json".parse()? ],
        pubkey: UPDATER_PUBKEY.into(),
        ..Default::default()
    };

    match check_update(current_version, config)? {
        Some(update) => {
            println!("Update available: {}", update.version);
        }
        None => {
            println!("No update available");
        }
    }
    // Test output
    Ok(())
}
