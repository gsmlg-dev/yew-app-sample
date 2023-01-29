#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SubwayAlertVariant)]
pub fn r#icon_subway_alert_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16 11V6H11V11H16M14.5 17C15.3 17 16 16.3 16 15.5C16 14.7 15.3 14 14.5 14S13 14.7 13 15.5C13 16.3 13.7 17 14.5 17M9 11V6H4V11H9M5.5 17C6.3 17 7 16.3 7 15.5C7 14.7 6.3 14 5.5 14S4 14.7 4 15.5C4 16.3 4.7 17 5.5 17M10 2C14.4 2 18 2.5 18 6V15.5C18 17.4 16.4 19 14.5 19L16 20.5V21H4V20.5L5.5 19C3.6 19 2 17.4 2 15.5V6C2 2.5 5.6 2 10 2M20 13V7H22V13H20M20 17V15H22V17H20Z" />
    </svg>
  }
}