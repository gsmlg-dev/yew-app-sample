#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Bone)]
pub fn r#icon_bone(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8,14A3,3 0 0,1 5,17A3,3 0 0,1 2,14C2,13.23 2.29,12.53 2.76,12C2.29,11.47 2,10.77 2,10A3,3 0 0,1 5,7A3,3 0 0,1 8,10C9.33,10.08 10.67,10.17 12,10.17C13.33,10.17 14.67,10.08 16,10A3,3 0 0,1 19,7A3,3 0 0,1 22,10C22,10.77 21.71,11.47 21.24,12C21.71,12.53 22,13.23 22,14A3,3 0 0,1 19,17A3,3 0 0,1 16,14C14.67,13.92 13.33,13.83 12,13.83C10.67,13.83 9.33,13.92 8,14Z" />
    </svg>
  }
}