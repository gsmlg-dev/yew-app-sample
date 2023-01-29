#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_TowerBeach)]
pub fn r#icon_tower_beach(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17,4V8H18V10H17.64L21,23H18.93L18.37,20.83L12,17.15L5.63,20.83L5.07,23H3L6.36,10H6V8H7V4H6V3L18,1V4H17M7.28,14.43L6.33,18.12L10,16L7.28,14.43M15.57,10H8.43L7.8,12.42L12,14.85L16.2,12.42L15.57,10M17.67,18.12L16.72,14.43L14,16L17.67,18.12Z" />
    </svg>
  }
}