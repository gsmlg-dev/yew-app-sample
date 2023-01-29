#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_BusArticulatedEnd)]
pub fn r#icon_bus_articulated_end(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21.5,6L20,7.5L21.5,9L20,10.5L21.5,12L20,13.5L21.5,15H12.5A3,3 0 0,1 9.5,18A3,3 0 0,1 6.5,15H2.5V8C2.5,6.89 3.39,6 4.5,6H21.5M18.5,7.5H15V10H18.5V7.5M13.5,7.5H9.5V10H13.5V7.5M8,7.5H4V10H8V7.5M9.5,13.5A1.5,1.5 0 0,0 8,15A1.5,1.5 0 0,0 9.5,16.5A1.5,1.5 0 0,0 11,15A1.5,1.5 0 0,0 9.5,13.5Z" />
    </svg>
  }
}