use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::*;

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, Route, User};

pub enum Msg {
    HandleMsg(String),
    UpdateInput(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: String,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: String::new(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        true
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        true
                    }
                    _ => false,
                }
            }
            Msg::SubmitMessage => {
                let trimmed = self.chat_input.trim().to_string();
                if !trimmed.is_empty() {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(trimmed),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    self.chat_input.clear();
                    return true;
                };
                false
            }
            Msg::UpdateInput(next) => {
                self.chat_input = next;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateInput(input.value())
        });
        let onkeydown = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                Some(Msg::SubmitMessage)
            } else {
                None
            }
        });

        html! {
            <div class="flex w-screen bg-gradient-to-br from-slate-900 via-sky-900 to-cyan-800 text-slate-100">
                <div class="flex-none w-64 h-screen bg-slate-900/60 border-r border-slate-700 backdrop-blur-sm">
                    <div class="text-xl p-4 font-black text-cyan-100">{"Online Users"}</div>
                    {
                        self.users.iter().map(|u| {
                            html! {
                                <div class="flex m-3 bg-white/10 rounded-xl p-2 border border-slate-600">
                                    <div>
                                        <img class="w-12 h-12 rounded-full" src={u.avatar.clone()} alt="avatar"/>
                                    </div>
                                    <div class="flex-grow p-3">
                                        <div class="flex text-xs justify-between">
                                            <div class="font-semibold text-slate-100">{u.name.clone()}</div>
                                        </div>
                                        <div class="text-xs text-slate-300">
                                            {"Ready to brainstorm"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>

                <div class="grow h-screen flex flex-col">
                    <div class="w-full h-16 border-b border-slate-700 bg-slate-900/50 flex justify-between items-center px-5 backdrop-blur-sm">
                        <div>
                            <div class="text-xl font-black text-cyan-100">{"Broadcast Chat Room"}</div>
                            <div class="text-xs text-slate-300">{"One message, everyone receives it."}</div>
                        </div>
                        <Link<Route> to={Route::Inspiration} classes="px-3 py-2 rounded-lg bg-cyan-300 text-slate-900 font-bold text-sm">
                            {"Inspiration Wall"}
                        </Link<Route>>
                    </div>

                    <div class="w-full grow overflow-auto border-b border-slate-700 bg-slate-950/20">
                        {
                            self.messages.iter().map(|m| {
                                let avatar = self
                                    .users
                                    .iter()
                                    .find(|u| u.name == m.from)
                                    .map(|u| u.avatar.clone())
                                    .unwrap_or_else(|| "https://avatars.dicebear.com/api/adventurer-neutral/default.svg".into());

                                html! {
                                    <div class="flex items-end w-3/6 bg-white/90 text-slate-800 m-6 rounded-tl-xl rounded-tr-xl rounded-br-xl border border-slate-200 shadow-sm">
                                        <img class="w-8 h-8 rounded-full m-3" src={avatar} alt="avatar"/>
                                        <div class="p-3">
                                            <div class="text-sm font-semibold text-slate-700">{m.from.clone()}</div>
                                            <div class="text-xs text-slate-600">
                                                if m.message.ends_with(".gif") {
                                                    <img class="mt-3 rounded-lg max-h-56" src={m.message.clone()}/>
                                                } else {
                                                    {m.message.clone()}
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>

                    <div class="w-full h-16 flex px-3 items-center bg-slate-900/50 border-t border-slate-700">
                        <input
                            type="text"
                            placeholder="Type your message..."
                            class="block w-full py-2 pl-4 mx-3 bg-white/90 text-slate-800 rounded-full outline-none border border-slate-200"
                            name="message"
                            required=true
                            value={self.chat_input.clone()}
                            {oninput}
                            {onkeydown}
                        />
                        <button onclick={submit} class="p-3 shadow-sm bg-emerald-500 w-10 h-10 rounded-full flex justify-center items-center color-white">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
