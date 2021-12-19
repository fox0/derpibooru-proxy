use serde::{Deserialize, Serialize};

/// Прямые ссылки на CDN
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct Representation {
    pub full: String,
    pub large: String,
    pub medium: String,
    pub small: String,
    pub tall: String,
    pub thumb: String,
    pub thumb_small: String,
    pub thumb_tiny: String,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct Image {
    pub id: u32,
    pub faves: u32,
    pub score: i32,
    pub upvotes: u32,
    pub downvotes: u32,
    pub view_url: String, //todo Option?
    pub representations: Representation,
    pub width: u32,
    pub height: u32,
    pub tags: Vec<String>,
    // pub source_url: Option<String>,
    // pub size: u32,
    // pub uploader: String,
    // wilson_score
    // pub tag_ids: Vec<u32>,
    // pub tag_count: u32,
    // pub intensities
    // pub sha512_hash: String,
}

impl Image {
    pub fn get_title(&self) -> String {
        format!(
            "Size: {}x{} | Tagged: {}",
            self.width,
            self.height,
            self.tags.join(" ")
        )
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct SearchImages {
    pub images: Vec<Image>,
    pub total: u32,
    // pub interactions: Vec<>
    // image_id	11
    // interaction_type	"voted"
    // user_id	311
    // value	"up"
}
