// use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::TypeScriptDefinition;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// An instance of a minecraft server
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize)]
pub struct MCServer {
    /// The name for the server
    pub name: String,
    /// The executable for the server
    pub jar: ServerJar
}

/// A distrobution of minecraft server to use
#[cfg_attr(target_arch = "wasm32", derive(TypeScriptDefinition))]
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
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize)]
pub struct ServerJar {
    /// The type of executible to use
    pub distro: ServerDistrobution,
    /// The version of minecraft to use
    pub version: String
}

/// The url for getting official mc versions
static MC_VERSIONS_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug)]
pub struct VersionListing {
    pub latest: Latest,
    pub versions: Vec<Version>
}

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Latest {
    pub release: String,
    pub snapshot: String
}

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub id: String,
    #[serde(alias = "type")] 
    pub release_type: ReleaseType,
    pub url: String,
    pub time: String,
    #[serde(alias = "releaseTime")] 
    pub release_time: String
}

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
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

// CACHE
// pub fn get_versions(snapshots: bool, old: bool) -> Result<VersionListing, Box<dyn std::error::Error>> {
//     let listing: VersionListing = reqwest::get(MC_VERSIONS_URL)?.json()?;

//     Ok(VersionListing {
//         latest: listing.latest,
//         versions: listing.versions.into_iter().filter(|x| match x.release_type {
//             ReleaseType::OldAlpha | ReleaseType::OldBeta if old => true,
//             ReleaseType::Snapshot if snapshots => true,
//             ReleaseType::Release => true,
//             _ => false
//         }).collect::<Vec<Version>>()
//     })
// }