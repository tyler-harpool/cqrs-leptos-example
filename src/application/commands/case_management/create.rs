use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateCaseDto {
    pub case_number: String,
    pub title: String,
    pub filing_date: String,
    pub description: String,
}
pub async fn create_case(
    case_number: String,
    title: String,
    filing_date: String,
    description: String
) -> Result<CreateCaseDto, ServerFnError> {
    let case = CreateCaseDto {
        case_number,
        title,
        filing_date,
        description,
    };
    println!("Creating new case: {:?}", case);
    Ok(case)
}
