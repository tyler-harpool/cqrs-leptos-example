use leptos::*;
use leptos_router::*;
use icondata::*;
use thaw::{Button, ButtonVariant, Layout, LayoutHeader, LayoutSider, Menu, MenuItem};
#[component]
pub fn Home() -> impl IntoView {
    let counter_key = "main_counter";
    let increment_count = create_server_action::<IncrementCount>();

    let count = create_resource(
        move || increment_count.version().get(),
        move |_| get_count(counter_key.to_string()),
    );
    let value = create_rw_signal(String::from("o"));
    view! {
      <Layout has_sider=true>
          <LayoutSider style="padding: 20px;">
            <Menu value default_expanded_keys=vec![String::from("area")]>
                <MenuItem key="a" label="And"/>
                <MenuItem key="o" label="Or"/>
                <MenuItem icon=icondata::AiAreaChartOutlined key="area" label="Area Chart">
                    <a href="/newpage"><MenuItem key="target" label="Target"/></a>
                    <MenuItem key="above" label="Above"/>
                    <MenuItem key="below" label="Below"/>
                </MenuItem>
                <MenuItem icon=icondata::AiPieChartOutlined key="pie" label="Pie Chart">
                    <MenuItem key="pie-target" label="Target"/>
                    <MenuItem key="pie-above" label="Above"/>
                    <MenuItem key="pie-below" label="Below"/>
                </MenuItem>
                <MenuItem icon=icondata::AiGithubOutlined key="github" label="Github"/>
                <MenuItem icon=icondata::AiChromeOutlined key="chrome" label="Chrome"/>
            </Menu>
          </LayoutSider>
          <Layout>
              <LayoutHeader style="background-color: #0078ffaa; padding: 20px;">
                 <h1>"Welcome to Lexodus"</h1>
              </LayoutHeader>
              <Layout style="background-color: #0078ff88; padding: 20px;">
                <ActionForm action=increment_count>
                    <input type="hidden" name="key" value=counter_key />
                    <Button variant=ButtonVariant::Primary>"Click Me: " {move || count.get()}</Button>
                </ActionForm>
              </Layout>
          </Layout>
      </Layout>



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
