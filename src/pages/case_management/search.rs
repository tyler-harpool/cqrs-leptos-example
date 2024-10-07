
use leptos::*;
use leptos_router::ActionForm;
use thaw::{Checkbox, Input, InputVariant, RadioItem, Select, SelectOption, Space, RadioGroup};
use serde::{Serialize, Deserialize};
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
                        <div>
                            <label for="filedDateFrom" class="mb-2 block text-lg font-medium text-gray-700">"Filed Date (from)"</label>
                            <input type="date" id="filedDateFrom" class="block w-full rounded-lg border border-gray-300 p-3 focus:border-blue-500 focus:ring-blue-500" />
                        </div>
                        <div>
                            <label for="filedDateTo" class="mb-2 block text-lg font-medium text-gray-700">"Filed Date (to)"</label>
                            <input type="date" id="filedDateTo" class="block w-full rounded-lg border border-gray-300 p-3 focus:border-blue-500 focus:ring-blue-500" />
                        </div>
                    </div>

                    <div class="mb-6 grid grid-cols-1 gap-6 md:grid-cols-2">
                        <div>
                            <label for="lastEntryDateFrom" class="mb-2 block text-lg font-medium text-gray-700">"Last Entry Date (from)"</label>
                            <input type="date" id="lastEntryDateFrom" class="block w-full rounded-lg border border-gray-300 p-3 focus:border-blue-500 focus:ring-blue-500" />
                        </div>
                        <div>
                            <label for="lastEntryDateTo" class="mb-2 block text-lg font-medium text-gray-700">"Last Entry Date (to)"</label>
                            <input type="date" id="lastEntryDateTo" class="block w-full rounded-lg border border-gray-300 p-3 focus:border-blue-500 focus:ring-blue-500" />
                        </div>
                    </div>

                    <div class="mb-6">
                        <label for="causeOfAction" class="mb-2 block text-lg font-medium text-gray-700">"Cause of Action"</label>
                        <select id="causeOfAction" class="block w-full rounded-lg border border-gray-300 p-3 focus:border-blue-500 focus:ring-blue-500" size="5">
                            <option value="0">"0 (No cause code entered)"</option>
                            <option value="02:0431">"02:0431 (Federal Election Commission: Failure Enforce Compliance)"</option>
                            <option value="05:0552">"05:0552 (Freedom of Information Act)"</option>
                            <option value="07:0601">"07:0601 (USDA Condemnation)"</option>
                            <option value="12:2601">"12:2601 (Real Estate Settlement Procedures Act)"</option>
                            <option value="05:0552fi">"05:0552fi (05:552 Freedom of Information Act)"</option>
                            <option value="05:0552pa">"05:0552pa (05:552 Right to Privacy Act)"</option>
                            <option value="08:1105">"08:1105 (8:1105(a) Aliens: Habeas Corpus to Release INS Detainee)"</option>
                            <option value="28:1332al">"28:1332al (28:1332 Diversity-Airline Crash)"</option>
                        </select>
                    </div>

                    <div class="mb-6">
                        <label for="natureSuit" class="mb-2 block text-lg font-medium text-gray-700">"Nature of Suit"</label>
                        <select id="natureSuit" name="nature_suit" class="block w-full rounded-lg border border-gray-300 p-3 focus:border-blue-500 focus:ring-blue-500" multiple size="4">
                            <option></option>
                            <option value="0">"0 (zero)"</option>
                            <option value="110">"110 (Insurance)"</option>
                            <option value="120">"120 (Contract: Marine)"</option>
                            // ... (more options)
                        </select>
                    </div>

                    <div class="mb-6">
                        <label for="lastBusinessName" class="mb-2 block text-lg font-medium text-gray-700">"Last/Business Name"</label>
                        <input type="text" id="lastBusinessName" class="block w-full rounded-lg border border-gray-300 p-3 focus:border-blue-500 focus:ring-blue-500" />
                        <label class="mt-4 inline-flex items-center"> <input type="checkbox" id="exactMatchesOnly" class="mr-2 text-blue-500 focus:ring-blue-500" /> "Exact matches only" </label>
                    </div>

                    <div class="mb-6 grid grid-cols-1 gap-6 md:grid-cols-3">
                        <div>
                            <label for="firstName" class="mb-2 block text-lg font-medium text-gray-700">"First Name"</label>
                            <input type="text" id="firstName" class="block w-full rounded-lg border border-gray-300 p-3 focus:border-blue-500 focus:ring-blue-500" />
                        </div>
                        <div>
                            <label for="middleName" class="mb-2 block text-lg font-medium text-gray-700">"Middle Name"</label>
                            <input type="text" id="middleName" class="block w-full rounded-lg border border-gray-300 p-3 focus:border-blue-500 focus:ring-blue-500" />
                        </div>
                        <div>
                            <label for="type" class="mb-2 block text-lg font-medium text-gray-700">"Type"</label>
                            <select id="type" class="block w-full rounded-lg border border-gray-300 p-3 focus:border-blue-500 focus:ring-blue-500">
                                <option value="">"Select Type"</option>
                                <option value="aty">"Attorney"</option>
                                <option value="pty">"Party"</option>
                            </select>
                        </div>
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
