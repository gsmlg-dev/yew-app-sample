#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Necklace)]
pub fn r#icon_necklace(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21.5 5H19.5C19.5 9.14 16.14 12.5 12 12.5C7.86 12.5 4.5 9.14 4.5 5H2.5C2.55 10.11 6.59 14.29 11.7 14.5C11.1 15.4 10 17.2 10 18C10 20.67 14 20.67 14 18C14 17.2 12.9 15.4 12.3 14.5C17.41 14.29 21.45 10.11 21.5 5Z" />
    </svg>
  }
}