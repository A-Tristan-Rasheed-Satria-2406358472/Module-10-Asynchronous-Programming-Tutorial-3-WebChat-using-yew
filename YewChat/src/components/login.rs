use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="w-screen min-h-screen bg-gradient-to-br from-slate-900 via-sky-900 to-cyan-800 text-white">
            <div class="container mx-auto px-6 py-16 flex flex-col justify-center items-center gap-8">
                <div class="text-center">
                    <h1 class="text-5xl font-black tracking-tight">{"YewChat"}</h1>
                    <p class="mt-3 text-cyan-100 max-w-xl">
                        {"A real-time broadcast chat built with Rust + Yew. Join the room, share ideas, and make the conversation alive."}
                    </p>
                </div>
                <div class="w-full max-w-2xl bg-white/10 backdrop-blur rounded-2xl p-6 shadow-2xl">
                    <div class="flex">
                        <input {oninput} class="w-full rounded-l-xl p-4 border-t border-b border-l text-gray-800 border-gray-200 bg-white" placeholder="Enter your username" />
                        <Link<Route> to={Route::Chat}>
                            <button type="button" {onclick} disabled={username.len()<1} class="px-8 rounded-r-xl bg-emerald-500 text-white font-bold p-4 uppercase border-emerald-500 border-t border-b border-r disabled:opacity-40" >
                                {"Go Chatting"}
                            </button>
                        </Link<Route>>
                    </div>
                    <div class="mt-4 flex items-center justify-between text-sm">
                        <span class="text-cyan-100">{"Tip: send a .gif URL in chat to render image preview."}</span>
                        <Link<Route> to={Route::Inspiration} classes="font-bold text-amber-300 hover:text-amber-200">
                            {"Open Inspiration Wall"}
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </div>
    }
}
