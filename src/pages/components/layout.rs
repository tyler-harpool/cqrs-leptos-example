// use crate::components::{Footer, Nav};

use leptos::*;
use thaw::{Layout, LayoutSider, Menu, MenuItem};

// use crate::components::{Footer, Nav, Action_Menu};

#[component]
pub fn Default_Layout(children: Children) -> impl IntoView {
    let value = create_rw_signal(String::from("o"));
    view! {

    <Layout class="app-layout" has_sider=true>
        <LayoutSider class="global-nav">
        <Menu value default_expanded_keys=vec![String::from("dashboard")]>
            <MenuItem key="lexodus" icon=icondata::AiDashboardOutlined label="Lexodus"/>
            <MenuItem icon=icondata::AiFileOutlined key="cases" label="Cases">
                <a href="/case-management/new">
                  <MenuItem key="new-case" label="New Case"/>
                </a>
                <a href="/case-management/search">
                  <MenuItem key="search-cases" label="Search Cases"/>
                </a>
                <MenuItem key="my-cases" label="My Cases"/>
                <a href="/case-management"><MenuItem key="list-cases" label="All Cases"/></a>
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

        <Layout class="main-content">
            <Layout class="content-area">
              {children()}
            </Layout>
        </Layout>
        <LayoutSider class="action-sidebar h-full">
            <h2>"Actions"</h2>
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
    </Layout>

    }
}
