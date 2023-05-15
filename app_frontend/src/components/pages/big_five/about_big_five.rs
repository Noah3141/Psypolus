use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::inc::survey_section_bar::SurveyNav;
use super::big_five_sections::BigFiveSections;

#[function_component(AboutBigFive)]
pub fn about_big_five() -> Html {



    html! {
        <>
        <div id="container" class="px-[10%] mt-10 text-dark text-2xl">
        
        <div class="px-4">
            <h1 class="title font-semibold text-4xl mb-4">{"The Big Five Personality Scale"}</h1>
            <p>
                {"Here we have minimal explanations of the scale. Arranged uniquely to the scale in question."}
            </p>
        <p>{"Short snippets and right left right photos"}</p>

        </div>
        </div>


        <SurveyNav />
        <BigFiveSections />
    
        </>

    }
}