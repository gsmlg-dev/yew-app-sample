#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_MedalOutline)]
pub fn r#icon_medal_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14.94 19.5L12 17.77L9.06 19.5L9.84 16.16L7.25 13.92L10.66 13.63L12 10.5L13.34 13.63L16.75 13.92L14.16 16.16M20 2H4V4L8.86 7.64A8 8 0 1 0 15.14 7.64L20 4M18 15A6 6 0 1 1 10.82 9.12A5.86 5.86 0 0 1 13.18 9.12A6 6 0 0 1 18 15M12.63 7H11.37L7.37 4H16.71Z" />
    </svg>
  }
}