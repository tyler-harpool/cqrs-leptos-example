use leptos::*;

#[server]
pub async fn get_count(key: String) -> Result<u64, ServerFnError> {
    use crate::application::services::counter_service::CounterService;
    let service = CounterService::new().map_err(|e| ServerFnError::new(e))?;
    service.get_count(&key).await.map_err(|e| ServerFnError::new(e))
}
