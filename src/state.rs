use derive_more::Display;

#[derive(Display, Eq, PartialEq, Debug)]
pub enum State {
    #[display(fmt = "Waiting")]
    Waiting,
    #[display(fmt = "Waiting")]
    Fighting,
    #[display(fmt = "Rest...")]
    Resting,
    #[display(fmt = "Finished.")]
    Finished,
}
