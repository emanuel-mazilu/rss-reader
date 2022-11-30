use colored::*;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use phf::phf_map;
use rss::Channel;
use std::error::Error;

static SOURCES: phf::Map<&'static str, &'static str> = phf_map! {
    "TVR" => "http://stiri.tvr.ro/rss/stiri.xml",
    "MediaFax" => "https://www.mediafax.ro/rss",
};

#[derive(Debug)]
struct News {
    title: String,
    description: String,
    link: String,
}

fn read_rss(url: String) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::blocking::get(url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn display_one_news(news: &News) -> Result<(), Box<dyn Error>> {
    println!("{}", news.title.red().bold());
    println!("{}", news.description.italic().yellow());
    println!("{}", news.link.blue().italic());
    println!("");
    Ok(())
}

fn display_menu(items: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    term.write_line("Select your news feed: ").unwrap();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    term.clear_screen().unwrap();
    match selection {
        Some(index) => {
            println!("{}", items[index].on_white().black().bold());
            Ok(items[index].to_string())
        }
        None => {
            println!("User did not select anything");
            Ok("None".to_string())
        }
    }
}

fn store_news(channel: Channel) -> Result<Vec<News>, Box<dyn Error>> {
    let stiri = channel.into_items();
    let mut news: Vec<News> = Vec::new();
    for stire in stiri {
        let title = stire.title().unwrap();
        let description = stire.description();
        let description = match description {
            Some(description) => description,
            None => "No description",
        };
        let link = stire.link();
        let link = match link {
            Some(link) => link,
            None => "No link for this news",
        };
        news.push(News {
            title: title.to_string(),
            description: description.to_string(),
            link: link.to_string(),
        })
    }
    Ok(news)
}

fn main() {
    let mut items = Vec::new();
    for site in &SOURCES {
        items.push(site.0.to_owned())
    }
    let url = display_menu(items).unwrap();
    let site_url = SOURCES.get(&url[..]).unwrap();
    let xml = read_rss(site_url.to_string()).unwrap();
    let news = store_news(xml).unwrap();
    for stire in news {
        display_one_news(&stire).unwrap();
    }
}
