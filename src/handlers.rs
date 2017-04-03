//! This module implements the todr command exeuction logic.

use std::env;
use std::error::Error;
use ease; // ::{Url, Request};
use serde_json;

// Use our internal types module.
use types;
use render;

// Endpoint for REST communication with the Todoist.
static TODOIST_API: &'static str = "https://todoist.com/API/v7/sync";


/// Used to specify what resources to fetch from the server.
/// It should be a JSON-encoded array of strings.
///
/// Here is a list of avaialbe resource types:
/// -  labels,
/// -  projects,
/// -  items,
/// -  notes,
/// -  filters,
/// -  reminders,
/// -  locations,
/// -  user,
/// -  live_notifications,
/// -  collaborators,
/// -  notification_settings
///
enum TodrResourceType {
    All,
    Items,
    Labels,
    Projects,
    Reminders,
}

fn to_resource_type(resource_type: TodrResourceType) -> String {
    let resource = match resource_type {
        TodrResourceType::All => String::from("all"),
        TodrResourceType::Items => String::from("items"),
        TodrResourceType::Labels => String::from("labels"),
        TodrResourceType::Projects => String::from("projects"),
        TodrResourceType::Reminders => String::from("reminders"),
    };

    format!("[\"{}\"]", &resource)
}

fn execute_request(resource_type: TodrResourceType) -> Result<ease::Response, ease::Error> {
    // Fetch the token from the users environment.
    let auth_token = env::var("TODR_AUTHTOKEN").unwrap();

    // Map the resource type to the proper string.
    let resource_string = to_resource_type(resource_type);

    let url = ease::Url::parse(TODOIST_API).unwrap();

    ease::Request::new(url)
        .param("token", &auth_token)
        .param("sync_token", "*")
        .param("resource_types", &resource_string)
        .param("all_data", "true")
        .get()
}

//
// Request response handler implementations.
//

fn process_error(error: ease::Error) {
    match error {
        ease::Error::UnsuccessfulResponse(r) => println!("Response: {}", r.body),
        ease::Error::Json(e) => println!("{}", e.description()),
        ease::Error::Hyper(he) => println!("{}", he.description()),
    }
}

fn process_response_items(response: ease::Response) {

    let sync_state: types::SyncStruct = serde_json::from_str(&response.body).unwrap();

    // Sort the items by their server order.
    //
    let mut items = sync_state.items.unwrap();
    items.sort_by_key(|i| i.item_order);

    for item in items {
        render::render_item(item);
    }
}

fn process_response_projects(response: ease::Response) {
    let sync_state: types::SyncStruct = serde_json::from_str(&response.body).unwrap();

    // Sort the items by their server order.
    //
    let mut projects = sync_state.projects.unwrap();
    projects.sort_by_key(|p| p.parent_id);

    for project in projects {
        render::render_project(project);
    }
}

//
// Command handler implementations.
//

pub fn items_command() {
    let response = execute_request(TodrResourceType::Items);

    match response {
        Ok(r) => process_response_items(r),
        Err(e) => process_error(e),
    }
}

pub fn projects_command() {
    let response = execute_request(TodrResourceType::Projects);

    match response {
        Ok(r) => process_response_projects(r),
        Err(e) => process_error(e),
    }
}

pub fn help_command() {
    println!("");
    println!("Commands:");
    println!("");
    println!("  h | help  - This help message");
    println!("");
    println!("  i | items - List all active todo items.");
    println!("");
    println!("  p | proj  - List all active projects.");
    println!("");
    println!("  q | quit  - Exit the application.");
    println!("");
}

pub fn unknown_command(command: &str) {
    println!("Unknown Command: {}", command);
}
