use leptos::*;

#[server]
pub async fn increment_counter(key: String) -> Result<(), ServerFnError> {
    use crate::application::services::counter_service::CounterService;
    let service = CounterService::new().map_err(|e| ServerFnError::new(e))?;
    service.increment_counter(&key).await.map_err(|e| ServerFnError::new(e))
}
