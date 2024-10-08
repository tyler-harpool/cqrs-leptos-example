use crate::application::commands::case_management::create::{CreateCaseDto, create_case};
use leptos::*;
use thaw::{Button, ButtonColor, Input, InputVariant, Space, TextArea};
use leptos_router::{ActionForm, use_navigate};
use crate::pages::components::Default_Layout;
#[server(NewCase, "/api")]
pub async fn create_case_action(
    case_number: String,
    title: String,
    filing_date: String,
    description: String
) -> Result<CreateCaseDto, ServerFnError> {
    create_case(case_number, title, filing_date, description).await
}


#[component]
pub fn NewCase() -> impl IntoView {
    let create_case_action = create_server_action::<NewCase>();
    let response = create_case_action.value();
    let has_error = move || response.with(|val| matches!(val, Some(Err(_))));

    let navigate = use_navigate();

    let case_number = create_rw_signal(String::new());
    let title = create_rw_signal(String::new());
    let filing_date = create_rw_signal(String::new());
    let description = create_rw_signal(String::new());

    view! {
      <Default_Layout>
        <div class="new-case">
            <h1>"New Case"</h1>
              <ActionForm action=create_case_action>
                <Space vertical=true>
                    <label for="case-number">"Case Number:"</label>
                    <Input
                        variant=InputVariant::Text
                        attr:id="case-number"
                        attr:name="case_number"
                        placeholder="Enter case number"
                    />

                    <label for="case-title">"Case Title:"</label>
                    <Input
                        variant=InputVariant::Text
                        attr:id="case-title"
                        attr:name="title"
                        placeholder="Enter case title"
                    />

                    <label for="filing-date">"Filing Date:"</label>
                    <Input
                        variant=InputVariant::Text
                        attr:id="filing-date"
                        attr:name="filing_date"
                        attr:type="date"
                    />

                    <label for="case-description">"Case Description:"</label>
                    <TextArea
                        attr:id="case-description"
                        attr:name="description"
                        placeholder="Enter case description"
                    />

                    <Space>
                        <Button
                            color=ButtonColor::Primary
                            attr:type="submit"
                        >
                            "Create Case"
                        </Button>

                    </Space>
                </Space>
            </ActionForm>
            <Button
                color=ButtonColor::Warning
                on_click=move |_| {
                    case_number.set(String::new());
                    title.set(String::new());
                    filing_date.set(String::new());
                    description.set(String::new());
                }
            >
                "Reset Form"
            </Button>
            {move || has_error().then(|| view! {
                <p class="error">"An error occurred while creating the case."</p>
            })}
            {move || response.with(|val| match val {
                Some(Ok(case)) => {
                    navigate(&format!("/case-management/{}", case.case_number), Default::default());
                    view! {}.into_view()
                },
                _ => view! {}.into_view()
            })}
        </div>
        </Default_Layout>
    }
}
