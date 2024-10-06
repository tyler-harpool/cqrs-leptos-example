use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::Home;

use crate::pages::home::NewPage;
use crate::pages::notfound::NotFound;
use crate::pages::case_management::{
    CaseManagementLayout,
    new_case::NewCase,
    list_case::ListCases,
    view_case::ViewCase,
    edit_case::EditCase,
};

#[component]
pub fn AppRouter() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        <Stylesheet id="leptos" href="/pkg/l3xodus.css"/>

        // sets the document title
                <Title text="Lexodus - Federal Courts Case Management"/>

        // content for this welcome page
        <Router>
            <main>
            <Routes>
                <Route path="" view=Home/>
                <Route path="/newpage" view=NewPage/>
                <Route path="cases" view=CaseManagementLayout>
                    <Route path="" view=ListCases/>
                    <Route path="new" view=NewCase/>
                    <Route path=":id" view=ViewCase/>
                    <Route path=":id/edit" view=EditCase/>
                </Route>
                <Route path="/*any" view=NotFound/>
            </Routes>
            </main>
        </Router>
    }
}
