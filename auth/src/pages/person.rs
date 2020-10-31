use crate::database::DbConn;

use yew::prelude::*;

pub struct Person {
    id: Option<u32>,
    can_write: bool,
    go_to_persons_list: Option<Callback<()>>,
    db_conn: DbConn,
    state: State,
    link: ComponentLink<Self>,
}

struct State {
    is_inserting: bool,
    name_value: String,
}

pub enum Msg {
    ChangeName(String),
    Save,
    Cancel,
}

#[derive(Debug, Clone, Properties)]
pub struct PersonProps {
    pub id: Option<u32>,
    pub name: String,
    pub can_write: bool,
    pub go_to_persons_list: Option<Callback<()>>,
    pub db_conn: Option<DbConn>,
}

impl Default for PersonProps {
    fn default() -> Self {
        Self {
            id: None,
            name: String::default(),
            can_write: false,
            go_to_persons_list: None,
            db_conn: None,
        }
    }
}

impl Component for Person {
    type Message = Msg;
    type Properties = PersonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            id: props.id,
            can_write: props.can_write,
            go_to_persons_list: props.go_to_persons_list,
            db_conn: props.db_conn.unwrap(),
            link,
            state: State {
                is_inserting: props.id.is_none(),
                name_value: props.name,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeName(value) => self.state.name_value = value,
            Msg::Save => {
                if self.state.is_inserting {
                    self.db_conn.borrow_mut().insert(&self.state.name_value);
                } else {
                    self.db_conn
                        .borrow_mut()
                        .update(self.id.unwrap(), &self.state.name_value);
                }

                if let Some(ref go_to_page) = self.go_to_persons_list {
                    go_to_page.emit(());
                }
            }
            Msg::Cancel => {
                if let Some(ref go_to_page) = self.go_to_persons_list {
                    go_to_page.emit(());
                }
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.id = props.id;
        self.state.name_value = props.name;
        self.can_write = props.can_write;
        self.state.is_inserting = props.id.is_none();
        self.go_to_persons_list = props.go_to_persons_list;
        self.db_conn = props.db_conn.unwrap();

        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <label>{"Id: "}</label>
                    <input
                        type="number"
                        value=match self.id { Some(id) => format!("{}", id), _ => String::default() }
                        disabled=true
                    />
                </div>
                <div>
                    <label>{"Name: "}</label>
                    <input
                        type="text"
                        value=&self.state.name_value
                        disabled=!self.can_write
                        oninput=self.link.callback(|e: InputData| Msg::ChangeName(e.value))
                    />
                </div>
                <div>
                    <button
                        onclick=self.link.callback(|_| Msg::Save)
                        disabled=!self.can_write
                    >
                        {
                            if self.state.is_inserting {
                                "Insert"
                            } else {
                                "Unpdate"
                            }
                        }
                    </button>
                    <button
                        onclick=self.link.callback(|_| Msg::Cancel)
                        disabled=!self.can_write
                    >
                        {"Cancel"}
                    </button>
                </div>
            </div>
        }
    }
}
