use leptos::*;
use leptos_router::*;
use leptos_meta::Style;
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
      <Layout class="app-layout" has_sider=true>
          <LayoutSider class="global-nav">
          <Menu value default_expanded_keys=vec![String::from("dashboard")]>
              <MenuItem key="dashboard" icon=icondata::AiDashboardOutlined label="Dashboard"/>
              <MenuItem icon=icondata::AiFileOutlined key="cases" label="Cases">
                  <a href="/newpage"><MenuItem key="new-case" label="New Case"/></a>
                  <MenuItem key="search-cases" label="Search Cases"/>
                  <MenuItem key="my-cases" label="My Cases"/>
                  <MenuItem key="all-cases" label="All Cases"/>
              </MenuItem>
              <MenuItem icon=icondata::AiCalendarOutlined key="calendar" label="Calendar">
                  <MenuItem key="my-schedule" label="My Schedule"/>
                  <MenuItem key="court-calendar" label="Court Calendar"/>
                  <MenuItem key="schedule-hearing" label="Schedule Hearing"/>
              </MenuItem>
              <MenuItem icon=icondata::AiFileTextOutlined key="documents" label="Documents">
                  <MenuItem key="file-new" label="File New Document"/>
                  <MenuItem key="doc-search" label="Document Search"/>
                  <MenuItem key="recent-filings" label="Recent Filings"/>
              </MenuItem>
              <MenuItem icon=icondata::AiTeamOutlined key="parties" label="Parties">
                  <MenuItem key="add-party" label="Add Party"/>
                  <MenuItem key="party-search" label="Party Search"/>
                  <MenuItem key="party-management" label="Party Management"/>
              </MenuItem>
              <MenuItem icon=icondata::AiOrderedListOutlined key="docket" label="Docket">
                  <MenuItem key="view-docket" label="View Docket"/>
                  <MenuItem key="docket-entries" label="Docket Entries"/>
              </MenuItem>
              <MenuItem icon=icondata::AiBarChartOutlined key="reports" label="Reports">
                  <MenuItem key="case-reports" label="Case Reports"/>
                  <MenuItem key="statistical-reports" label="Statistical Reports"/>
                  <MenuItem key="financial-reports" label="Financial Reports"/>
              </MenuItem>
              <MenuItem icon=icondata::AiSettingOutlined key="admin" label="Administration"/>
              <MenuItem icon=icondata::AiQuestionCircleOutlined key="help" label="Help & Support"/>
              <MenuItem icon=icondata::AiUserOutlined key="account" label="My Account"/>
          </Menu>
          </LayoutSider>
          <LayoutSider class="action-sidebar">
              <h2>"Action Menu"</h2>
              <Menu>
                  <MenuItem key="case-actions" label="Case Actions">
                      <MenuItem key="update-status" label="Update Case Status"/>
                      <MenuItem key="add-note" label="Add Note to Case"/>
                      <MenuItem key="set-hearing" label="Set Next Hearing Date"/>
                      <MenuItem key="generate-report" label="Generate Case Report"/>
                      <MenuItem key="assign-case" label="Assign to Judge/Staff"/>
                  </MenuItem>
                  <MenuItem key="document-actions" label="Document Actions">
                      <MenuItem key="upload-doc" label="Upload New Document"/>
                      <MenuItem key="edit-doc" label="Edit Document Details"/>
                      <MenuItem key="download-doc" label="Download Document"/>
                      <MenuItem key="share-doc" label="Share Document"/>
                      <MenuItem key="request-signature" label="Request Signature"/>
                  </MenuItem>
                  <MenuItem key="calendar-actions" label="Calendar Actions">
                      <MenuItem key="add-event" label="Add New Event"/>
                      <MenuItem key="reschedule" label="Reschedule Selected Event"/>
                      <MenuItem key="cancel-event" label="Cancel Event"/>
                      <MenuItem key="send-reminder" label="Send Event Reminder"/>
                  </MenuItem>
                  <MenuItem key="party-actions" label="Party Actions">
                      <MenuItem key="edit-party" label="Edit Party Details"/>
                      <MenuItem key="add-related" label="Add Related Party"/>
                      <MenuItem key="view-history" label="View Case History"/>
                      <MenuItem key="send-notification" label="Send Notification"/>
                  </MenuItem>
              </Menu>
          </LayoutSider>
          <Layout class="main-content">
              <LayoutHeader>
                 <h1>"Federal Court Case Management System"</h1>
              </LayoutHeader>
              <Layout class="content-area">
                <ActionForm action=increment_count>
                    <input type="hidden" name="key" value=counter_key />
                    <Button variant=ButtonVariant::Primary>"Click Me: " {move || count.get()}</Button>
                </ActionForm>
              </Layout>
          </Layout>
      </Layout>
      <Style>
          "
          :root {
              --primary-color: #003366;
              --secondary-color: #336699;
              --accent-color: #F0F7FF;
              --text-color: #333333;
              --background-color: #FFFFFF;
              --sidebar-bg: #E6EFF7;
              --header-bg: #003366;
              --button-bg: #E6EFF7;
              --button-text: #003366;
              --button-hover: #D0E3F5;
          }

          body {
              font-family: 'Arial', sans-serif;
              color: var(--text-color);
              background-color: var(--background-color);
              line-height: 1.6;
              font-size: 16px;
          }


          .action-sidebar {
              background-color: var(--sidebar-bg);
              padding: 1rem;
              overflow-y: auto;
          }

          .action-section {
              margin-bottom: 1.5rem;
          }

          .action-section h3 {
              font-size: 0.9rem;
              color: var(--primary-color);
              margin-bottom: 0.5rem;
              border-bottom: 1px solid var(--primary-color);
              padding-bottom: 0.25rem;
          }

          .action-button {
              display: block;
              width: 100%;
              padding: 0.5rem;
              margin-bottom: 0.5rem;
              background-color: var(--button-bg);
              color: var(--button-text);
              border: none;
              border-radius: 4px;
              font-size: 0.9rem;
              text-align: left;
              cursor: pointer;
              transition: background-color 0.2s ease;
          }

          .action-button:hover {
              background-color: var(--button-hover);
          }

          .main-content {
              display: flex;
              flex-direction: column;
          }

          .main-content > .thaw-layout-header {
              background-color: var(--header-bg);
              color: var(--background-color);
              padding: 1rem;
          }

          .content-area {
              padding: 2rem;
              overflow-y: auto;
          }

          h1, h2 {
              color: var(--primary-color);
          }

          h1 {
              font-size: 1.5rem;
          }

          h2 {
              font-size: 1.2rem;
              margin-bottom: 1rem;
          }

          .thaw-layout-sider {
              flex-grow: 0 !important;
          }

          .action-sidebar .thaw-menu {
              background-color: transparent;
          }

          .action-sidebar .thaw-menu-item {
              color: var(--text-color);
          }

          .action-sidebar .thaw-menu-submenu-title {
              font-weight: bold;
              color: var(--primary-color);
          }

          .action-sidebar .thaw-menu-item-content {
              padding-left: 1rem;
          }

          .action-sidebar .thaw-menu-item:hover {
              background-color: var(--button-hover);
          }


          "
      </Style>
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
