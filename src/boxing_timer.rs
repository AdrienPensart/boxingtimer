use std::fmt;
use std::time::Duration;
use gloo::timers::callback::Interval;
use gloo::console::{log, Timer};
use yew::{html, Component, Context, Html};
use crate::state::State;
use crate::boxing_rounds::{BoxingRounds, RenderedBoxingRounds};
use crate::boxing_bell::{BoxingBell};

pub enum Msg {
    Tick,
    Toggle,
    Reset,
}

#[derive(Debug)]
pub struct BoxingTimer {
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
        write!(f, "{minutes}:{seconds_left:02}")
    }
}

impl BoxingTimer {
    fn new(timeout: Duration, interval: Duration, tick: Option<Interval>) -> Self {
        Self {
            round: 0,
            paused: false,
            state: State::Waiting,
            timeout,
            tick: tick.unwrap_or_else(|| Interval::new(interval.as_millis() as u32, || log!("Boxing timer not set yet"))),
            console_timer: Timer::new("Console Timer"),
        }
    }

    fn reset(&mut self, wait: Duration) {
        log!("reseting timer");
        self.round = 0;
        self.timeout = wait;
        self.state = State::Waiting;
    }

    fn prepare_to_fight(&mut self, fight: Duration) {
        log!("prepare to fight !");
        self.timeout = fight;
        self.state = State::Fighting;
        BoxingBell::play();
    }

    fn rest_to_fight(&mut self, fight: Duration) {
        log!("rest to fight !");
        self.timeout = fight;
        self.state = State::Fighting;
        BoxingBell::play();
    }

    fn fight_to_rest(&mut self, rest: Duration) {
        log!("fight to rest !");
        self.timeout = rest;
        self.state = State::Resting;
        BoxingBell::play();
    }

    fn fight_to_finished(&mut self) {
        log!("fight to finish !");
        self.state = State::Finished;
        BoxingBell::play();
    }

    fn update(&mut self, boxing_rounds: &BoxingRounds) {
        if self.paused {
            return
        }
        if !self.timeout.is_zero() {
            self.timeout = self.timeout.saturating_sub(boxing_rounds.interval);
            return
        }
        match self.state {
            State::Waiting => {
                self.round += 1;
                self.prepare_to_fight(boxing_rounds.fight);
            }
            State::Resting => {
                self.round += 1;
                if self.round > boxing_rounds.rounds {
                    self.state = State::Finished;
                } else {
                    self.rest_to_fight(boxing_rounds.fight);
                }
            }
            State::Fighting => {
                if self.round == boxing_rounds.rounds {
                    self.fight_to_finished();
                } else {
                    self.fight_to_rest(boxing_rounds.rest);
                }
            }
            State::Finished => {
                self.timeout = Duration::from_secs(0);
            }
        }
        if self.round > boxing_rounds.rounds {
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
    type Properties = BoxingRounds;

    fn create(ctx: &Context<Self>) -> Self {
        let tick = {
            let link = ctx.link().clone();
            let interval = ctx.props().interval.as_millis();
            Some(Interval::new(interval as u32, move || link.send_message(Msg::Tick)))
        };
        let mut boxing_timer = BoxingTimer::new(
            ctx.props().wait,
            ctx.props().interval,
            tick
        );

        web_sys::window()
            .and_then(|window| window.location().href().ok())
            .and_then(|href| web_sys::Url::new(&href).ok())
            .map(|url| {
                let search_params = url.search_params();
                // let boxing_rounds = BoxingRounds::from_query(&search_params);

                let _ = search_params
                    .get("round")
                    .and_then(|round| round.parse::<u16>().ok())
                    .map(|round| boxing_timer.round = round);
            });
        boxing_timer
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.update(ctx.props());
                true
            },
            Msg::Toggle => {
                self.paused = !self.paused;
                true
            },
            Msg::Reset => {
                self.reset(ctx.props().wait);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = &self.state.as_ref();
        let round = &self.round;
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
                        <span class="fight">{ format!("{round}/") }</span>
                        <RenderedBoxingRounds ..*ctx.props() />
                    </li>
                    <li class={format!("state {state}")}>
                        { &self.state }
                    </li>
                    <li class={format!("timer {state}")}>
                        { &self }
                    </li>
                </ul>
            </>
        }
    }
}
