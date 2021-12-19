use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

/// Прямые ссылки на CDN
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
#[derive(Deserialize)]
pub struct Image {
    pub id: u32,
    pub faves: u32,
    pub score: i32,
    pub upvotes: u32,
    pub downvotes: u32,
    pub representations: Representation,
    pub mime_type: String,
    #[serde(skip_deserializing)]
    pub title: Option<String>,

    // поля ниже не используются в шаблоне
    pub width: u32,
    pub height: u32,
    pub view_url: String, //todo Option?
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
    fn get_title(&self) -> String {
        format!(
            "Size: {}x{} | Tagged: {}",
            self.width,
            self.height,
            self.tags.join(" ")
        )
    }
}

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // the number of fields in the struct.
        let mut s = serializer.serialize_struct("Image", 8)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("faves", &self.faves)?;
        s.serialize_field("score", &self.score)?;
        s.serialize_field("upvotes", &self.upvotes)?;
        s.serialize_field("downvotes", &self.downvotes)?;
        s.serialize_field("representations", &self.representations)?;
        s.serialize_field("mime_type", &self.mime_type)?;
        // из-за этой строчки пришлось реализовывать трейт
        s.serialize_field("title", &self.get_title())?;
        s.end()
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
