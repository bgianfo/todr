//! This module implements the todr command execution logic.

use std::cmp::Ordering;
use std::collections::HashMap;

use reqwest;

// Use our internal types module.
use renderer;
use types;
use config::Configuration;

// Endpoint for REST communication with the Todoist.
static TODOIST_API: &'static str = "https://todoist.com/API/v7/sync";

/// Used to specify what resources to fetch from the server.
/// It should be a JSON-encoded array of strings.
///
/// Here is a list of available resource types:
/// -  labels,
/// -  projects,
/// -  items,
/// -  notes,
/// -  filters,
/// -  reminders,
/// -  locations,
/// -  user,
/// -  `live_notifications`,
/// -  collaborators,
/// - `notification_settings`
///
enum TodrResourceType {
    Items,
    Projects,
}

fn to_resource_type(resource_type: &TodrResourceType) -> String {
    let resource = match resource_type {
        TodrResourceType::Items => String::from("items"),
        TodrResourceType::Projects => String::from("projects"),
    };

    format!("[\"{}\"]", &resource)
}

fn execute_request(resource_type: &TodrResourceType) -> Result<reqwest::Response, reqwest::Error> {
    // Fetch the token from the users environment.
    let auth_token = Configuration::api_token();

    // Map the resource type to the proper string.
    let resource_string = to_resource_type(resource_type);

    // Setup the request parameters.
    //
    // See the API documentation available for the full
    // list of parameters, their values and what they do.
    //
    // Link: https://developer.todoist.com/sync/v8/?shell
    //
    let sync_token: String = "*".to_string();
    let all_data: String = "true".to_string();
    let mut params = HashMap::new();
    params.insert("token", &auth_token);
    params.insert("sync_token", &sync_token);
    params.insert("resource_types", &resource_string);
    params.insert("all_data", &all_data);

    let client = reqwest::Client::new();

    // Issue the request.
    client.get(TODOIST_API).query(&params).send()
}

//
// Request response handler implementations.
//

fn process_error(error: &reqwest::Error) {
    println!("{}", error);
}

fn common_response_handler(response: &mut reqwest::Response) {
    // Only print these traces in debug mode.
    if cfg!(debug_assertions) {
        println!("Headers:\n{:?}", response.headers());
        println!("Body:\n{:?}", response.text());
    }
}

fn process_response_items(mut response: reqwest::Response) {
    common_response_handler(&mut response);

    let sync_state: types::SyncStruct = response
        .json()
        .expect("Failed to de-serialize JSON response");

    let mut items = sync_state.items.expect("Failed to parse items JSON");

    // Sort the items by their server order.
    let custom_sort = |a: &types::ItemStruct, b: &types::ItemStruct| {
        let item_order_sort = a.item_order.cmp(&b.item_order);
        let project_order_sort = a.project_id.cmp(&b.project_id);

        // item_order values are only unique per project, so if we want to
        // display items in the correct order we need to sort by project
        // groupings first, and then sort by item order.
        if project_order_sort == Ordering::Equal {
            item_order_sort
        } else {
            project_order_sort
        }
    };

    items.sort_by(custom_sort);

    for item in items {
        renderer::render_item(&item);
    }
}

fn process_response_projects(mut response: reqwest::Response) {
    common_response_handler(&mut response);

    let sync_state: types::SyncStruct = response
        .json()
        .expect("Failed to deserialize json response");

    // Sort the items by their server order.
    //
    let mut projects = sync_state.projects.expect("Failed to parse projects JSON");
    projects.sort_by_key(|p| p.item_order);

    for project in projects {
        renderer::render_project(&project);
    }
}

//
// Command handler implementations.
//

pub fn items_command() {
    let response = execute_request(&TodrResourceType::Items);

    match response {
        Ok(r) => process_response_items(r),
        Err(e) => process_error(&e),
    }
}

pub fn projects_command() {
    let response = execute_request(&TodrResourceType::Projects);

    match response {
        Ok(r) => process_response_projects(r),
        Err(e) => process_error(&e),
    }
}

pub fn help_command() {
    println!();
    println!("Commands:");
    println!();
    println!("  h | help  - This help message");
    println!();
    println!("  i | items - List all active todo items.");
    println!();
    println!("  p | projs - List all active projects.");
    println!();
    println!("  q | quit  - Exit the application.");
    println!();
}

pub fn unknown_command(command: &str) {
    println!("Unknown Command: {}", command);
}
