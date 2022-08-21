use select::node::Node;
use select::predicate::Name;
use itertools::Itertools;
use std::{fmt, fmt::{Debug, Display, Formatter, Result as FmtResult}};
use std::error::Error;

pub struct Match {
    p1: String,
    p2: String,
    p1_scores: Vec<String>,
    p2_scores: Vec<String>,

    // 1 for p1 win, 2 for p2 win, 3 for p1 serving, 4 for p2 serving
    state: u8,
}

pub enum MatchParseError {
    PlayerParseError,
    ScoreParseError,
    StateParseError,
    ScoreFormatError,
}

impl MatchParseError {
    fn message(&self) -> &str {
        match self {
            MatchParseError::PlayerParseError => "Error parsing player names",
            MatchParseError::ScoreParseError => "Error parsing scores",
            MatchParseError::StateParseError => "Error parsing match state",
            MatchParseError::ScoreFormatError => "Error formatting score",
        }
    }
}

impl Display for MatchParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for MatchParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for MatchParseError {}

impl Match {
    fn format_score(&self) -> Result<String, MatchParseError> {
        let mut ret_str = String::new();

        let mut iter_1 = self.p1_scores.iter();
        let mut iter_2 = self.p2_scores.iter();

        if self.state == 2 || self.state == 4 {
            iter_1 = self.p2_scores.iter();
            iter_2 = self.p1_scores.iter();
        }

        while let Some(s1) = iter_1.next() {
            if let Some(s2) = iter_2.next() {
                if s1 == "7" && &s2[0..1] == "6" && s2.len() > 1 {
                    ret_str += format!(" 7-6({})", &s2[2..]).as_str();
                } else if s2 == "7" && &s1[0..1] == "6" && s1.len() > 1 {
                    ret_str += format!(" 6({})-7", &s1[2..]).as_str();
                } else {
                    let s1 = if s1 != "-" { s1 } else { "0" };
                    let s2 = if s2 != "-" { s2 } else { "0" };
                    ret_str += format!(" {}-{}", s1, s2).as_str();
                }
            } else {
                return Err(MatchParseError::ScoreFormatError);
            }
        }
        if let Some(_) = iter_2.next() {
            return Err(MatchParseError::ScoreFormatError);
        }

        Ok(ret_str)
    }
}

impl TryFrom<Node<'_>> for Match {
    type Error = MatchParseError;

    fn try_from(match_node: Node) -> Result<Match, Self::Error> {
        let p1;
        let p2;
        let p1_scores;
        let p2_scores;
        let mut winner_name: Option<&str> = None;
        let mut server_name: Option<&str> = None;
        let state;

        match match_node
            .find(Name("a"))
            .filter_map(|n| n.attr("title"))
            .map(|x| x.to_string())
            .collect_tuple() {
                Some(tup) => (p1, p2) = tup,
                None => return Err(MatchParseError::PlayerParseError),
            }

        if let Some(winner_node) = match_node
            .find(Name("small"))
            .filter(|n| n.text() == "Winner")
            .map(|n| n.parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap())
            .next() {
                winner_name = Some(winner_node.find(Name("a"))
                    .find_map(|n| n.attr("title"))
                    .ok_or_else(|| MatchParseError::StateParseError)?);
            }

        if let Some(winner_node) = match_node
            .find(Name("small"))
            .filter(|n| n.text() == "Service")
            .map(|n| n.parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap())
            .next() {
                server_name = Some(winner_node.find(Name("a"))
                    .find_map(|n| n.attr("title")).unwrap());
            }

        match match_node
            .find(Name("div"))
            .filter(|n| n.attr("class").unwrap_or("") == "tc-match__stats")
            .map(|n| n.children()
                .filter(|n| n.attr("class")
                        .unwrap_or("")
                        .contains("--set")
                )
                .map(|n| n.text().trim().to_string()).collect::<Vec<String>>())
            .collect_tuple() {
                Some(tup) => (p1_scores, p2_scores) = tup,
                None => return Err(MatchParseError::ScoreParseError),
            }

        if winner_name == Some(&p1) {
            state = 1;
        } else if winner_name == Some(&p2) {
            state = 2;
        } else if server_name == Some(&p1) {
            state = 3;
        } else if server_name == Some(&p2) {
            state = 4;
        } else {
            return Err(MatchParseError::StateParseError);
        }

        Ok(Match {
            p1,
            p2,
            p1_scores,
            p2_scores,
            state,
        })
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let fmt_str = match self.state {
            1 => format!("{} d. {}", self.p1, self.p2),
            2 => format!("{} d. {}", self.p2, self.p1),
            3 => format!("LIVE: *{} v. {}", self.p1, self.p2),
            4 => format!("LIVE: *{} v. {}", self.p2, self.p1),
            _ => return Err(fmt::Error),
        };
        
        write!(f, "{} {}", fmt_str, self.format_score().map_err(|_| fmt::Error)?)
    }
}