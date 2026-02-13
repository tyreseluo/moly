use anyhow::Result;
use cargo_packager_updater::{Config, Update, check_update, semver::Version};
use makepad_widgets::*;
use url::Url;

const MOLY_UPDATER_PUBKEY: &str = "<MOLY_PUBKEY>";

#[derive(Clone, Debug, DefaultNone)]
pub enum UpdaterAction {
    Checking,
    NoUpdate,
    UpdateAvailable(Update),
    Failed(String),
    None,
}

/// Checks GitHub Releases for an available update without blocking the UI thread.
///
/// This queries the `latest` release endpoint using target/arch-specific
/// manifests uploaded by the release workflow (e.g. `latest-windows-x86_64.json`).
/// When finished, it posts a [`UpdaterAction`] into the Makepad event queue so the UI
/// can react without the startup path blocking on network I/O.
pub async fn check_moly_update() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION").parse::<Version>()?;
    let config = Config {
        endpoints: vec![Url::parse("https://github.com/tyreseluo/moly/releases/latest/download/latest-{{target}}-{{arch}}.json").expect("Failed to parse URL")],
        pubkey: MOLY_UPDATER_PUBKEY.into(),
        ..Default::default()
    };

    // Start checking for updates
    Cx::post_action(UpdaterAction::Checking);

    let update_result = tokio::task::spawn_blocking(move || check_update(current_version, config))
        .await?;

    match update_result {
        Ok(Some(update)) => Cx::post_action(UpdaterAction::UpdateAvailable(update)),
        Ok(None) => Cx::post_action(UpdaterAction::NoUpdate),
        Err(err) => Cx::post_action(UpdaterAction::Failed(err.to_string()))
    }

    Ok(())
}
