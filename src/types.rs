///!
///! Define types which allow us to serialize/de-serialize
///! messages when interacting with the todoist API.
///!
///! Types manually converted from documentation here:
///!  - [Todoist API](https://developer.todoist.com/)
///!
///! Note: These are using v7 definition of the todoist API.
///!
///! We attribute each of the structs with:
///!
///! - `Serialize` and `Deserialize` impls for serialization.
///! - 'Debug ' impl for ... debugging.
///!
use serde::{Serialize, Deserialize};

/// Struct to be used for objects which we don't
/// yet support. It doesn't deserialize anything.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct NotYetUsedStruct {}


#[derive(Serialize, Deserialize, Debug)]
pub struct LabelStruct {}

// Automatically generate:
// - `Serialize` and `Deserialize` impls for serialization.
// - 'Debug ' impl for ... debugging.
#[derive(Serialize, Deserialize, Debug)]
pub struct ItemStruct {
    /// The id of the task.
    pub id: u64,

    /// The owner of the task.
    pub user_id: u64,

    /// The project that the task resides in.
    pub project_id: u64,

    /// The parent id of the task.
    pub parent_id: Option<u64>,

    /// The text of the task.
    pub content: String,

    /// The date of the task, added in free form text,
    /// for example it can be every day @ 10 (or null or an empty string if not set).
    pub date_string: String,

    /// The language of the date_string.
    /// Valid languages are: en, da, pl, zh, ko, de, pt, ja, it, fr, sv, ru, es, nl
    pub date_lang: String,

    /// The date of the task in the format:
    /// Mon 07 Aug 2006 12:34:56 +0000 (or null if not set).
    ///
    /// For all day task (i.e. task due “Today”),
    /// the time part will be set as xx:xx:59.
    pub due_date_utc: Option<String>,

    /// The priority of the task between: 1..4.
    /// - 1 is natural
    /// - 4 is very urgent
    pub priority: u8,

    /// The indent of the task between: 1..4.
    /// - 1 is top-level
    pub indent: u8,

    /// The order of the task inside the project.
    /// smallest value is placed at the top.
    pub item_order: u64,

    /// The order of the task inside the Today or Next 7 days view
    /// (a number, where the smallest value would place the task at the top).
    pub day_order: i64,

    /// Whether the task's sub-tasks are collapsed.
    pub collapsed: u8,

    /// The tasks label ids.
    pub labels: Vec<u64>,

    /// The id of the user who assigned the current task.
    /// Only makes sense for shared projects.
    pub assigned_by_uid: Option<u64>,

    /// The id of user who is responsible for the current task.
    /// Only makes sense for shared projects.
    pub responsible_uid: Option<u64>,

    /// Whether the task is marked as completed.
    pub checked: u8,

    /// Whether the task hsa been marked as completed is marked to be moved to history.
    pub in_history: u8,

    /// Whether the task is marked as deleted.
    pub is_deleted: u8,

    /// Whether the task is marked as archived.
    pub is_archived: u8,

    /// A special id for shared tasks. Can be ignored.
    pub sync_id: Option<u64>,

    /// The date when the task was created.
    pub date_added: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFeaturesStruct {
    karma_disabled: bool,

    restriction: u8,

    karma_vacation: bool,

    beta: u8,

    has_push_reminders: bool,

    dateist_inline_disabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeZoneStruct {
    /// Hours difference from GMT.
    hours: i8,

    /// Minutes difference from GMT.
    minutes: i8,

    /// Time difference from GMT as a string.
    gmt_string: String,

    /// Whether daylight saving time applies.
    is_dst: u8,

    /// The timezone name.
    timezone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserStruct {
    /// URL to the small size of the user avatar.
    pub avatar_small: String,

    /// The users avatar in a medium size.
    pub avatar_medium: String,

    /// The users avatar in a big size.
    pub avatar_big: String,

    /// The users avatar in yet another resolution.
    pub avatar_s640: String,

    /// The users start page.
    pub start_page: String,

    /// What features the user has set.
    pub features: UserFeaturesStruct,

    /// How many tasks the user has completed today.
    pub completed_today: u32,

    /// Is this a premium user.
    pub is_premium: bool,

    /// If projects should be shown in oldest dates first (value = 0)
    /// or oldest dates last (value = 1)
    pub sort_order: u8,

    /// The users full name.
    pub full_name: String,

    /// The default time in minutes for the automatic reminders.
    pub auto_reminder: u32,

    /// The date the user joined.
    pub join_date: Option<String>,

    /// The users identification number.
    pub id: u64,

    /// The day of the next week, that tasks will be postponed to.
    /// (Between 1 and y, where Monday is 1 and 7 is Sunday)
    pub next_week: u8,

    /// The total number tasks the user has completed.
    pub completed_count: u64,

    /// The daily goal of tasks.
    pub daily_goal: u32,

    /// The currently selected Todoist theme (between 0 and 10).
    pub theme: u8,

    /// The users email address.
    pub email: String,

    /// The first day of the week (between 1 (Monday) and 7 (Sunday))
    pub start_day: u8,

    /// Users timezone information.
    pub tz_info: TimeZoneStruct,

    /// Whether to use the DD-MM-YYYY date format (if set to 0),
    /// or the MM-DD-YYYY format (if set to 1).
    pub date_format: u8,

    /// The project the user has selected to use as their inbox.
    pub inbox_project: u64,

    /// Whether to use a 24h format such as 13:00 (if set to 0) when displaying time,
    /// or a 12h format such as 1:00pm (if set to 1).
    pub time_format: u8,

    /// The id of the user's avatar.
    pub image_id: String,

    /// The user's karma trend.
    pub karma_trend: String,

    /// The id o the user's business account.
    pub business_account_id: Option<u64>,

    /// The users mobile phone number.
    pub mobile_number: Option<String>,

    /// The users mobile host number.
    pub mobile_host: Option<String>,

    /// The date when the users's Premium subscrpition ends.
    pub premium_until: Option<String>,

    /// The users authentication token.
    pub token: String,

    /// The user's karma score.
    pub karma: u64,

    /// Whether the user is a business account admin.
    pub is_biz_admin: bool,

    /// The default reminder for the user. Reminders are only possible for Premium users.
    /// The default reminder can be one of the following: email to send reminders by email,
    /// mobile to send reminders to mobile devices via SMS, push to send reminders to smart
    /// devices using push notifications (one of the Android or iOS official clients must be
    /// installed on the client side to receive these notifications),
    /// no_default to turn off sending default reminders.
    pub default_reminder: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterStruct {
    /// The presentation order of the filter.
    item_order: u8,

    /// The name of the filter.
    name: String,

    /// The query specifier for this filter.
    query: String,

    /// The color to present this filter as.
    color: u8,

    /// Is the filter deleted.
    is_deleted: u8,

    /// The identifier of this filter.
    id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReminderStruct {
    /// The item id that this reminder is for.
    item_id: u64,

    /// The alias name for the location.
    name: String,

    /// The service to get the reminder for.
    service: String,

    /// The type of reminder, 'relative' for time-based reminder in minutes.
    /// 'absolute' for a time-based reminder with a specific time and date in the future.
    /// 'location' for a location-based reminder.
    /// type: String,

    /// The date of the task, added in free form text,
    /// for example it can be every day @ 10 (or null or an empty string if not set).
    date_string: String,

    /// The language of the date_string.
    /// Valid languages are: en, da, pl, zh, ko, de, pt, ja, it, fr, sv, ru, es, nl
    date_lang: String,

    /// Is the reminder deleted (0,1).
    is_deleted: u8,

    /// The identifier of this filter.
    id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectStruct {
    /// The name of the project.
    pub name: String,

    /// The id of the parent project.
    pub parent_id: Option<u64>,

    /// The color to present this filter as.
    pub color: u8,

    /// Whether the project's sub-projects are collapsed (0,1)
    pub collapsed: u8,

    /// This project is marked as the inbox project.
    pub inbox_project: Option<bool>,

    /// This project has more notes.
    pub has_more_notes: Option<bool>,

    /// The item order (0,1).
    pub item_order: u8,

    // The indent of the time (1..4) where 1 is top level.
    pub indent: u8,

    /// The identifier of this filter.
    pub id: u64,

    /// Is the project deleted (0,1).
    pub is_deleted: u8,

    /// Whether the project is marked as archived (0,1)
    pub is_archived: u8,
}

// Automatically generate:
// - `Serialize` and `Deserialize` impls for serialization.
// - 'Debug ' impl for ... debugging.
#[derive(Serialize, Deserialize, Debug)]
pub struct SyncStruct {
    /// Collaborators
    /// Note: Not yet used, so we don't parse it yet.
    pub collaborators: Option<Vec<NotYetUsedStruct>>,

    /// Collaborator states.
    /// Note: Not yet used, so we don't parse it yet.
    pub collaborator_states: Option<Vec<NotYetUsedStruct>>,

    // Specifies the order of items in daily agenda.
    pub day_orders: Option<NotYetUsedStruct>,

    /// An array of filters.
    pub filters: Option<Vec<FilterStruct>>,

    /// Whether the response contains all data (full sync), or incremental.
    pub full_sync: bool,

    /// An array of items.
    pub items: Option<Vec<ItemStruct>>,

    /// An array of labels.
    pub labels: Option<Vec<LabelStruct>>,

    /// An array of live notifications.
    pub live_notifications: Option<Vec<NotYetUsedStruct>>,

    /// The last live notification the user saw.
    /// Used for implementing unread notifications.
    pub live_notifications_last_read_id: Option<u64>,

    /// An array of notes.
    /// Note: Not yet used, so we don't parse it yet.
    pub notes: Option<Vec<NotYetUsedStruct>>,

    /// An array of projects.
    pub projects: Option<Vec<ProjectStruct>>,

    /// An array of reminders.
    pub reminders: Option<Vec<ReminderStruct>>,

    /// A new synchronization token.
    /// Used by the client on the next sync request to do incremental sync.
    pub sync_token: String,

    /// The users information.
    pub user: Option<UserStruct>,
}

#[cfg(test)]
use serde_json;

#[test]
fn item_deserialize_test() {

    let json_item = r#"{
                      "id": 33511505,
                      "user_id": 1_855_589,
                      "project_id": 128_501_470,
                      "content": "Task1",
                      "date_string": "",
                      "date_lang": "en",
                      "due_date_utc": null,
                      "indent": 1,
                      "priority": 1,
                      "item_order": 1,
                      "day_order": -1,
                      "collapsed": 0,
                      "labels": [12_839_231, 18_391_839],
                      "assigned_by_uid": 1_855_589,
                      "responsible_uid": null,
                      "checked": 0,
                      "in_history": 0,
                      "is_deleted": 0,
                      "is_archived": 0,
                      "sync_id": null,
                      "date_added": "Fri 26 Sep 2014 08:25:05 +0000"
                    }"#;

    let item: ItemStruct = serde_json::from_str(&json_item).unwrap();

    assert_eq!(item.id, 33_511_505);
    assert_eq!(item.user_id, 1_855_589);
    assert_eq!(item.content, "Task1");
    assert_eq!(item.date_string, "");
    assert_eq!(item.date_lang, "en");
    assert_eq!(item.due_date_utc, None);
    assert_eq!(item.indent, 1);
    assert_eq!(item.priority, 1);
    assert_eq!(item.item_order, 1);
    assert_eq!(item.day_order, -1);
    assert_eq!(item.collapsed, 0);
    assert_eq!(item.labels, vec![12_839_231, 18_391_839]);
    assert_eq!(item.assigned_by_uid.unwrap(), 1_855_589);
    assert_eq!(item.responsible_uid, None);
    assert_eq!(item.checked, 0);
    assert_eq!(item.in_history, 0);
    assert_eq!(item.is_deleted, 0);
    assert_eq!(item.is_archived, 0);
    assert_eq!(item.sync_id, None);
    assert_eq!(item.date_added.unwrap(), "Fri 26 Sep 2014 08:25:05 +0000");
}
