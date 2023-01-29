#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_ApacheKafka)]
pub fn r#icon_apache_kafka(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15.54 12.97C14.86 12.97 14.24 13.22 13.76 13.64L12.47 12.89C12.56 12.6 12.6 12.29 12.6 11.97C12.6 11.65 12.56 11.34 12.5 11.05L13.73 10.32C14.21 10.76 14.85 11 15.54 11C17.03 11 18.24 9.81 18.24 8.32S17.03 5.63 15.54 5.63 12.84 6.84 12.84 8.33C12.84 8.5 12.86 8.7 12.89 8.88L11.64 9.6C11.21 9.15 10.64 8.82 10 8.65V7.26C11.09 6.91 11.88 5.89 11.88 4.69C11.88 3.2 10.67 2 9.18 2C7.69 2 6.5 3.2 6.5 4.69C6.5 5.89 7.26 6.9 8.34 7.26V8.66C6.86 9.04 5.76 10.37 5.76 11.97C5.76 13.57 6.86 14.91 8.34 15.28V16.73C7.26 17.09 6.5 18.1 6.5 19.3C6.5 20.79 7.69 22 9.18 22C10.67 22 11.88 20.79 11.88 19.3C11.88 18.1 11.09 17.08 10 16.73V15.29C10.64 15.13 11.2 14.8 11.64 14.35L12.9 15.08C12.86 15.27 12.84 15.46 12.84 15.66C12.84 17.15 14.05 18.36 15.54 18.36S18.24 17.15 18.24 15.66 17.03 12.97 15.54 12.97M15.54 7C16.28 7 16.87 7.59 16.87 8.32S16.28 9.66 15.54 9.66 14.21 9.06 14.21 8.32 14.8 7 15.54 7M7.85 4.69C7.85 3.95 8.44 3.35 9.18 3.35C9.92 3.35 10.5 3.95 10.5 4.69S9.92 6.03 9.18 6.03C8.44 6.03 7.85 5.43 7.85 4.69M10.5 19.3C10.5 20.04 9.92 20.64 9.18 20.64C8.44 20.64 7.85 20.04 7.85 19.3C7.85 18.56 8.44 17.96 9.18 17.96C9.92 17.96 10.5 18.56 10.5 19.3M9.18 13.89C8.12 13.89 7.26 13.03 7.26 11.97C7.26 10.91 8.12 10.05 9.18 10.05S11.1 10.91 11.1 11.97C11.1 13.03 10.24 13.89 9.18 13.89M15.54 17C14.8 17 14.21 16.4 14.21 15.66S14.8 14.33 15.54 14.33 16.87 14.93 16.87 15.66 16.28 17 15.54 17Z" />
    </svg>
  }
}