use yew::prelude::*;

use stylist::css;
use stylist::yew::use_style;

use crate::routes::AppRoute;
use crate::theme::Theme;

use crate::ui::header::Header as UIHeader;
use crate::ui::link::Link;

/// Header component
#[function_component(Header)]
pub fn header() -> Html {
    let theme = use_context::<Theme>().expect("no theme found");
    let style = use_style(css!(
        r#"
        color: ${color};
        background-color: ${background_color};
    "#,
        color = theme.foreground,
        background_color = theme.background,
    ));

    html! {
      <UIHeader
        classes={style}
        logo={
          html! {
            <h1>
              { "Yew APP Sample" }
            </h1>
          }
        }
        menu={
          html! {
            <>
              <Link<AppRoute>
                to={AppRoute::Home}
              >
                { "Home" }
              </Link<AppRoute>>
              <Link<AppRoute>
                to={AppRoute::About}
              >
                { "About" }
              </Link<AppRoute>>
            </>
          }
        }
      />

    }
}
