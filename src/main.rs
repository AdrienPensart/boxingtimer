mod app;
pub mod state;
pub mod boxing_timer;
pub mod boxing_bell;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
