#[derive(serde_derive::Deserialize)]
pub struct Category {
    pub name: String,
    sprawl: bool,
    aliases: Vec<String>
}

#[derive(serde_derive::Deserialize)]
pub struct Feed {
    category: String,
    url: String,
    pub name: String,
    aliases: Vec<String>
}

pub async fn display_category_feeds(category: &Category) {
    // ...
}

pub async fn display_feed(feed: &Feed) {
    let content = reqwest::get(&feed.url)
        .await.unwrap()
        .text()
        .await.unwrap();

    println!("{:?}", &content);

    // ....
}
