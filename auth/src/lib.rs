#![recursion_limit = "1024"]

use crate::pages::{Login, PersonsList};

use wasm_bindgen::prelude::*;
use yew::html;
use yew::prelude::*;

mod database;
mod pages;
mod person;
mod user;

enum Page {
    Login,
    PersonsList,
    OnePerson(Option<person::Person>),
}

struct AuthApp {
    page: Page,
    current_user: Option<user::User>,
    can_write: bool,
    db_conn: database::DbConn,
    link: ComponentLink<Self>,
}

enum Msg {
    LoggedIn(user::User),
    ChangeUser,
    GoToOnePersonPage(Option<person::Person>),
    GoToPersonsListPage,
}

impl Component for AuthApp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            page: Page::Login,
            current_user: None,
            can_write: false,
            db_conn: database::Database::new_thread_safe(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoggedIn(user) => {
                self.page = Page::PersonsList;
                self.current_user = Some(user.clone());
                self.can_write = user.privileges.contains(&user::DbPrivilege::CanWrite);
            },
            Msg::ChangeUser => self.page = Page::Login,
            Msg::GoToOnePersonPage(person) => self.page = Page::OnePerson(person),
            Msg::GoToPersonsListPage => self.page = Page::PersonsList,
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let go_to_page = self.link.callback(|u: user::User| {
            Msg::LoggedIn(u)
        });

        let go_to_one_person_page = self.link.callback(|p: Option<person::Person>| {
            Msg::GoToOnePersonPage(p)
        });

        html! {
            <div>
                <style>
                    {r#"
                        .current-user {
                            color: #0000C0;
                        }
                    "#}
                </style>
                <header>
                    <h2>{"People Management"}</h2>
                    <p>
                        {"Current User: "}
                        <span class="current-user">
                            {
                                if let Some(user) = &self.current_user {
                                    user.username.as_str()
                                } else {
                                    "---"
                                }
                            }
                        </span>
                        {
                            match self.page {
                                Page::Login => html! {
                                    <div />
                                },
                                _ => html! {
                                    <span>
                                        {""}
                                        <button onclick=&self.link.callback(|_| Msg::ChangeUser)>
                                            {"Change User"}
                                        </button>
                                    </span>
                                }
                            }
                        }
                    </p>
                    <hr />
                </header>
                {
                    match &self.page {
                        Page::Login => html! {
                            <Login
                                user=self.current_user.clone()
                                on_log_in=go_to_page.clone()
                                db_conn=Some(self.db_conn.clone())
                            />
                        },
                        Page::PersonsList => html! {
                            <PersonsList
                                can_write=true
                                go_to_one_person_page=go_to_one_person_page.clone()
                                db_conn=Some(self.db_conn.clone())
                            />
                        },
                        // Page::OnePerson(_) => html! {
                        //     <div>
                        //         <h1>{" One Person Page "}</h1>
                        //     </div>
                        // },
                        _ => html! { <div></div> }
                    }
                }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<AuthApp>::new().mount_to_body();
}
