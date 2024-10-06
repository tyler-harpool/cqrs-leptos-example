pub mod new_case;
pub mod list_case;
pub mod view_case;
pub mod edit_case;


use leptos::*;
use leptos_router::*;

#[component]
pub fn CaseManagementLayout() -> impl IntoView {
    view! {
        <div class="case-management-layout">
            <h1>"Case Management"</h1>
            <nav>
                <A href="/cases">"List Cases"</A>
                <A href="/cases/new">"New Case"</A>
            </nav>
            <main>
                <Outlet/>
            </main>
        </div>
    }
}
