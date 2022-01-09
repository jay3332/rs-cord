use crate::constants::DISCORD_CDN_URL;
use crate::ClientState;

use std::fmt::Display;
use std::path::Path;

pub const ALLOWED_FORMATS: [&str; 5] = [
    "png",
    "jpg",
    "jpeg",
    "webp",
    "gif",
];

pub const ALLOWED_SIZES: [u16; 9] = [
    16,
    32,
    64,
    128,
    256,
    512,
    1024,
    2048,
    4096,
];

/// Represents a Discord asset, such as an avatar or guild icon.
#[derive(Clone, Debug)]
pub struct Asset {
    state: ClientState,

    /// The URL path of this asset.
    pub path: String,

    /// The format of this asset.
    pub format: String,

    /// The format of this asset if it is not animated.
    /// 
    /// If this is none, the format will always be [`format`][`Asset::format`].
    pub static_format: Option<String>,

    /// Whether or not this asset is animated.
    pub animated: bool,

    /// The size of this asset.
    pub size: Option<u16>,
}

impl Asset {
    pub(crate) fn new(state: ClientState, path: String, animated: bool) -> Self {
        Self {
            state,
            path,
            format: String::from(if animated { "gif" } else { "png" }),
            static_format: None,
            animated,
            size: None,
        }
    }

    /// Returns the format being used.
    /// 
    /// If this is an animated asset, [`format`][`Asset::format`] will be used.  
    /// If this is a static asset, [`static_format`][`Asset::static_format`] will be used.
    #[must_use]
    pub fn format(&self) -> String {
        if self.animated || self.static_format.is_none() {
            self.format.clone()
        } else {
            self.static_format.clone().unwrap()  // we already checked that this is not none, so it is safe to unwrap.
        }
    }

    /// Returns the URL of this asset.
    #[must_use]
    pub fn url(&self) -> String {
        let size = if self.size.is_some() {
            format!("?size={}", self.size.unwrap())
        } else {
            "".to_string()
        };

        format!("{}/{}.{}{}", DISCORD_CDN_URL, self.path, self.format(), size)
    }

    /// Returns a clone of this asset with the given format.
    /// 
    /// # Panics
    /// - The format is not one of `png`, `jpg`, `jpeg`, `webp`, or `gif`.
    pub fn with_format(&self, format: impl Display) -> Self {
        let entity = format.to_string();
        
        if !ALLOWED_FORMATS.contains(&entity.as_str()) {
            panic!("The format must be one of `png`, `jpg`, `jpeg`, `webp`, or `gif`.");
        }
        
        let mut new = self.clone();
        new.format = entity;

        new
    }

    /// Returns a clone of this asset with the given static format.
    /// 
    /// # Panics
    /// - The format is not one of `png`, `jpg`, `jpeg`, `webp`, or `gif`.
    pub fn with_static_format(&self, format: impl Display) -> Self {
        let entity = format.to_string();
        
        if !ALLOWED_FORMATS.contains(&entity.as_str()) {
            panic!("The static format must be one of `png`, `jpg`, `jpeg`, `webp`, or `gif`.");
        }
        
        let mut new = self.clone();
        new.static_format = Some(entity);

        new
    }

    /// Returns a clone of this asset with the given size.
    /// 
    /// # Panics
    /// - The size is not a power of 2 between 16 and 4096.
    pub fn with_size(&self, size: u16) -> Self {
        if !ALLOWED_SIZES.contains(&size) {
            panic!("The size must be a power of 2 between 16 and 4096.");
        }
        
        let mut new = self.clone();
        new.size = Some(size);

        new
    }

    /// Fetches this asset from Discord's CDN and reads it's bytes.
    pub async fn read(&self) -> crate::ThreadSafeResult<Vec<u8>> {
        self.state.http.request_cdn(self.url()).await
    }

    /// Saves this asset locally to the given file path.
    pub async fn save(&self, path: impl AsRef<Path>) -> crate::ThreadSafeResult<()> {
        tokio::fs::write(path, self.read().await?.as_slice()).await.map_err(Into::into)
    }
}

impl Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.url().as_str())
    }
}

impl Into<String> for Asset {
    fn into(self) -> String {
        self.url()
    }
}
