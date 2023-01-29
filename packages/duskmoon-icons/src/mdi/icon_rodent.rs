#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Rodent)]
pub fn r#icon_rodent(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21.33 17.39C22.73 18.66 21.8 21 19.92 21H11.06C8.25 21 6 18.75 6 15.94V15.89C3.7 15.42 2 13.41 2 11C2 8.25 4.22 6 7 6H9.5C9.8 6 10 5.77 10 5.5S9.8 5 9.5 5H7V3H9.5C10.88 3 12 4.13 12 5.5C12 6.89 10.88 8 9.5 8H7C5.34 8 4 9.33 4 11C4 12.37 4.92 13.5 6.14 13.87C6.7 11.67 8.67 10 11.06 10C11.86 10 12.66 10.22 13.36 10.55C11.95 11.34 11 12.8 11 14.5C11 15.75 11.5 16.87 12.33 17.67L13.03 16.97C12.38 16.36 12 15.47 12 14.5C12 11.91 14.34 11 15.5 11C17.58 11 19.45 12.89 18.94 15.23L21.33 17.39M18 19C18.56 19 19 18.56 19 18S18.56 17 18 17 17 17.44 17 18 17.44 19 18 19Z" />
    </svg>
  }
}