#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Cupcake)]
pub fn r#icon_cupcake(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,1.5A2.5,2.5 0 0,1 14.5,4A2.5,2.5 0 0,1 12,6.5A2.5,2.5 0 0,1 9.5,4A2.5,2.5 0 0,1 12,1.5M15.87,5C18,5 20,7 20,9C22.7,9 22.7,13 20,13H4C1.3,13 1.3,9 4,9C4,7 6,5 8.13,5C8.57,6.73 10.14,8 12,8C13.86,8 15.43,6.73 15.87,5M5,15H8L9,22H7L5,15M10,15H14L13,22H11L10,15M16,15H19L17,22H15L16,15Z" />
    </svg>
  }
}