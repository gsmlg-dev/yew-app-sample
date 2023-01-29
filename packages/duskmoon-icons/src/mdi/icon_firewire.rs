#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Firewire)]
pub fn r#icon_firewire(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 10C13.1 10 14 10.9 14 12S13.1 14 12 14 10 13.1 10 12 10.9 10 12 10M12 8C9.8 8 8 9.8 8 12S9.8 16 12 16 16 14.2 16 12 14.2 8 12 8M15 17H9V19H15V17M15 20H9V22H15V20M6.3 10.6L10.5 6.4L9.2 4.9L4.9 9.2L6.3 10.6M4.2 8.5L8.4 4.3L7.1 2.8L2.8 7.1L4.2 8.5M15 17H9V19H15V17M13.4 6.3L17.6 10.5L21.1 7L16.9 2.8L13.4 6.3Z" />
    </svg>
  }
}