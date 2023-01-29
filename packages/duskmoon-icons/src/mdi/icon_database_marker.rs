#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_DatabaseMarker)]
pub fn r#icon_database_marker(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18.5 12C16.6 12 15 13.6 15 15.5C15 18.1 18.5 22 18.5 22S22 18.1 22 15.5C22 13.6 20.4 12 18.5 12M18.5 16.8C17.8 16.8 17.3 16.2 17.3 15.6C17.3 14.9 17.9 14.4 18.5 14.4S19.7 15 19.7 15.6C19.8 16.2 19.2 16.8 18.5 16.8M4 12V9C4 11.21 7.58 13 12 13C12.57 13 13.13 12.97 13.67 12.91C13.25 13.69 13 14.57 13 15.5C13 15.65 13 15.81 13.03 15.96C12.69 16 12.35 16 12 16C7.58 16 4 14.21 4 12M4 7C4 4.79 7.58 3 12 3S20 4.79 20 7 16.42 11 12 11 4 9.21 4 7M15 20.71C14.07 20.9 13.06 21 12 21C7.58 21 4 19.21 4 17V14C4 16.21 7.58 18 12 18C12.5 18 13.03 17.97 13.5 17.93C13.9 18.91 14.44 19.87 15 20.71Z" />
    </svg>
  }
}