use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! { 
        <>
            <h1>{"Uh oh!"}</h1> 
            <p class="mt-3">
            {"There doesn't seem to be anything here D:"}
            </p>
        </>
    }
}