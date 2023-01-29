#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_WifiStrength1Alert)]
pub fn r#icon_wifi_strength_1_alert(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 3C7.8 3 3.7 4.4 .4 7C4.4 12.1 7.9 16.4 12 21.5C14.4 18.5 16.7 15.7 19 12.8V9.6L15.6 13.8C14.5 13.3 13.2 13 12 13S9.5 13.3 8.4 13.8L3.3 7.4C5.9 5.8 8.9 5 12 5S18.1 5.9 20.7 7.4L20.3 8H22.9C23.1 7.7 23.5 7.3 23.7 7C20.3 4.4 16.2 3 12 3M21 10V16H23V10M21 18V20H23V18" />
    </svg>
  }
}