use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// An instance of a minecraft server
#[derive(Serialize, Deserialize)]
pub struct MCServer {
    /// The path to the server
    path: PathBuf,
    /// The name for the server
    name: String,
    /// The executable for the server
    jar: ServerJar
}

/// A distrobution of minecraft server to use
#[derive(Serialize, Deserialize)]
pub enum ServerDistrobution {
    Vanilla,
    // Spigot,
    // Bukkit,
    // Forge,
    // Paper,
    // ...
}

/// Information about the server distro to use
#[derive(Serialize, Deserialize)]
pub struct ServerJar {
    /// The type of executible to use
    distro: ServerDistrobution,
    /// The version of minecraft to use
    version: String
}

/// The url for getting official mc versions
static MC_VERSIONS_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionListing {
    latest: Latest,
    versions: Vec<Version>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Latest {
    release: String,
    snapshot: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    id: String,
    #[serde(alias = "type")] 
    release_type: ReleaseType,
    url: String,
    time: String,
    #[serde(alias = "releaseTime")] 
    release_time: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ReleaseType {
    #[serde(alias = "release")] 
    Release,
    #[serde(alias = "snapshot")]
    Snapshot,
    #[serde(alias = "old_alpha")]
    OldAlpha,
    #[serde(alias = "old_beta")]
    OldBeta
}

pub fn get_versions(snapshots: bool, old: bool) -> Result<VersionListing, Box<dyn std::error::Error>> {
    let listing: VersionListing = reqwest::get(MC_VERSIONS_URL)?.json()?;

    Ok(VersionListing {
        latest: listing.latest,
        versions: listing.versions.into_iter().filter(|x| match x.release_type {
            ReleaseType::OldAlpha | ReleaseType::OldBeta if old => true,
            ReleaseType::Snapshot if snapshots => true,
            ReleaseType::Release => true,
            _ => false
        }).collect::<Vec<Version>>()
    })
}