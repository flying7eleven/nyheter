use lazy_static::lazy_static;
use regex::Regex;
use rss::{Category, Channel};
use std::fmt::{Debug, Error, Formatter};
use std::str::FromStr;

struct NyheterCategory {
    raw_name: String,
}

struct NyheterNewsEntry {
    title: String,
    description: String,
    categories: Vec<NyheterCategory>,
}

impl NyheterNewsEntry {
    pub fn get_short_description(&self) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new("<[^>]*(>|$)").unwrap();
        }
        let mut short_description = self.description.clone();

        // remove linebreaks, HTML and then trim the text
        short_description = short_description.replace("\n", "");
        short_description = RE.replace(short_description.as_str(), "").to_string();
        short_description = short_description.to_owned().trim().to_string();

        // ensure the description is not more than 100 characters
        if short_description.chars().count() >= 100 {
            short_description = short_description
                .chars()
                .into_iter()
                .take(100 - 3)
                .collect();
            return format!("{}...", short_description);
        }
        short_description
    }
}

impl FromStr for NyheterCategory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NyheterCategory {
            raw_name: s.to_string(),
        })
    }
}

impl Debug for NyheterCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.raw_name.as_str())
    }
}

impl Debug for NyheterNewsEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!(
            "{} :: {} :: {:?}",
            self.title,
            self.get_short_description(),
            self.categories
        ))
    }
}

fn get_enhanced_categories(input: &[Category]) -> Vec<NyheterCategory> {
    let mut category_list: Vec<NyheterCategory> = Vec::new();
    for category in input {
        category_list.push(NyheterCategory::from_str(category.name()).unwrap())
    }
    category_list
}

fn main() {
    let input_feeds = vec![
        "https://www.aerotelegraph.com/feed".to_string(),
        "https://feeds2.feedburner.com/stadt-bremerhaven/dqXM".to_string(),
        "https://rp-online.de/nrw/staedte/duesseldorf/feed.rss".to_string(),
        "https://www.tagesschau.de/xml/rss2".to_string(),
    ];

    for feed in input_feeds {
        let foo = match Channel::from_url(feed.as_str()) {
            Ok(channel) => channel,
            Err(err) => panic!(
                "Could not parse the RSS feed ({}). The error was: {}",
                feed, err
            ),
        };

        for item in foo.items() {
            let categories = get_enhanced_categories(item.categories());
            let new_entry = NyheterNewsEntry {
                title: item.title().unwrap().to_string(),
                description: item.description().unwrap().to_string(),
                categories,
            };
            println!("{:?}", new_entry);
        }
    }
}
