use crate::database::DbConn;
use crate::user::User;
use yew::prelude::*;
use yew::services::DialogService;
use yew::{html, Callback};

pub struct Login {
    user: Option<User>,
    on_log_in: Option<Callback<User>>,
    db_conn: DbConn,
    link: ComponentLink<Self>,
    state: State,
}

#[derive(Debug)]
pub enum Msg {
    UsernameChange(String),
    PasswordChange(String),
    OnLogin,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub user: Option<User>,
    pub on_log_in: Option<Callback<User>>,
    pub db_conn: Option<DbConn>,
}

struct State {
  username_field: String,
  password_field: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            user: None,
            on_log_in: None,
            db_conn: None,
        }
    }
}

impl Default for State {
  fn default() -> Self {
      Self {
        username_field: String::default(),
        password_field: String::default(),
      }
  }
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            user: props.user,
            on_log_in: props.on_log_in,
            db_conn: props.db_conn.unwrap(),
            state: State::default(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
        Msg::UsernameChange(new_username) => self.state.username_field = new_username,
        Msg::PasswordChange(new_password) => self.state.password_field = new_password,
        Msg::OnLogin => {
          if let Some(user) = self
            .db_conn
            .borrow()
            .get_user_by_username(&self.state.username_field) {
              if user.password == self.state.password_field {
                if let Some(ref got_to_page) = self.on_log_in {
                  got_to_page.emit(user.clone());
                }
              } else {
                DialogService::alert("Invalid password!");
              }
            } else {
              DialogService::alert("User not found");
            }
        }
      }

      true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.user = props.user;
        self.on_log_in = props.on_log_in;
        self.db_conn = props.db_conn.unwrap();

        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <label>{"Username: "}</label>
                    <input
                        type="text"
                        value=&self.state.username_field
                        oninput=self.link.callback(|e: InputData| Msg::UsernameChange(e.value))
                    />
                </div>
                <div>
                    <label>{"Password: "}</label>
                    <input
                        type="password"
                        value=&self.state.password_field
                        oninput=self.link.callback(|e: InputData| Msg::PasswordChange(e.value))
                    />
                </div>
                <button
                    onclick=self.link.callback(|_| Msg::OnLogin)>
                    { "Log in" }
                </button>
            </div>
        }
    }
}
