#![allow(non_snake_case)]

mod comps;
mod pages;

use std::future::Future;
use tokio::runtime::Handle;

use crate::comps::{Footer, Header};
use crate::pages::{ArticleAdd, HomePage, NotFoundPage, SettingsPage, SignInPage, SignUpPage};
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use sir::{global_css, AppStyle};

fn main() {
    // Init debug tool for WebAssembly.
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    global_css!(" a:focus { outline: 0; } ");

    cx.render(rsx!(
        AppStyle{ },
        Router {
            Header { }
            Route { to: "/", HomePage {} }
            Route { to: "/signin", SignInPage {} }
            Route { to: "/signup", SignUpPage {} }
            Route { to: "/article_add", ArticleAdd {} }
            Route { to: "/settings", SettingsPage {} }
            // If the current location doesn't match any of the above routes,
            // render the NotFoundPage component.
            Route { to: "", NotFoundPage {} }
            Footer{ }
        }
    ))
}

pub(crate) fn block_on<T: Send + Sync + 'static>(
    f: impl Future<Output = T> + Send + Sync + 'static,
) -> T {
    let handle = Handle::current();
    std::thread::spawn(move || {
        // Using Handle::block_on to run async code in the new thread.
        handle.block_on(f)
    })
    .join()
    .unwrap()
}
