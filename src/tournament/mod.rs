use crate::{engine, shogi};
use chrono::{DateTime, Utc};

mod pgn_out_wrapper;
mod reporter_wrapper;
mod round_robin;

pub use pgn_out_wrapper::PgnOutWrapper;
pub use reporter_wrapper::ReporterWrapper;
pub use round_robin::RoundRobin;

#[derive(Debug, Clone)]
pub struct MatchTicket {
    pub id: u64,
    pub engines: [usize; 2],
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub ticket: MatchTicket,
    pub game_start: DateTime<Utc>,
    pub outcome: shogi::GameOutcome,
    pub moves: Vec<engine::MoveRecord>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TournamentState {
    Continue,
    Stop,
}

pub trait Tournament {
    fn next(&mut self) -> Option<MatchTicket>;
    fn match_complete(&mut self, result: MatchResult) -> TournamentState;
    fn expected_maximum_match_count(&self) -> Option<u64>;
}
