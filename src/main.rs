use std::process::exit;
mod feeds;

#[derive(serde_derive::Deserialize)]
struct FConfig {
    categories: Vec<feeds::Category>,
    feeds: Vec<feeds::Feed>
}

fn main() {
    let cfg = read_config();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please choose a particular category.");
        return;
    }

    let key_name = args.get(1).unwrap();

    // First check if the entered data was a category
    for cat in &cfg.categories {
        if &cat.name == key_name {
            feeds::display_category_feeds(&cat);
            return;
        }
    }

    // Search for feeds if there was not a matching category
    for feed in &cfg.feeds {
        if &feed.name == key_name {
            feeds::display_feed(&feed);
            return;
        }
    }

    println!("No category or feed named: {}, was found.", key_name);
}

fn read_config() -> FConfig {
    let contents = match std::fs::read_to_string("Feeds.toml") {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Failed to reed the Feeds.toml file.");
            exit(1);
        }
    };

    let data: FConfig = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Could not load any data from the Feeds.toml file.");
            exit(1);
        }
    };

    return data;
}
