#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_RemoteTvOff)]
pub fn r#icon_remote_tv_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2,5.27L3.28,4L20,20.72L18.73,22L17,20.25C16.86,21.24 16,22 15,22H9A2,2 0 0,1 7,20V10.27L2,5.27M9,2H11V4H13V2H15A2,2 0 0,1 17,4V15.18L13,11.18V10H15V8H13V6H11V8H9.82L7,5.18V4A2,2 0 0,1 9,2M9,20H11V18H9V20M13,20H15V18.27L14.73,18H13V20M9,14V16H11V14.27L10.73,14H9Z" />
    </svg>
  }
}