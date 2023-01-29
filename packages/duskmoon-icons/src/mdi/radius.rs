#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Radius)]
pub fn r#icon_radius(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2M17,15V13H13.72C13.36,13.62 12.71,14 12,14A2,2 0 0,1 10,12A2,2 0 0,1 12,10C12.71,10 13.36,10.38 13.72,11H17V9L20,12L17,15Z" />
    </svg>
  }
}