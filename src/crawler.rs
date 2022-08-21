use std::io::Read;
use select::document::Document;
use select::predicate::Name;
use reqwest::blocking::Client;

use crate::tournament::Tournament;

pub struct Crawler<'a> {
    client: Client,
    origin_url: &'a str,
}

impl<'a> Crawler<'a> {
    pub fn new(origin_url: &'a str) -> Self {
        Self {
            client: Client::new(),
            origin_url,
        }
    }

    pub fn get_tournaments(&self) -> Vec<Tournament> {
        let mut tournaments: Vec<Tournament> = vec![];

        for link in self.get_tournament_links_iter() {
            let mut res = self.client.get(&link).send().unwrap();
            let mut body = String::new();
            res.read_to_string(&mut body).expect("Unable to read request");

            let doc = Document::from(body.as_str());

            if let Ok(tournament) = Tournament::try_from(doc) {
                tournaments.push(tournament);
            }
        }

        tournaments
    }

    fn get_tournament_links_iter(&self) -> Vec<String> {
        let mut res = self.client.get(self.origin_url).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).expect("Unable to read request");
        
        Document::from(body.as_str())
            .find(Name("a"))
            .filter(|n| n.last_child()
                .unwrap()
                .text() == "Results"
            )
            .filter_map(|n| n.attr("href"))
            .map(|x| String::from("https://www.tennis.com") + x)
            .collect()
    }
}