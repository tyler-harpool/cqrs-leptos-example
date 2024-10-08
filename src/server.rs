use leptos_spin::{render_best_match_to_stream, RouteTable, server_fn::register_explicit};
use spin_sdk::http::{IncomingRequest, ResponseOutparam};
use spin_sdk::http_component;

#[http_component]
async fn handle_l3xodus(req: IncomingRequest, resp_out: ResponseOutparam) {
    let mut conf = leptos::get_configuration(None).await.unwrap();
    conf.leptos_options.output_name = "l3xodus".to_owned();

    // Register server functions
    register_explicit::<crate::pages::home::IncrementCount>();
    register_explicit::<crate::pages::home::GetCount>();
    register_explicit::<crate::application::queries::get_count::GetCount>();
    register_explicit::<crate::application::commands::increment_counter::IncrementCounter>();    let app_router = crate::routes::AppRouter;
    register_explicit::<crate::pages::case_management::new_case::NewCase>();
    register_explicit::<crate::pages::case_management::search::SearchCasesAction>();

    register_explicit::<crate::pages::case_management::view_case::GetCaseDetails>();

    let mut routes = RouteTable::build(app_router);
    routes.add_server_fn_prefix("/api").unwrap();

    render_best_match_to_stream(req, resp_out, &routes, app_router, &conf.leptos_options).await
}
