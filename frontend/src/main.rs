use yew::prelude::*;

mod fetch;

use fetch::RawEvent;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    let games = RawEvent::fetch_this_weeks().await;
    html! {
        <div>
            <h2 class={"heading"}>{"Hello, World!"}</h2>
        </div>
    }
}
