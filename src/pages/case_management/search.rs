
use leptos::*;
use leptos_router::ActionForm;
use thaw::{Table, MultiSelect, TagVariant, MultiSelectOption,Icon, InputPrefix, DatePicker, Checkbox, Input, InputVariant, RadioItem, Select, SelectOption, Space, RadioGroup, Button};
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use crate::pages::components::Default_Layout;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CaseSearchCriteria {
    pub case_number: Option<String>,
    pub case_status: Option<String>,
    pub filed_date_from: Option<String>,
    pub filed_date_to: Option<String>,
    pub last_entry_date_from: Option<String>,
    pub last_entry_date_to: Option<String>,
    pub cause_of_action: Option<String>,
    pub nature_suit: Option<Vec<String>>,
    pub last_business_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub type_field: Option<String>,
    pub exact_matches_only: Option<bool>,
}

#[component]
pub fn SearchCases() -> impl IntoView {
  let case_number = create_rw_signal(String::new());
  let case_status = create_rw_signal(None);
  let nature_suit_value = create_rw_signal(vec![]);
  let cause_of_action_value = create_rw_signal(vec![]);
  let type_value = create_rw_signal(None::<String>);

  let filed_date_from = create_rw_signal(None::<NaiveDate>);
  let filed_date_to = create_rw_signal(None::<NaiveDate>);
  let last_business_name = create_rw_signal(String::new());
  let first_name = create_rw_signal(String::new());
  let middle_name = create_rw_signal(String::new());
  let exact_matches_only = create_rw_signal(false);

  let nature_suit_options = vec![
      MultiSelectOption::new("0 (zero)", "0".to_string()),
      MultiSelectOption::new("110 (Insurance)", "110".to_string())
          .with_variant(TagVariant::Success),
      // ... (keep the other options)
  ];

  let cause_of_action_options = vec![
      MultiSelectOption::new("0 (No cause code entered)", "0".to_string()),
      MultiSelectOption::new("02:0431 (Federal Election Commission: Failure Enforce Compliance)", "02:0431".to_string()),
      MultiSelectOption::new("05:0552 (Freedom of Information Act)", "05:0552".to_string()),
      MultiSelectOption::new("07:0601 (USDA Condemnation)", "07:0601".to_string()),
      MultiSelectOption::new("12:2601 (Real Estate Settlement Procedures Act)", "12:2601".to_string()),
      MultiSelectOption::new("05:0552fi (05:552 Freedom of Information Act)", "05:0552fi".to_string()),
      MultiSelectOption::new("05:0552pa (05:552 Right to Privacy Act)", "05:0552pa".to_string()),
      MultiSelectOption::new("08:1105 (8:1105(a) Aliens: Habeas Corpus to Release INS Detainee)", "08:1105".to_string()),
      MultiSelectOption::new("28:1332al (28:1332 Diversity-Airline Crash)", "28:1332al".to_string()),
  ];

  let type_options = vec![
    SelectOption::new("Attorney", String::from("attorney")),
    SelectOption::new("Party", String::from("party")),
  ];

  let search_action = create_server_action::<SearchCasesAction>();


    view! {
      <Default_Layout>
      <div class="min-h-screen bg-gray-50 p-4 md:p-8">
          <div class="mx-auto max-w-4xl rounded-xl bg-white p-6 shadow-lg">
              <h1 class="mb-8 text-3xl font-bold text-gray-900">"Search Cases"</h1>


                 <ActionForm action=search_action class="space-y-6">
                 <Space vertical=true class="w-full">
                    <label for="caseNumber" class="block text-sm font-medium text-gray-700">"Case Number"</label>
                    <Input
                        variant=InputVariant::Text
                        attr:id="caseNumber"
                        attr:name="case_number"
                        placeholder="Enter case number"
                        value=case_number
                        class="w-full"
                    >
                    <InputPrefix slot>
                        <Icon icon=icondata::FiActivity class="text-gray-400"/>
                    </InputPrefix>
                  </Input>
                 </Space>

                 <Space vertical=true class="w-full">
                     <label for="case_status" class="block text-sm font-medium text-gray-700">"Case Status:"</label>
                     <RadioGroup value=case_status >
                         <RadioItem key="open" class="flex space-x-4">"Open"</RadioItem>
                         <RadioItem key="closed" class="flex space-x-4">"Closed"</RadioItem>
                         <RadioItem key="all" class="flex space-x-4">"All"</RadioItem>
                     </RadioGroup>
                 </Space>

                    <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                    <Space vertical=true class="w-full">
                        <label for="filedDateFrom" class="block text-sm font-medium text-gray-700">"Filed Date (from)"</label>
                        <DatePicker
                            attr:id="filedDateFrom"
                            attr:name="filed_date_from"
                            value=filed_date_from
                            class="w-full"
                        />
                    </Space>
                    <Space vertical=true class="w-full">
                        <label for="filedDateTo" class="block text-sm font-medium text-gray-700">"Filed Date (to)"</label>
                        <DatePicker
                            attr:id="filedDateTo"
                            attr:name="filed_date_to"
                            value=filed_date_to
                            class="w-full"
                        />
                    </Space>
                 </div>
                 <Space vertical=true class="w-full">
                     <label for="causeOfAction" class="block text-sm font-medium text-gray-700">"Cause of Action"</label>
                     <MultiSelect
                         value=cause_of_action_value
                         options=cause_of_action_options
                         class="w-full"
                     />
                 </Space>


                     <Space vertical=true class="w-full">
                            <label for="nature_suit" class="block text-sm font-medium text-gray-700">"Nature of Suit"</label>
                            <MultiSelect
                                attr:name="nature_suit"
                                value=nature_suit_value
                                options=nature_suit_options
                                class="w-full"
                            />
                        </Space>

                            <Space vertical=true class="w-full">
                                <label for="lastBusinessName" class="block text-sm font-medium text-gray-700">"Last/Business Name"</label>
                                <Input
                                    variant=InputVariant::Text
                                    attr:id="lastBusinessName"
                                    attr:name="last_business_name"
                                    value=last_business_name
                                    placeholder="Lastname/business name"
                                    class="w-full"
                                >
                                    <InputPrefix slot>
                                        <Icon icon=icondata::BiBuildingRegular class="text-gray-400"/>
                                    </InputPrefix>
                                </Input>
                                <Checkbox
                                    value=exact_matches_only
                                    attr:id="exact_matches_only"
                                    attr:name="exact_matches_only"
                                    class="mt-2"
                                >
                                    "Exact matches only"
                                </Checkbox>
                            </Space>

                            <div class="grid grid-cols-1 gap-6 md:grid-cols-3">
                                 <Space vertical=true class="w-full">
                                     <label for="firstName" class="block text-sm font-medium text-gray-700">"First Name"</label>
                                     <Input
                                         variant=InputVariant::Text
                                         attr:id="firstName"
                                         attr:name="first_name"
                                         value=first_name
                                         placeholder="Type Firstname"
                                         class="w-full"
                                     >
                                         <InputPrefix slot>
                                             <Icon icon=icondata::OcProjectLg class="text-gray-400"/>
                                         </InputPrefix>
                                     </Input>
                                 </Space>
                                 <Space vertical=true class="w-full">
                                     <label for="middleName" class="block text-sm font-medium text-gray-700">"Middle Name"</label>
                                     <Input
                                         variant=InputVariant::Text
                                         attr:id="middleName"
                                         attr:name="middle_name"
                                         value=middle_name
                                         placeholder="Type Middlename"
                                         class="w-full"
                                     >
                                         <InputPrefix slot>
                                             <Icon icon=icondata::OcProjectLg class="text-gray-400"/>
                                         </InputPrefix>
                                     </Input>
                                 </Space>
                                 <Space vertical=true class="w-full">
                                     <label for="type_value" class="block text-sm font-medium text-gray-700">"Select Type"</label>
                                     <Select
                                         value=type_value
                                         options=type_options
                                         class="w-full"
                                     />
                                 </Space>
                             </div>

                    <div class="flex justify-end space-x-4">
                        <Button class="rounded-lg bg-blue-600 px-6 py-3 font-bold text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">"Run Query"</Button>
                        <Button class="rounded-lg bg-gray-600 px-6 py-3 font-bold text-white hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2">"Clear"</Button>
                    </div>
                </ActionForm>
                {move || {
                    let result = search_action.value();
                    view! {
                        <div>
                            {move || match result.get() {
                                None => view! { <div>"No search performed yet"</div> },
                                Some(Ok(cases)) => view! {
                                    <div class="mt-8">
                                        <h2 class="mb-4 text-2xl font-bold">"Search Results"</h2>
                                        <p class="mb-4">"Found " {cases.len()} " cases"</p>
                                        <SearchResultsTable results=cases />
                                    </div>
                                },
                                Some(Err(e)) => view! { <div class="text-red-500">"Error: " {e.to_string()}</div> },
                            }}
                        </div>
                    }
                }}
            </div>
        </div>
        </Default_Layout>
    }
}

#[component]
pub fn SearchResultsTable(results: Vec<CaseSearchCriteria>) -> impl IntoView {
    view! {
        <Table class="w-full border-collapse border border-gray-300">
            <thead>
                <tr class="bg-gray-100">
                    <th class="border border-gray-300 px-4 py-2">"Case Number"</th>
                    <th class="border border-gray-300 px-4 py-2">"Status"</th>
                    <th>"Filed Date"</th>
                    <th>"Last Entry Date"</th>
                    <th>"Cause of Action"</th>
                    <th>"Nature of Suit"</th>
                    <th>"Last/Business Name"</th>
                    <th>"First Name"</th>
                    <th>"Middle Name"</th>
                    <th>"Type"</th>
                </tr>
            </thead>
            <tbody>
                {results.into_iter().map(|case| view! {
                    <tr>
                        <td>{case.case_number.unwrap_or_default()}</td>
                        <td>{case.case_status.unwrap_or_default()}</td>
                        <td>{case.filed_date_from.unwrap_or_default()}</td>
                        <td>{case.last_entry_date_from.unwrap_or_default()}</td>
                        <td>{case.cause_of_action.unwrap_or_default()}</td>
                        <td>{case.nature_suit.map(|suits| suits.join(", ")).unwrap_or_default()}</td>
                        <td>{case.last_business_name.unwrap_or_default()}</td>
                        <td>{case.first_name.unwrap_or_default()}</td>
                        <td>{case.middle_name.unwrap_or_default()}</td>
                        <td>{case.type_field.unwrap_or_default()}</td>
                    </tr>
                }).collect_view()}
            </tbody>
        </Table>
    }
}

#[server(SearchCasesAction, "/api/search_cases")]
pub async fn search_cases_action(
    case_number: Option<String>,
    case_status: Option<String>,
    filed_date_from: Option<String>,
    filed_date_to: Option<String>,
    last_entry_date_from: Option<String>,
    last_entry_date_to: Option<String>,
    cause_of_action: Option<String>,
    nature_suit: Option<Vec<String>>,
    last_business_name: Option<String>,
    first_name: Option<String>,
    middle_name: Option<String>,
    type_field: Option<String>,
    exact_matches_only: Option<bool>,
) -> Result<Vec<CaseSearchCriteria>, ServerFnError> {


    let mock_cases = vec![
        CaseSearchCriteria {
            case_number: Some("2021-CV-001".to_string()),
            case_status: Some("Open".to_string()),
            filed_date_from: Some("2021-01-15".to_string()),
            filed_date_to: None,
            last_entry_date_from: Some("2023-05-20".to_string()),
            last_entry_date_to: None,
            cause_of_action: Some("05:0552".to_string()),
            nature_suit: Some(vec!["110".to_string()]),
            last_business_name: Some("Smith Corp".to_string()),
            first_name: Some("John".to_string()),
            middle_name: Some("A".to_string()),
            type_field: Some("Party".to_string()),
            exact_matches_only: Some(false),
        },
        // Add more mock cases here...
    ];



    Ok(mock_cases)
}
