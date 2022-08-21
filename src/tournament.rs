use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::error::Error;
use select::document::Document;
use select::predicate::Name;

use crate::tmatch::Match;

pub struct Tournament {
    pub name: String,
    pub doc: Document,
}

pub enum TournamentParseError {
    NameParseError,
    MatchesParseError,
}

impl TournamentParseError {
    fn message(&self) -> &str {
        match self {
            TournamentParseError::NameParseError => "Error parsing tournament name",
            TournamentParseError::MatchesParseError => "Error parsing tournament match nodes",
        }
    }
}

impl Display for TournamentParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for TournamentParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for TournamentParseError {}

impl TryFrom<Document> for Tournament {
    type Error = TournamentParseError;

    fn try_from(doc: Document) -> Result<Tournament, Self::Error> {
        let name = doc.find(Name("span"))
            .filter(|n| n.attr("class") == Some("tc-tournament-header__name"))
            .map(|n| n.text())
            .next().ok_or(TournamentParseError::NameParseError)?;

        Ok(Tournament { name: name.to_string(), doc })
    }
}

impl Tournament {

    pub fn top_n_matches(&self, n: usize) -> Vec<Result<Match, TournamentParseError>> {
        let mut matches: Vec<Result<Match, TournamentParseError>> = vec![];

        let matches_iter = self.doc
            .find(Name("div"))
            .filter(|n| n.attr("class")
                .unwrap_or("")
                .contains("tc-match__content"));

        for (i, match_node) in matches_iter.enumerate() {
            if i >= n { break }

            if let Ok(tmatch) = Match::try_from(match_node) {
                matches.push(Ok(tmatch));
            } else {
                matches.push(Err(TournamentParseError::MatchesParseError));
            }
        }
        matches
    }
}