#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CameraWireless)]
pub fn r#icon_camera_wireless(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,10.8A3.2,3.2 0 0,1 15.2,14A3.2,3.2 0 0,1 12,17.2A3.2,3.2 0 0,1 8.8,14A3.2,3.2 0 0,1 12,10.8M16,3.33V2A6,6 0 0,1 22,8H20.67C20.67,5.42 18.58,3.33 16,3.33M16,6V4.67C17.84,4.67 19.33,6.16 19.33,8H18C18,6.89 17.11,6 16,6M17,9H22V20A2,2 0 0,1 20,22H4A2,2 0 0,1 2,20V8A2,2 0 0,1 4,6H7.17L9,4H15V7C16.11,7 17,7.89 17,9M12,19A5,5 0 0,0 17,14A5,5 0 0,0 12,9A5,5 0 0,0 7,14A5,5 0 0,0 12,19Z" />
    </svg>
  }
}