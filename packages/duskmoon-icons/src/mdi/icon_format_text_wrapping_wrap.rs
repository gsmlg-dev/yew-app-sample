#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FormatTextWrappingWrap)]
pub fn r#icon_format_text_wrapping_wrap(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7,21H5V3H7V21M19,3H17V21H19V3M13,8H9V10H12.97C13.14,10 14,10.16 14,12C14,13.84 13.14,14 13,14H11V12L8,15L11,18V16H13C14.04,16 16,15.16 16,12C16,8.84 14.04,8 13,8Z" />
    </svg>
  }
}
