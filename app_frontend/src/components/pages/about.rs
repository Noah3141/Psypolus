use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(About)]
pub fn about() -> Html {



    html! {

        <div class="container">

            <h1>{"About"}</h1>
            <p>{"This is the about page."}</p>
        </div>


    }
}