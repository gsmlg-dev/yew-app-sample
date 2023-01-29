#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ChiliHot)]
pub fn r#icon_chili_hot(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10.43 7.32L8.86 6.42C9.38 5.6 10.11 5 10.94 4.7C10.81 4.3 10.45 4 10 4V2C10.77 2 11.47 2.29 12 2.77V4C12.45 4 12.81 4.3 12.94 4.7C11.83 5.08 10.93 6.05 10.43 7.32M10 11C10 10.23 10.23 9.5 10.6 8.91L9.26 8.15C8.5 8.44 8 9.16 8 10V11C8 18.05 12.9 20.8 15.03 21.66C12.41 19.62 10 16.2 10 11M6.43 7.32L4.86 6.42C5.38 5.6 6.11 5 6.94 4.7C6.81 4.3 6.45 4 6 4V2C6.77 2 7.47 2.29 8 2.77V4C8.45 4 8.81 4.3 8.94 4.7C7.83 5.08 6.93 6.05 6.43 7.32M6 11C6 10.23 6.23 9.5 6.6 8.91L5.26 8.15C4.5 8.44 4 9.16 4 10V11C4 18.05 8.9 20.8 11.03 21.66C8.41 19.62 6 16.2 6 11M19 8.28L17.75 9L16 8L14.25 9L13 8.28C12.4 8.63 12 9.27 12 10V11C12 20 20 22 20 22V10C20 9.27 19.6 8.63 19 8.28M12.73 6.63L14.25 7.5L16 6.5L17.75 7.5L19.27 6.63C18.72 5.66 17.91 4.94 16.97 4.65C16.79 3.16 15.54 2 14 2V4C14.44 4 14.8 4.29 14.94 4.69C14.03 5 13.26 5.7 12.73 6.63Z" />
    </svg>
  }
}