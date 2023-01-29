#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Dice6Outline)]
pub fn r#icon_dice_6_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 5V19H5V5H19M19 3H5C3.9 3 3 3.9 3 5V19C3 20.1 3.9 21 5 21H19C20.1 21 21 20.1 21 19V5C21 3.9 20.1 3 19 3M7.5 6C6.7 6 6 6.7 6 7.5S6.7 9 7.5 9 9 8.3 9 7.5 8.3 6 7.5 6M16.5 15C15.7 15 15 15.7 15 16.5C15 17.3 15.7 18 16.5 18C17.3 18 18 17.3 18 16.5C18 15.7 17.3 15 16.5 15M16.5 10.5C15.7 10.5 15 11.2 15 12S15.7 13.5 16.5 13.5C17.3 13.5 18 12.8 18 12S17.3 10.5 16.5 10.5M16.5 6C15.7 6 15 6.7 15 7.5S15.7 9 16.5 9C17.3 9 18 8.3 18 7.5S17.3 6 16.5 6M7.5 10.5C6.7 10.5 6 11.2 6 12S6.7 13.5 7.5 13.5 9 12.8 9 12 8.3 10.5 7.5 10.5M7.5 15C6.7 15 6 15.7 6 16.5C6 17.3 6.7 18 7.5 18S9 17.3 9 16.5C9 15.7 8.3 15 7.5 15Z" />
    </svg>
  }
}