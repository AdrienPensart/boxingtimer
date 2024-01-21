use gloo::console::log;
use js_sys::Error;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use yew::{html, Component, Context, Html};

fn muted() -> Result<bool, Error> {
    Ok(audio()?.muted())
}

fn mute() -> Result<(), Error> {
    audio()?.set_muted(true);
    Ok(())
}

fn unmute() -> Result<(), Error> {
    audio()?.set_muted(false);
    Ok(())
}

fn audio() -> Result<web_sys::HtmlAudioElement, Error> {
    let window = web_sys::window().ok_or_else(|| Error::new("cannot get window"))?;
    let document = window
        .document()
        .ok_or_else(|| Error::new("cannot get document"))?;
    let bell = document
        .get_element_by_id("bell")
        .ok_or_else(|| Error::new("cannot get bell element"))?;
    bell.dyn_into::<web_sys::HtmlAudioElement>()
        .map_err(|_| Error::new("cannot convert to audio element"))
}

fn play() -> Result<(), Error> {
    if !muted()? {
        let promise = audio()?.play()?;
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        spawn_local(async move {
            let _ = future.await.map_err(|e| log!("failed to await future", e));
        });
    }
    Ok(())
}

pub enum BellMsg {
    Play,
    Toggle,
}

pub struct BoxingBell;

impl BoxingBell {
    fn toggle_text() -> String {
        match muted() {
            Err(_) | Ok(true) => "Unmute".to_string(),
            Ok(false) => "Mute".to_string(),
        }
    }

    fn toggle() {
        match muted() {
            Ok(true) => Self::unmute(),
            Ok(false) => Self::mute(),
            Err(e) => {
                log!("unable to mute or unmute bell", e.message());
            }
        };
    }

    fn mute() {
        if mute().is_err() {
            log!("Unable to mute bell");
        } else {
            log!("Bell muted");
        }
    }

    fn unmute() {
        if unmute().is_err() {
            log!("Unable to unmute bell");
        } else {
            log!("Bell unmuted");
        }
    }

    pub fn play() {
        if play().is_err() {
            log!("Unable to play bell");
        }
    }
}

impl Component for BoxingBell {
    type Message = BellMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BellMsg::Toggle => {
                Self::toggle();
                true
            }
            BellMsg::Play => {
                Self::play();
                true
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            Self::mute();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <audio id="bell" src="static/bell.mp3" preload="auto" autoplay=false />
                <button onclick={ctx.link().callback(|_| BellMsg::Toggle)} class="btn">
                    { Self::toggle_text() }
                </button>
                <button onclick={ctx.link().callback(|_| BellMsg::Play)} class="btn">
                    { "BELL" }
                </button>
            </>
        }
    }
}
