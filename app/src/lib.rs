#![allow(non_snake_case)]
mod counter;
mod index;
mod nav;
mod post;

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, StaticRouter};

#[derive(Route, Clone)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/counter")]
    Counter,
    #[to("/blog")]
    PostsList,
    #[to("/blog/<path>")]
    Post { path: String },
    #[not_found]
    NotFound,
}

fn switch(route: ReadSignal<AppRoutes>) -> View {
    view! {
        div {
            nav::Nav()
            (match route.get_clone() {
                AppRoutes::Index => index::Index(),
                AppRoutes::Counter => counter::Counter(),
                AppRoutes::PostsList => post::PostList(),
                AppRoutes::Post { path } => post::Post(path),
                AppRoutes::NotFound => view! {
                    "404 Not Found"
                },
            })
        }
    }
}

/// # Props
/// * `pathname` - Set to `Some(_)` if running on the server.
#[component]
pub fn App(pathname: Option<String>) -> View {
    match pathname {
        Some(pathname) => {
            let route = AppRoutes::default().match_path(&pathname);
            view! {
                StaticRouter(
                    view=switch,
                    route=route,
                )
            }
        }
        None => view! {
            Router(
                view=switch,
                integration=HistoryIntegration::new(),
            )
        },
    }
}
