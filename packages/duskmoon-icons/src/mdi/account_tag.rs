#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_AccountTag)]
pub fn r#icon_account_tag(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21.8 17.8L18.2 14.2C18 14.1 17.8 14 17.6 14H14.8C14.4 14 14 14.4 14 14.8V17.6C14 17.8 14.1 18 14.2 18.2L17.8 21.8C17.9 21.9 18.1 22 18.4 22C18.6 22 18.8 21.9 19 21.8L21.8 19C21.9 18.9 22 18.7 22 18.4C22 18.2 21.9 18 21.8 17.8M15.4 16C15.1 16 14.8 15.7 14.8 15.4C14.8 15.1 15.1 14.8 15.4 14.8C15.7 14.8 16 15.1 16 15.4C16 15.7 15.7 16 15.4 16M10 4C7.8 4 6 5.8 6 8S7.8 12 10 12 14 10.2 14 8 12.2 4 10 4M10 14C5.6 14 2 15.8 2 18V20H13.2L12.8 19.6C12.3 19.1 12 18.4 12 17.6V14.8C12 14.6 12 14.4 12.1 14.1C11.4 14 10.7 14 10 14Z" />
    </svg>
  }
}