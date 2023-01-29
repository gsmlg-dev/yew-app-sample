#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CircleExpand)]
pub fn r#icon_circle_expand(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16,4L20,8V4M20,16L16,20H20M8,20L4,16V20M4,8L8,4H4M16.95,7.05C14.22,4.32 9.78,4.32 7.05,7.05C4.32,9.78 4.32,14.22 7.05,16.95C9.78,19.68 14.22,19.68 16.95,16.95C19.68,14.22 19.68,9.79 16.95,7.05M15.85,15.85C13.72,18 10.28,18 8.15,15.85C6,13.72 6,10.28 8.15,8.15C10.28,6 13.72,6 15.85,8.15C18,10.28 18,13.72 15.85,15.85Z" />
    </svg>
  }
}