#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_AsteriskCircleOutline)]
pub fn r#icon_asterisk_circle_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 2C6.5 2 2 6.5 2 12C2 17.5 6.5 22 12 22C17.5 22 22 17.5 22 12C22 6.5 17.5 2 12 2M12 20C7.61 20 4 16.39 4 12C4 7.61 7.61 4 12 4C16.39 4 20 7.61 20 12C20 16.39 16.39 20 12 20M13 10.27L15.83 8.63L16.83 10.37L14 12L16.83 13.63L15.83 15.37L13 13.73V17H11V13.73L8.17 15.37L7.17 13.63L10 12L7.17 10.37L8.17 8.63L11 10.27V7H13V10.27Z" />
    </svg>
  }
}