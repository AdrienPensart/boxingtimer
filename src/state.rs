use std::fmt;
use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug)]
pub enum State {
    #[strum(serialize = "prepare")]
    Prepare,
    #[strum(serialize = "fight")]
    Fight,
    #[strum(serialize = "rest")]
    Rest,
    #[strum(serialize = "finished")]
    Finished,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = self.as_ref();
        let mut chars = repr.chars();
        let capitalized = match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        };
        match self {
            Self::Prepare => write!(f, "{}.", capitalized),
            Self::Fight => write!(f, "{}!", capitalized),
            Self::Rest => write!(f, "{}...", capitalized),
            Self::Finished => write!(f, "{}.", capitalized),
        }
    }
}
