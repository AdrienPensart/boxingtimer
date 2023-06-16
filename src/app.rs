use yew::{function_component, html};
use crate::app::html::Html;
use crate::boxing_timer::BoxingTimer;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BoxingTimer />
    }
}
