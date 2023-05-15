use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component(Home)]
pub fn home() -> Html {



    html! {
        <>

        <div class="px-[10%] mt-10 text-dark text-2xl">
        <div class="px-4">
            <h1 class="title font-semibold text-4xl mb-4">{"Psypolus"}</h1>
            <p>
                {"The home page contains a list of surveys, that you can use to go to each landing
                page."}
            </p>
        </div>
        <div class="mt-10 px-4">
            <h2 class="text-3xl mb-4">{"Surveys"}</h2>
            <ul>
                <li>
                    <Link<Route>
                        to={ Route::AboutBigFive } 
                        classes="text-md font-semibold leading-6
                        ">{"Big Five Personality Test"}
                    </Link<Route>>
                </li>
            </ul>
        </div>
    </div>
    
    

        </>
    }
}