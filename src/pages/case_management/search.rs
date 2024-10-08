
use leptos::*;
use leptos_router::ActionForm;
use thaw::{SelectLabel, MultiSelect, Tag, TagVariant, MultiSelectOption,Radio, DatePicker, Checkbox, Input, InputVariant, RadioItem, Select, SelectOption, Space, RadioGroup};
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
    pub nature_suit: Vec<String>,
    pub last_business_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub type_field: Option<String>,
    pub exact_matches_only: bool,
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
  let last_entry_date_from = create_rw_signal(None::<NaiveDate>);
  let last_entry_date_to = create_rw_signal(None::<NaiveDate>);
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
        <div class="min-h-screen  p-8">
            <div class="mx-auto max-w-2xl rounded-lg bg-white p-8 shadow-lg">
                <h2 class="mb-6 text-2xl font-extrabold text-gray-800">"Search Clues"</h2>

                 <ActionForm action=search_action>
                 <Space vertical=true>
                     <label for="caseNumber" class="mb-2 block text-lg font-medium text-gray-700">"Case Number"</label>
                     <Input
                         variant=InputVariant::Text
                         attr:id="caseNumber"
                         attr:name="case_number"
                         placeholder="Enter case number"
                         value=case_number
                     />
                 </Space>

                 <Space vertical=true>
                     <label class="mb-2 block text-lg font-medium text-gray-700">"Case Status:"</label>
                     <RadioGroup value=case_status>
                         <RadioItem key="open">"Open"</RadioItem>
                         <RadioItem key="closed">"Closed"</RadioItem>
                         <RadioItem key="all">"All"</RadioItem>
                     </RadioGroup>
                 </Space>

                 <div class="mb-6 grid grid-cols-1 gap-6 md:grid-cols-2">
                     <Space vertical=true>
                         <label for="filedDateFrom" class="mb-2 block text-lg font-medium text-gray-700">"Filed Date (from)"</label>
                         <DatePicker
                             attr:id="filedDateFrom"
                             attr:name="filed_date_from"
                             value=filed_date_from
                         />
                     </Space>
                     <Space vertical=true>
                         <label for="filedDateTo" class="mb-2 block text-lg font-medium text-gray-700">"Filed Date (to)"</label>
                         <DatePicker
                             attr:id="filedDateTo"
                             attr:name="filed_date_to"
                             value=filed_date_to
                         />
                     </Space>
                 </div>
                    <div class="mb-6">
                        <label for="causeOfAction" class="mb-2 block text-lg font-medium text-gray-700">"Cause of Action"</label>
                        <MultiSelect
                            value=cause_of_action_value
                            options=cause_of_action_options
                        />
                    </div>

                    <div class="mb-6">
                        <label for="natureSuit" class="mb-2 block text-lg font-medium text-gray-700">
                            "Nature of Suit"
                        </label>
                        <MultiSelect
                            value=nature_suit_value
                            options=nature_suit_options
                        />
                    </div>

                    <div class="mb-6">
                    <label for="lastBusinessName" class="mb-2 block text-lg font-medium text-gray-700">"Last/Business Name"</label>
                    <Input
                        variant=InputVariant::Text
                        attr:id="lastBusinessName"
                        attr:name="last_business_name"
                        value=last_business_name
                         placeholder="last/business name"
                    />
                    <Checkbox
                        value=exact_matches_only
                        attr:id="exactMatchesOnly"
                        attr:name="exact_matches_only"

                    >
                        "Exact matches only"
                    </Checkbox>
                    </div>

                    <div class="mb-6 grid grid-cols-1 gap-6 md:grid-cols-3">
                        <Space vertical=true>
                            <label for="firstName" class="mb-2 block text-lg font-medium text-gray-700">"First Name"</label>
                            <Input
                                variant=InputVariant::Text
                                attr:id="firstName"
                                attr:name="first_name"
                                value=first_name
                                placeholder="Type Firstname"
                            />
                        </Space>
                        <Space vertical=true>
                            <label for="middleName" class="mb-2 block text-lg font-medium text-gray-700">"Middle Name"</label>
                            <Input
                                variant=InputVariant::Text
                                attr:id="middleName"
                                attr:name="middle_name"
                                value=middle_name
                                placeholder="Type Middlename"
                            />
                        </Space>
                        <Space vertical=true>
                            <label for="type_value" class="mb-2 block text-lg font-medium text-gray-700">"Select Type"</label>
                            <Select
                                value=type_value
                                options=type_options

                            >


                            </Select>
                        </Space>
                    </div>

                    <div class="flex justify-end space-x-4">
                        <button type="button" class="rounded-lg bg-blue-600 px-6 py-3 font-bold text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">"Run Query"</button>
                        <button type="reset" class="rounded-lg bg-gray-600 px-6 py-3 font-bold text-white hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2">"Clear"</button>
                    </div>
                </ActionForm>
            </div>
        </div>
        </Default_Layout>
    }
}
#[server(SearchCasesAction, "/api/search_cases")]
pub async fn search_cases_action(criteria: CaseSearchCriteria) -> Result<Vec<CaseSearchCriteria>, ServerFnError> {
    // Implement your search logic here
    // For now, we'll just return an empty vector
    Ok(vec![])
}
