use leptos::*;
use leptos_router::*;
use crate::pages::components::default_layout::DefaultLayout;
#[component]
pub fn Home() -> impl IntoView {
    let counter_key = "main_counter";
    let increment_count = create_server_action::<IncrementCount>();

    let count = create_resource(
        move || increment_count.version().get(),
        move |_| get_count(counter_key.to_string()),
    );

    view! {
      <DefaultLayout>

        <a href="/newpage">"Go New Page"</a>
        <ActionForm action=increment_count>
            <input type="hidden" name="key" value=counter_key />
            <button>"Click Me: " {move || count.get()}</button>
        </ActionForm>
        </DefaultLayout>
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
      <DefaultLayout>
        <ActionForm action=increment_count>
            <input type="hidden" name="key" value=counter_key />
            <button>"Increment New Counter: " {move || count.get()}</button>
        </ActionForm>
        <a href="/">"Go back"</a>
        </DefaultLayout>
    }
}
