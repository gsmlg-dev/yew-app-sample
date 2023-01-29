#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CommentQuote)]
pub fn r#icon_comment_quote(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 2H4C2.9 2 2 2.9 2 4V16C2 17.1 2.9 18 4 18H8V21C8 21.6 8.4 22 9 22H9.5C9.7 22 10 21.9 10.2 21.7L13.9 18H20C21.1 18 22 17.1 22 16V4C22 2.9 21.1 2 20 2M11 13H7V8.8L8.3 6H10.3L8.9 9H11V13M17 13H13V8.8L14.3 6H16.3L14.9 9H17V13Z" />
    </svg>
  }
}