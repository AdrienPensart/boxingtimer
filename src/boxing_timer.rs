extern crate lenient_bool;
use std::fmt;
use std::time::Duration;
use std::str::FromStr;
use gloo::timers::callback::Interval;
use gloo::console::{log, Timer};
use yew::{html, Component, Context, Html};
use lenient_bool::LenientBool;
use crate::state::State;
use crate::boxing_bell::BoxingBell;

pub enum Msg {
    Tick,
    Toggle,
    Reset,
}

fn get_param_or<T: FromStr>(param: &str, default: T) -> T {
    web_sys::window()
        .and_then(|window| window.location().href().ok())
        .and_then(|href| web_sys::Url::new(&href).ok())
        .and_then(|url| url.search_params().get(param))
        .and_then(|param| param.parse::<T>().ok())
        .unwrap_or(default)
}

#[derive(Debug)]
pub struct BoxingTimer {
    /// Current round
    round: u64,
    /// Number of rounds
    rounds: u64,
    /// We let some time to prepare
    wait: Duration,
    /// Duration of a round
    fight: Duration,
    /// Duration of rest between each round
    rest: Duration,
    /// Is the timer running ?
    paused: bool,
    /// Current state of training
    state: State,
    /// Interval between each tick (in milliseconds)
    interval: Duration,
    /// Left seconds in current round
    timeout: Duration,
    #[allow(dead_code)]
    /// Interval betweeen each timer tick
    tick: Interval,
    #[allow(dead_code)]
    /// Internal timer
    console_timer: Timer<'static>,
}

impl fmt::Display for BoxingTimer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let seconds = self.timeout.as_secs();
        let (minutes, seconds_left) = (seconds / 60, seconds % 60);
        write!(f, "{minutes}:{seconds_left:02}")
    }
}

impl BoxingTimer {
    fn reset(&mut self) {
        log!("reseting timer");
        self.round = 0;
        self.timeout = self.wait;
        self.state = State::Waiting;
    }

    fn prepare_to_fight(&mut self) {
        log!("prepare to fight !");
        self.timeout = self.fight;
        self.state = State::Fighting;
        BoxingBell::play();
    }

    fn rest_to_fight(&mut self) {
        log!("rest to fight !");
        self.timeout = self.fight;
        self.state = State::Fighting;
        BoxingBell::play();
    }

    fn fight_to_rest(&mut self) {
        log!("fight to rest !");
        self.timeout = self.rest;
        self.state = State::Resting;
        BoxingBell::play();
    }

    fn fight_to_finished(&mut self) {
        log!("fight to finish !");
        self.state = State::Finished;
        BoxingBell::play();
    }

    fn update(&mut self) {
        if self.paused {
            return
        }
        if !self.timeout.is_zero() {
            self.timeout = self.timeout.saturating_sub(self.interval);
            return
        }
        match self.state {
            State::Waiting => {
                self.round += 1;
                self.prepare_to_fight();
            }
            State::Resting => {
                self.round += 1;
                if self.round > self.rounds {
                    self.state = State::Finished;
                } else {
                    self.rest_to_fight();
                }
            }
            State::Fighting => {
                if self.round == self.rounds {
                    self.fight_to_finished();
                } else {
                    self.fight_to_rest();
                }
            }
            State::Finished => {
                self.timeout = Duration::from_secs(0);
            }
        }
        if self.round > self.rounds {
            self.round = 0
        }
    }

    fn toggle_text(&self) -> String {
        if self.paused {
            "Start".to_string()
        } else {
            "Pause".to_string()
        }
    }
}

impl Component for BoxingTimer {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();

        let round = get_param_or("round", 0);
        let rounds = get_param_or("rounds", 12);
        let wait = get_param_or("wait", 5);
        let fight = get_param_or("fight", 180);
        let rest = get_param_or("rest", 60);
        let interval = get_param_or("interval", 1000);
        let paused = get_param_or::<LenientBool>("paused", LenientBool(false));

        BoxingTimer {
            round,
            rounds,
            wait: Duration::from_secs(wait),
            fight: Duration::from_secs(fight),
            rest: Duration::from_secs(rest),
            paused: paused.into(),
            state: State::Waiting,
            interval: Duration::from_millis(interval),
            timeout: Duration::from_secs(wait),
            tick: Interval::new(interval as u32, move || link.send_message(Msg::Tick)),
            console_timer: Timer::new("Console Timer"),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.update();
                true
            },
            Msg::Toggle => {
                self.paused = !self.paused;
                true
            },
            Msg::Reset => {
                self.reset();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = &self.state.as_ref();
        let round = &self.round;
        let rounds = self.rounds;
        let wait = self.wait.as_secs();
        let fight = self.fight.as_secs();
        let rest = self.rest.as_secs();
        html! {
            <>
                <div class="controls">
                    <BoxingBell />
                    <button onclick={ctx.link().callback(|_| Msg::Toggle)} class="btn">
                        { self.toggle_text()  }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::Reset)} class="btn">
                        { "Reset" }
                    </button>
                </div>
                <ul class="centered">
                    <li class="boxing_rounds">
                        <span class="fight">
                            { format!("{round}/{rounds} rounds ({fight}s)") }
                        </span>
                        <br />
                        <span class="wait">
                            { format!("Wait ({wait}s) ") }
                        </span>
                        <span class="rest">
                            { format!("Rest ({rest}s)") }
                        </span>
                    </li>
                    <li class={format!("state {state}")}>
                        { format!("{}", self.state) }
                    </li>
                    <li class={format!("timer {state}")}>
                        { format!("{}", self) }
                    </li>
                </ul>
            </>
        }
    }
}
