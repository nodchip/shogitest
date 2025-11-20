use crate::{shogi, tournament, tournament::Tournament};

pub struct ReporterWrapper {
    inner: Box<dyn tournament::Tournament>,
    engine_names: Vec<String>,
}

impl ReporterWrapper {
    pub fn new(
        inner: Box<dyn tournament::Tournament>,
        engine_names: Vec<String>,
    ) -> ReporterWrapper {
        ReporterWrapper {
            inner: inner,
            engine_names: engine_names,
        }
    }
}

impl ReporterWrapper {
    fn format_of_max_string(&self) -> String {
        match self.expected_maximum_match_count() {
            Some(count) => format!(" of {count}"),
            None => String::from(""),
        }
    }
}

impl tournament::Tournament for ReporterWrapper {
    fn next(&mut self) -> Option<tournament::MatchTicket> {
        let ticket = self.inner.as_mut().next();
        if let Some(ticket) = &ticket {
            println!(
                "Started game {}{} ({} vs {})",
                ticket.id + 1,
                self.format_of_max_string(),
                &self.engine_names[ticket.engines[0]],
                &self.engine_names[ticket.engines[1]]
            );
        }
        ticket
    }
    fn match_complete(&mut self, result: tournament::MatchResult) -> tournament::TournamentState {
        let ticket = &result.ticket;
        println!(
            "Finished game {} ({} vs {}): {} {{{}}}",
            ticket.id + 1,
            &self.engine_names[ticket.engines[0]],
            &self.engine_names[ticket.engines[1]],
            match result.outcome.winner() {
                Some(shogi::Color::Sente) => "1-0",
                Some(shogi::Color::Gote) => "0-1",
                None => "1/2-1/2",
            },
            result.outcome.to_string(),
        );
        self.inner.as_mut().match_complete(result)
    }
    fn expected_maximum_match_count(&self) -> Option<u64> {
        self.inner.as_ref().expected_maximum_match_count()
    }
}
