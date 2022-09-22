use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{home::Home, login::Login, setup_page::SetupPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/afterdelete")]
    AfterDelete,
    #[at("/afterRun")]
    AfterRun,
    #[at("/login")]
    Login,
    #[at("/setup")]
    SetupPage,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::AfterDelete => html! { <Redirect<Route> to={Route::Home}/> },
        Route::AfterRun => html! { <Redirect<Route> to={Route::AfterDelete}/> },
        Route::Login => html! { <Login /> },
        Route::SetupPage => html! { <SetupPage /> },
    }
}
