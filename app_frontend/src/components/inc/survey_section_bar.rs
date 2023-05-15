use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component(SurveyNav)]
pub fn survey_section_bar() -> Html {



    html! {

    <div class="h-12 py-2 block bg-light-700 border-t-2 border-light-400">
        <div class="tab-header">{"About"}</div>
        <div class="tab-header">{"Data"}</div>
        <div class="tab-header">{"Research"}</div>
        <div class="tab-header">{"Take the test"}</div>
    </div>


    }
}