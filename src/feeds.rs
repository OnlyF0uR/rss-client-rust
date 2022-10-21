#[derive(Debug, serde_derive::Deserialize)]
pub struct Category {
    pub name: String,
    sprawl: bool,
    aliases: Vec<String>
}

#[derive(Debug, serde_derive::Deserialize)]
pub struct Feed {
    category: String,
    url: String,
    pub name: String
}

pub fn display_category_feeds(category: &Category) {

}

pub fn display_feed(feed: &Feed) {

}
