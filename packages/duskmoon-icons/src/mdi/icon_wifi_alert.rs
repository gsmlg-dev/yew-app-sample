#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_WifiAlert)]
pub fn r#icon_wifi_alert(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20.24 5H18V7.25C16.16 6.45 14.13 6 12 6C8.62 6 5.5 7.12 3 9L1.2 6.6C4.21 4.34 7.95 3 12 3C14.97 3 17.77 3.73 20.24 5M8.4 16.2L12 21L15.6 16.2C14.6 15.45 13.35 15 12 15S9.4 15.45 8.4 16.2M4.8 11.4L6.6 13.8C8.1 12.67 9.97 12 12 12S15.9 12.67 17.4 13.8L18 13V10.62C16.23 9.59 14.19 9 12 9C9.3 9 6.81 9.89 4.8 11.4M20 17H22V15H20V17M20 7V13H22V7H20Z" />
    </svg>
  }
}