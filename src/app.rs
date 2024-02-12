use crate::error_template::{AppError, ErrorTemplate};
use board::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod board;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-krabbels-6.css"/>

        // sets the document title
        <Title text="Welcome to Krabbels"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {

        <main class="container mx-auto pt-2 lg:p-5 grid grid-rows-2 lg:grid-rows-1 lg:grid-cols-2">
            <div class="flex justify-center">
                <Board/>
            </div>

            <div class="pl-5">
                <h1 class="hidden lg:block p-5 text-4xl font-bold dark:text-yellow-100 text-center">"KRABBELS"</h1>
                <h2 class="text-xs text-center p-2 mb-5 border-b-2 border-black dark:border-white dark:text-white">"A study project to learn further RUST, LEPTOS framework and TAILDWIND css."</h2>
                // <TileBag/>
            </div>
        </main>
    }
}
