pub mod new_case;
pub mod list_case;
pub mod view_case;
pub mod edit_case;
pub mod search;
use thaw::LayoutHeader;
use leptos::*;
use leptos_router::*;

#[component]
pub fn CaseManagementLayout() -> impl IntoView {
    view! {
        <div class="case-management-layout">
        <LayoutHeader>
           <h1>"Case Management"</h1>
        </LayoutHeader>
            <main>
                <Outlet/>
            </main>
        </div>
    }
}
