use crate::config::CONFIG;

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

const PER_PAGE: u32 = 40;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Serialize)]
pub struct Pagination {
    current_page: u32,
    last_page: u32,
    total: u32,
}

impl Pagination {
    pub fn new(current_page: u32, total: u32) -> Self {
        Self {
            current_page,
            last_page: total / PER_PAGE + 1,
            total,
        }
    }
}

/// The current sort field
#[allow(non_camel_case_types)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Serialize)]
pub enum SortField {
    wilson_score,
    //todo
}

impl SortField {
    // todo derive
    pub const fn get_choices() -> [(&'static str, &'static str); 17] {
        [
            ("id", "Sort by image ID"),
            ("updated_at", "Sort by last modification date"),
            ("first_seen_at", "Sort by initial post date"),
            ("aspect_ratio", "Sort by aspect ratio"),
            ("faves", "Sort by fave count"),
            ("upvotes", "Sort by upvotes"),
            ("downvotes", "Sort by downvotes"),
            ("score", "Sort by score"),
            ("wilson_score", "Sort by Wilson score"),
            ("_score", "Sort by relevance"),
            ("width", "Sort by width"),
            ("height", "Sort by height"),
            ("comment_count", "Sort by comments"),
            ("tag_count", "Sort by tag count"),
            ("pixels", "Sort by pixels"),
            ("size", "Sort by file size"),
            ("duration", "Sort by duration"),
            // ("random:324483087", "Random!"),
        ]
    }
}

/// The current sort direction
#[allow(non_camel_case_types)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Serialize)]
pub enum SortDirection {
    /// Ascending
    asc,
    /// Descending
    desc,
}

impl SortDirection {
    // todo derive
    pub const fn get_choices() -> [(&'static str, &'static str); 2] {
        [("desc", "Descending"), ("asc", "Ascending")]
    }
}

/// This is a list of general parameters that are useful when working with the API.
/// Not all parameters may be used in every request.
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Serialize)]
pub struct Parameters {
    /// An optional authentication token.
    /// If omitted, no user will be authenticated.
    pub key: String,
    /// The current search query, if the request is a search request.
    pub q: Option<String>,
    /// Controls the current page of the response, if the response is paginated.
    /// Empty values default to the first page.
    pub page: Option<u32>,
    /// Controls the number of results per page, up to a limit of 50, if the response is paginated.
    /// The default is 25.
    pub per_page: Option<u32>,
    /// The current sort field, if the request is a search request.
    pub sf: Option<String>, //todo enum
    /// The current sort direction, if the request is a search request.
    pub sd: Option<String>, //todo enum
}

impl Parameters {
    pub fn new(
        page: Option<u32>,
        q: Option<String>,
        sf: Option<String>,
        sd: Option<String>,
    ) -> Self {
        Self {
            key: CONFIG.api_key.clone(),
            q: Option::from(q.unwrap_or_else(|| "*".to_string())),
            page: Option::from(page.unwrap_or(1)),
            per_page: Some(PER_PAGE),
            sf,
            sd,
        }
    }

    pub fn new_only_key() -> Self {
        Self {
            key: CONFIG.api_key.clone(),
            q: None,
            page: None,
            per_page: None,
            sf: None,
            sd: None,
        }
    }
}

/// Прямые ссылки на CDN
/// A mapping of representation names to their respective URLs.
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
    // незадокументированные поля
    pub mp4: Option<String>,
    pub webm: Option<String>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Deserialize)]
pub struct Image {
    /// The image's ID.
    pub id: u32,
    /// The number of faves the image has.
    pub faves: u32,
    /// The image's number of upvotes minus the image's number of downvotes.
    pub score: i32,
    /// The image's number of upvotes.
    pub upvotes: u32,
    /// The number of downvotes the image has.
    pub downvotes: u32,
    /// A mapping of representation names to their respective URLs.
    pub representations: Representation,
    /// The MIME type of this image.
    /// One of "image/gif", "image/jpeg", "image/png", "image/svg+xml", "video/webm".
    pub mime_type: String,

    /// The image's height, in pixels.
    #[serde(skip_serializing)]
    pub height: u32,
    /// The image's width, in pixels.
    #[serde(skip_serializing)]
    pub width: u32,
    /// The image's view URL, including tags.
    #[serde(skip_serializing)]
    pub view_url: String, //todo Option?
    /// A list of tag names the image contains.
    #[serde(skip_serializing)]
    pub tags: Vec<String>,
    /// The current source URL of the image.
    #[serde(skip_serializing)]
    pub source_url: Option<String>,
    // pub size: u32,
    // pub uploader: String,
    // wilson_score
    // pub tag_ids: Vec<u32>,
    // pub tag_count: u32,
    // pub intensities
    // pub sha512_hash: String,
    #[serde(skip_deserializing)]
    pub title: Option<String>,
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
