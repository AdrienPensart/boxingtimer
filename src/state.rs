use std::fmt;
use strum_macros::AsRefStr;

#[derive(AsRefStr, Eq, PartialEq, Debug)]
pub enum State {
    #[strum(serialize = "wait")]
    Waiting,
    #[strum(serialize = "fight")]
    Fighting,
    #[strum(serialize = "rest")]
    Resting,
    #[strum(serialize = "finished")]
    Finished,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = self.as_ref();
        let mut chars = repr.chars();
        let capitalized = chars.next().map_or_else(String::new, |f| {
            f.to_uppercase().collect::<String>() + chars.as_str()
        });
        match self {
            Self::Waiting => write!(f, "{capitalized}"),
            Self::Fighting => write!(f, "{capitalized}!"),
            Self::Resting => write!(f, "{capitalized}..."),
            Self::Finished => write!(f, "{capitalized}."),
        }
    }
}
