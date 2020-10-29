#![recursion_limit = "256"]

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use yew::html;
use yew::prelude::*;
use yew::InputData;

struct State {
    addend1: f64,
    addend2: f64,
    total: f64,
}

enum Page {
    Login,
    PersonsList,
    OnePerson(Option<Person>),
}

struct AuthApp {
    page: Page,
    current_user: Option<String>,
    can_write: bool,
    db_conn: Rc<RefCell<DbConn>>,
}

enum Msg {
    ChangeInputValue(String, String),
    Sum,
}

impl Component for AuthApp {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State {
                addend1: 0_f64,
                addend2: 0_f64,
                total: 0_f64,
            },
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeInputValue(input_name, value) => {
                if let Ok(next_value) = value.parse::<f64>() {
                    if input_name == "addend1" {
                        self.state.addend1 = next_value;
                    } else {
                        self.state.addend2 = next_value;
                    }
                }

                true
            }
            Msg::Sum => {
                self.state.total = self.state.addend1 + self.state.addend2;

                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <table>
                <tr>
                    <td>{"Addend 1:"}</td>
                    <td>
                        <input
                            type="number"
                            style="text-align: right;"
                            oninput=self.link.callback(|e: InputData| Msg::ChangeInputValue(String::from("addend1"), e.value))
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Addend 2:"}</td>
                    <td>
                        <input
                            type="number"
                            style="text-align: right;"
                            oninput=self.link.callback(|e: InputData| Msg::ChangeInputValue(String::from("addend2"), e.value))
                        />
                    </td>
                </tr>
                <tr>
                    <td></td>
                    <td align="center">
                        <button
                            onclick=self.link.callback(|_| Msg::Sum)
                        >
                            {"Add"}
                        </button>
                    </td>
                </tr>
                <tr>
                    <td>{"Total: "}</td>
                    <td>
                        <input
                            type="number"
                            style="text-align: right;"
                            value=self.state.total
                        />
                    </td>
                </tr>
            </table>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<AuthApp>::new().mount_to_body();
}
