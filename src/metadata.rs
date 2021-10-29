use dbus::arg::{RefArg, Variant};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    pub trackid: String,
    pub length: u64,
    pub art_url: String,
    pub album: String,
    pub album_artist: Vec<String>,
    pub artist: Vec<String>,
    pub auto_rating: f64,
    pub disc_number: i32,
    pub title: String,
    pub track_number: i32,
    pub url: String,
}

impl Metadata {
    pub fn to_map(&self) -> HashMap<String, Variant<Box<dyn RefArg + 'static>>> {
        let mut metadata = HashMap::new();
        metadata.insert(
            "mpris:trackid".to_string(),
            Variant(Box::new(self.trackid.clone()) as Box<dyn RefArg>),
        );
        metadata.insert(
            "mpris:length".to_string(),
            Variant(Box::new(self.length.clone()) as Box<dyn RefArg>),
        );
        metadata.insert(
            "mpris:artUrl".to_string(),
            Variant(Box::new(self.art_url.clone()) as Box<dyn RefArg>),
        );
        metadata.insert(
            "xesam:album".to_string(),
            Variant(Box::new(self.album.clone()) as Box<dyn RefArg>),
        );
        metadata.insert(
            "xesam:albumArtist".to_string(),
            Variant(Box::new(self.album_artist.clone()) as Box<dyn RefArg>),
        );
        metadata.insert(
            "xesam:artist".to_string(),
            Variant(Box::new(self.artist.clone()) as Box<dyn RefArg>),
        );
        metadata.insert(
            "xesam:autoRating".to_string(),
            Variant(Box::new(self.auto_rating.clone()) as Box<dyn RefArg>),
        );
        metadata.insert(
            "xesam:discNumber".to_string(),
            Variant(Box::new(self.disc_number.clone()) as Box<dyn RefArg>),
        );
        metadata.insert(
            "xesam:title".to_string(),
            Variant(Box::new(self.title.clone()) as Box<dyn RefArg>),
        );
        metadata.insert(
            "xesam:trackNumber".to_string(),
            Variant(Box::new(self.track_number.clone()) as Box<dyn RefArg>),
        );
        metadata.insert(
            "xesam:url".to_string(),
            Variant(Box::new(self.url.clone()) as Box<dyn RefArg>),
        );
        return metadata;
    }
}
