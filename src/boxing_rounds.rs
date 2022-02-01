use std::fmt;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use yew::Properties;
use web_sys::UrlSearchParams;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Properties)]
pub struct BoxingRounds {
    /// We let some time to prepare
    pub waiting: Duration,
    /// Duration of a round
    pub fight: Duration,
    /// Duration of rest between each round
    pub rest: Duration,
    /// Interval between each tick (in milliseconds)
    pub interval: Duration,
    /// Number of rounds
    pub rounds: u16,
}

impl BoxingRounds {
    pub fn from_query(params: UrlSearchParams) -> Self {
        let mut boxing_rounds = BoxingRounds::default();
        let _ = params
            .get("waiting")
            .and_then(|waiting| waiting.parse::<u64>().ok())
            .map(|waiting| boxing_rounds.waiting = Duration::from_secs(waiting));

        let _ = params
            .get("fight")
            .and_then(|fight| fight.parse::<u64>().ok())
            .map(|fight| boxing_rounds.fight = Duration::from_secs(fight));

        let _ = params
            .get("rest")
            .and_then(|rest| rest.parse::<u64>().ok())
            .map(|rest| boxing_rounds.rest = Duration::from_secs(rest));

        let _ = params
            .get("interval")
            .and_then(|interval| interval.parse::<u64>().ok())
            .map(|interval| boxing_rounds.interval = Duration::from_millis(interval));

        let _ = params
            .get("rounds")
            .and_then(|rounds| rounds.parse::<u16>().ok())
            .map(|rounds| boxing_rounds.rounds = rounds);
        boxing_rounds
    }
}

impl Default for BoxingRounds {
    fn default() -> Self {
        Self {
            waiting: Duration::from_secs(5),
            fight: Duration::from_secs(180),
            rest: Duration::from_secs(60),
            interval: Duration::from_millis(1000),
            rounds: 12,
        }
    }
}

impl fmt::Display for BoxingRounds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let waiting = self.waiting.as_secs();
        let fight = self.fight.as_secs();
        let rest = self.rest.as_secs();
        write!(f, "{} rounds ({}s) \nPrepare({}s) Rest({}s)", self.rounds, fight, waiting, rest)
    }
}

