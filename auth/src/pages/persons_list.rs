use std::collections::HashSet;
use yew::prelude::*;
use yew::services::DialogService;

use crate::database::DbConn;
use crate::person::Person;

pub struct PersonsList {
    id_to_find: Option<u32>,
    name_portion: String,
    filtered_persons: Vec<Person>,
    selected_ids: HashSet<u32>,
    can_write: bool,
    go_to_one_person_page: Option<Callback<Option<Person>>>,
    db_conn: DbConn,
    link: ComponentLink<Self>,
}

#[derive(Debug)]
pub enum Msg {
    IdChanged(String),
    Find,
    ChangePartialName(String),
    Filter,
    Delete,
    Add,
    Edit(u32),
    ToggleSelect(u32),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub can_write: bool,
    pub go_to_one_person_page: Option<Callback<Option<Person>>>,
    pub db_conn: Option<DbConn>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            can_write: false,
            go_to_one_person_page: None,
            db_conn: None,
        }
    }
}

impl Component for PersonsList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = PersonsList {
            id_to_find: None,
            name_portion: String::default(),
            filtered_persons: Vec::<Person>::new(),
            selected_ids: HashSet::<u32>::new(),
            can_write: props.can_write,
            go_to_one_person_page: props.go_to_one_person_page,
            db_conn: props.db_conn.unwrap(),
            link,
        };

        model.filtered_persons = model
            .db_conn
            .borrow()
            .get_persons_by_name("")
            .map(|p| p.to_owned())
            .collect();

        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::IdChanged(id) => self.id_to_find = id.parse::<u32>().ok(),
            Msg::Find => match self.id_to_find {
                Some(id) => {
                    self.update(Msg::Edit(id));
                }
                None => {
                    DialogService::alert("No ID provided");
                }
            },
            Msg::ChangePartialName(value) => self.name_portion = value,
            Msg::Filter => {
                self.filtered_persons = self
                    .db_conn
                    .borrow()
                    .get_persons_by_name(self.name_portion.as_str())
                    .map(|p| p.to_owned())
                    .collect();
            }
            Msg::Delete => {
                if DialogService::confirm("Are you sure you want to delete the entries selected?") {
                    {
                        let mut db = self.db_conn.borrow_mut();
    
                        for id in &self.selected_ids {
                            db.delete(*id);
                        }
                    }
                        
                    self.update(Msg::Filter);
                    DialogService::alert("Entries deleted successfully");
                }
            }
            Msg::Add => {
                if let Some(ref go_to_page) = self.go_to_one_person_page {
                    go_to_page.emit(None);
                }
            }
            Msg::ToggleSelect(id) => {
                if self.selected_ids.contains(&id) {
                    self.selected_ids.remove(&id);
                } else {
                    self.selected_ids.insert(id);
                }
            }
            Msg::Edit(id) => match self.db_conn.borrow().get_person_by_id(id) {
                Some(person) => {
                    if let Some(ref go_to_page) = self.go_to_one_person_page {
                        go_to_page.emit(Some(person.clone()));
                    }
                }
                None => DialogService::alert(&format!("No entry with ID: {} found", id)),
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.can_write = props.can_write;
        self.go_to_one_person_page = props.go_to_one_person_page;
        self.db_conn = props.db_conn.unwrap();
        self.filtered_persons = self
            .db_conn
            .borrow()
            .get_persons_by_name("")
            .map(|p| p.to_owned())
            .collect();

        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <label>{"Id: "}</label>
                    <input
                        type="number"
                        oninput=self.link.callback(|e: InputData| Msg::IdChanged(e.value))
                    />
                    <button onclick=self.link.callback(|_| Msg::Find)>
                        {"Find"}
                    </button>
                </div>
                <div>
                    <label>{"Name portion: "}</label>
                    <input
                        type="text"
                        oninput=self.link.callback(|e: InputData| Msg::ChangePartialName(e.value))
                    />
                    <button onclick=self.link.callback(|_| Msg::Filter)>
                        {"Filter"}
                    </button>
                </div>
                <button
                    disabled=!self.can_write
                    onclick=self.link.callback(|_| Msg::Delete)
                >
                    {"Delete Selection"}
                </button>
                <button
                    disabled=!self.can_write
                    onclick=self.link.callback(|_| Msg::Add)
                >
                    {"Add New"}
                </button>
                {
                    if self.filtered_persons.is_empty() {
                        html! {
                            <p>{"No results"}</p>
                        }
                    } else {
                        html! {
                            <table>
                                <thead>
                                    <th></th>
                                    <th></th>
                                    <th>{"ID"}</th>
                                    <th>{"Name"}</th>
                                </thead>
                                <tbody>
                                    {
                                        for self.filtered_persons.iter().map(|person| {
                                            let id = person.id;
                                            let name = person.name.clone();
                                            let on_toggle_select = self.link.callback(move |_| Msg::ToggleSelect(id));
                                            let on_edit = self.link.callback(move |_| Msg::Edit(id));

                                            html! {
                                                <tr>
                                                    <td>
                                                        <input
                                                            type="checkbox"
                                                            oninput=on_toggle_select
                                                            checked=self.selected_ids.contains(&id)
                                                        />
                                                    </td>
                                                    <td>
                                                        <button onclick=on_edit>
                                                            {"Edit"}
                                                        </button>
                                                    </td>
                                                    <td>{id}</td>
                                                    <td>{name}</td>
                                                </tr>
                                            }
                                        })
                                    }
                                </tbody>
                            </table>
                        }
                    }
                }
            </div>
        }
    }
}
