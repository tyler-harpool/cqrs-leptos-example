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
        <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600&family=Merriweather:wght@700&display=swap" rel="stylesheet" />
        <Link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet" />
        <script src="https://cdn.tailwindcss.com"></script>

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
