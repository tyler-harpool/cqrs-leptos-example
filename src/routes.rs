use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::Home;
use crate::pages::home::NewPage;
use crate::pages::notfound::NotFound;

#[component]
pub fn AppRouter() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        <Stylesheet id="leptos" href="/pkg/l3xodus.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Home/>
                        <Route path="/newpage" view=NewPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
