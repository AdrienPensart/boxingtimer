use std::fmt;
use std::time::Duration;
use gloo::timers::callback::Interval;
use gloo::console::{log, Timer};
use yew::{html, Component, Context, Html};
use crate::state::State;
use crate::boxing_rounds::BoxingRounds;
use crate::boxing_bell::{BoxingBell};

pub enum Msg {
    Tick,
    Toggle,
}

pub struct BoxingTimer {
    boxing_rounds: BoxingRounds,
    paused: bool,
    round: u16,
    state: State,
    timeout: Duration,
    #[allow(dead_code)]
    tick: Interval,
    #[allow(dead_code)]
    console_timer: Timer<'static>,
}

impl fmt::Display for BoxingTimer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let seconds = self.timeout.as_secs();
        let (minutes, seconds_left) = (seconds / 60, seconds % 60);
        write!(f, "{}:{:02}", minutes, seconds_left)
    }
}

impl BoxingTimer {
    fn new(boxing_rounds: BoxingRounds, tick: Option<Interval>) -> Self {
        Self {
            round: 0,
            paused: false,
            state: State::Prepare,
            boxing_rounds,
            timeout: boxing_rounds.waiting,
            tick: tick.unwrap_or_else(|| Interval::new(boxing_rounds.interval.as_millis() as u32, || log!("Boxing timer not set yet"))),
            console_timer: Timer::new("Console Timer"),
        }
    }

    fn prepare_to_fight(&mut self) {
        self.timeout = self.boxing_rounds.fight;
        self.state = State::Fight;
        BoxingBell::play();
    }

    fn rest_to_fight(&mut self) {
        self.timeout = self.boxing_rounds.fight;
        self.state = State::Fight;
        BoxingBell::play();
    }

    fn fight_to_rest(&mut self) {
        self.timeout = self.boxing_rounds.rest;
        self.state = State::Rest;
        BoxingBell::play();
    }

    fn fight_to_finished(&mut self) {
        self.state = State::Finished;
        BoxingBell::play();
    }

    fn update(&mut self) {
        if self.paused {
            return
        }
        if !self.timeout.is_zero() {
            self.timeout = self.timeout.saturating_sub(Duration::from_secs(1));
            return
        }
        match self.state {
            State::Prepare => {
                self.round += 1;
                self.prepare_to_fight();
            }
            State::Rest => {
                self.round += 1;
                if self.round > self.boxing_rounds.rounds {
                    self.state = State::Finished;
                } else {
                    self.rest_to_fight();
                }
            }
            State::Fight => {
                if self.round == self.boxing_rounds.rounds {
                    self.fight_to_finished();
                } else {
                    self.fight_to_rest();
                }
            }
            State::Finished => {
                self.timeout = Duration::from_secs(0);
            }
        }
        if self.round > self.boxing_rounds.rounds {
            self.round = 0
        }
    }
}

impl Component for BoxingTimer {
    type Message = Msg;
    type Properties = BoxingRounds;

    fn create(ctx: &Context<Self>) -> Self {
        let tick = {
            let link = ctx.link().clone();
            let interval = ctx.props().interval.as_millis();
            Some(Interval::new(interval as u32, move || link.send_message(Msg::Tick)))
        };
        BoxingTimer::new(*ctx.props(), tick)
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.update();
                true
            }
            Msg::Toggle => {
                self.paused = !self.paused;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = &self.state.as_ref();
        html! {
            <>
                <BoxingBell />
                <button onclick={ctx.link().callback(|_| Msg::Toggle)} class="btn">
                    { if self.paused { "Start" } else { "Pause" }  }
                </button>
                <div class="centered">
                    <pre class="boxing_rounds">
                        { format!("{}/{}", &self.round, &self.boxing_rounds) }
                    </pre>
                    <div class={format!("state {}", state)}>
                        { &self.state }
                    </div>
                    <div class={format!("timer {}", state)}>
                        { &self }
                    </div>
                </div>
            </>
        }
    }
}
