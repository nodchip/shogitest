use crate::{cli, pgn, tournament};

pub struct PgnOutWrapper {
    inner: Box<dyn tournament::Tournament>,
    pgn: pgn::PgnWriter,
}

impl PgnOutWrapper {
    pub fn new(
        inner: Box<dyn tournament::Tournament>,
        options: &cli::PgnOutOptions,
        meta: &cli::MetaDataOptions,
        engine_names: Vec<String>,
    ) -> Result<PgnOutWrapper, std::io::Error> {
        Ok(PgnOutWrapper {
            inner: inner,
            pgn: pgn::PgnWriter::new(&options, &meta, engine_names)?,
        })
    }
}

impl tournament::Tournament for PgnOutWrapper {
    fn next(&mut self) -> Option<tournament::MatchTicket> {
        self.inner.as_mut().next()
    }
    fn match_complete(&mut self, result: tournament::MatchResult) -> tournament::TournamentState {
        self.pgn.write(&result).unwrap();
        self.inner.as_mut().match_complete(result)
    }
    fn expected_maximum_match_count(&self) -> Option<u64> {
        self.inner.as_ref().expected_maximum_match_count()
    }
}
