#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_AlertRemove)]
pub fn r#icon_alert_remove(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14 19C14 16.21 15.91 13.87 18.5 13.2L12 2L1 21H14.35C14.13 20.37 14 19.7 14 19M13 18H11V16H13V18M13 14H11V10H13V14M23.54 16.88L21.41 19L23.54 21.12L22.12 22.54L20 20.41L17.88 22.54L16.47 21.12L18.59 19L16.47 16.88L17.88 15.47L20 17.59L22.12 15.46L23.54 16.88Z" />
    </svg>
  }
}