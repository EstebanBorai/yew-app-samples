use wasm_bindgen::prelude::*;
use yew::html;
use yew::prelude::*;

struct Counter {
    value: u64,
    link: ComponentLink<Self>,
}

enum Msg {
    Increment,
    Reset,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { value: 0, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                true
            }
            Msg::Reset => {
                self.value = 0;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Increment)>{"Increment"}</button>
                <button onclick=self.link.callback(|_| Msg::Reset)>{"Reset"}</button>
                <input
                    readonly=true
                    value={self.value}
                />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Counter>::new().mount_to_body();
}
