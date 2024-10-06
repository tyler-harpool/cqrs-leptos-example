use crate::application::commands::case_management::create::{CreateCaseDto, create_case};
use leptos::*;
use thaw::{Button, ButtonColor, Input, InputVariant, Space, TextArea};
use leptos_router::ActionForm;

#[server(CreateCase, "/api")]
pub async fn create_case_action(case: CreateCaseDto) -> Result<(), ServerFnError> {
    // No need for date conversion, just pass the case directly
    create_case(case).await
}

#[component]
pub fn NewCase() -> impl IntoView {
    let create_case_action = create_server_action::<CreateCase>();
    let value = create_case_action.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    let case_number = create_rw_signal(String::new());
    let title = create_rw_signal(String::new());
    let filing_date = create_rw_signal(String::new());
    let description = create_rw_signal(String::new());

    view! {
        <div class="new-case">
            <h1>"New Case"</h1>
            <ActionForm action=create_case_action>
                <Space vertical=true>
                    <label for="case-number">"Case Number:"</label>
                    <Input
                        value=case_number
                        variant=InputVariant::Text
                        attr:id="case-number"
                        attr:name="case_number"
                        placeholder="Enter case number"
                    />

                    <label for="case-title">"Case Title:"</label>
                    <Input
                        value=title
                        variant=InputVariant::Text
                        attr:id="case-title"
                        attr:name="title"
                        placeholder="Enter case title"
                    />

                    <label for="filing-date">"Filing Date:"</label>
                    <Input
                        value=filing_date
                        variant=InputVariant::Text
                        attr:id="filing-date"
                        attr:name="filing_date"
                        attr:type="date"
                    />

                    <label for="case-description">"Case Description:"</label>
                    <TextArea
                        value=description
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
            {move || value.with(|val| match val {
                Some(Ok(_)) => view! { <p class="success">"Case created successfully!"</p> }.into_view(),
                _ => view! {}.into_view()
            })}
        </div>
    }
}
