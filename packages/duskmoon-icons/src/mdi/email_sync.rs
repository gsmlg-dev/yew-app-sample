#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_EmailSync)]
pub fn r#icon_email_sync(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M3 4C1.89 4 1 4.89 1 6V18C1 19.11 1.9 20 3 20H13.5A6.5 6.5 0 0 1 13 17.5A6.5 6.5 0 0 1 19.5 11A6.5 6.5 0 0 1 21 11.18V6C21 4.89 20.1 4 19 4H3M3 6L11 11L19 6V8L11 13L3 8V6M19 12L16.75 14.25L19 16.5V15C20.38 15 21.5 16.12 21.5 17.5C21.5 17.9 21.41 18.28 21.24 18.62L22.33 19.71C22.75 19.08 23 18.32 23 17.5C23 15.29 21.21 13.5 19 13.5V12M15.67 15.29C15.25 15.92 15 16.68 15 17.5C15 19.71 16.79 21.5 19 21.5V23L21.25 20.75L19 18.5V20C17.62 20 16.5 18.88 16.5 17.5C16.5 17.1 16.59 16.72 16.76 16.38L15.67 15.29Z" />
    </svg>
  }
}