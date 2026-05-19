use yew::prelude::*;
use yew_router::prelude::*;

use crate::{Route, User};

#[function_component(Creative)]
pub fn creative() -> Html {
    let user = use_context::<User>().expect("No context found.");
    let has_username = !user.username.borrow().trim().is_empty()
        && user.username.borrow().as_str() != "initial";
    let back_route = if has_username {
        Route::Chat
    } else {
        Route::Login
    };

    html! {
        <div class="w-full min-h-screen bg-slate-100 p-8">
            <div class="max-w-5xl mx-auto min-h-[80vh] flex flex-col">
                <div class="flex justify-between items-center mb-8">
                    <h1 class="text-3xl font-black text-slate-800">{"Inspiration Wall"}</h1>
                    <Link<Route> to={back_route} classes="px-4 py-2 rounded-lg bg-slate-800 text-white font-semibold">
                        {"Back"}
                    </Link<Route>>
                </div>

                <div class="grow flex items-center justify-center">
                    <blockquote class="max-w-3xl text-center bg-white rounded-2xl shadow-lg p-10 border border-slate-200">
                        <p class="text-3xl font-black text-slate-800 leading-relaxed">
                            {"\"Code can work, but great products make people feel understood.\""}
                        </p>
                    </blockquote>
                </div>
            </div>
        </div>
    }
}
