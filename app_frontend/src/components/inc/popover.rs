use yew::prelude::*;

#[function_component(Popover)]
pub fn popover() -> Html {



    html! {

        <div class="button group">
        <span class="tooltip-right group-hover:scale-100">{"Hover here"}</span>
        </div>


    }
}