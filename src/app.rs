use yew::{function_component, html};
use crate::boxing_timer::BoxingTimer;
use crate::boxing_rounds::BoxingRounds;

#[function_component(App)]
pub fn app() -> Html {
    web_sys::window()
        .and_then(|window| window.location().href().ok())
        .and_then(|href| web_sys::Url::new(&href).ok())
        .map(|url| {
            let search_params = url.search_params();
            let boxing_rounds = BoxingRounds::from_query(search_params);
            html! {
                <BoxingTimer ..boxing_rounds />
            }
        })
        .unwrap_or_else(|| html! { <BoxingTimer ..BoxingRounds::default() /> })
}
