use leptos::*;
use leptos_router::*;
use thaw::{Button, ButtonVariant, Layout, LayoutHeader};

use crate::pages::components::Default_Layout;
#[component]
pub fn Home() -> impl IntoView {
    let counter_key = "main_counter";
    let increment_count = create_server_action::<IncrementCount>();

    let count = create_resource(
        move || increment_count.version().get(),
        move |_| get_count(counter_key.to_string()),
    );




    view! {
      <Default_Layout>
      <Layout class="app-layout" has_sider=true>



          <Layout class="main-content">
              <LayoutHeader>
                 <h1>"Federal Court Case Management System"</h1>
              </LayoutHeader>
              <Layout class="content-area">
                <ActionForm action=increment_count>
                    <input type="hidden" name="key" value=counter_key />
                    <Button variant=ButtonVariant::Primary>"Click Me: " {move || count.get()}</Button>
                </ActionForm>
                <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                    "Click number " {count}
                </button>
              </Layout>
          </Layout>

      </Layout>
      </Default_Layout>
    }
}

#[server(IncrementCount, "/api")]
pub async fn increment_count(key: String) -> Result<(), ServerFnError> {
    use crate::application::commands::increment_counter::increment_counter;
    increment_counter(key).await
}

#[server(GetCount, "/api")]
pub async fn get_count(key: String) -> Result<u64, ServerFnError> {
    use crate::application::queries::get_count::get_count as get_count_query;
    get_count_query(key).await
}

#[component]
pub fn NewPage() -> impl IntoView {
    let counter_key = "new_page_counter";
    let increment_count = create_server_action::<IncrementCount>();

    let count = create_resource(
        move || increment_count.version().get(),
        move |_| get_count(counter_key.to_string()),
    );

    view! {
        <h1>"New Page Counter"</h1>
        <ActionForm action=increment_count>
             <input type="hidden" name="key" value=counter_key />
             <Button variant=ButtonVariant::Primary>"Increment New Counter: " {move || count.get()}</Button>
        </ActionForm>
        <a href="/">"Go back"</a>
    }
}
