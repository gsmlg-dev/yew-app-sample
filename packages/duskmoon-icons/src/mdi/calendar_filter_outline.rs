#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CalendarFilterOutline)]
pub fn r#icon_calendar_filter_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 3H18V1H16V3H8V1H6V3H4.75C4.31 3.07 3.9 3.27 3.58 3.58C3.27 3.9 3.07 4.31 3 4.75V19.25C3.07 19.69 3.27 20.1 3.58 20.42C3.9 20.73 4.31 20.93 4.75 21H15V19H5V9H19V11H21V5C21 3.89 20.11 3 19 3M19 7H5V5H19V7M17 21L18.8 22.77C19.3 23.27 20 22.87 20 22.28V18L22.8 14.6C23.3 13.9 22.8 13 22 13H15C14.2 13 13.7 14 14.2 14.6L17 18V21" />
    </svg>
  }
}