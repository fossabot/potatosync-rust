use chrono::{DateTime, FixedOffset, Local, SecondsFormat, Utc};
use chrono::serde::ts_milliseconds::*;
use diesel::prelude::*;
use serde_derive::*;

use crate::schema::notes;
use crate::status_response::StatusResponse;

/// General Note struct used for retrieving from db and updating notes
#[table_name = "notes"]
#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset, PartialEq, Clone)]
#[primary_key(note_id, account_id)]
// {"note_id": 1, "title": "Test", "content": "Test", "is_starred": false, "date": "2020-03-19T14:21:06.275Z", "color": 0, "image_url":null, "is_list": false, "list_parse_string":null, "reminders":null, "hide_content": false, "pin":null, "password":null, "is_deleted": false, "is_archived": false}
pub struct Note {
    pub note_id: i32,
    pub account_id: i32,
    pub title: String,
    pub content: String,
    pub style_json: String,
    pub starred: bool,
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub creation_date: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub last_modify_date: DateTime<Utc>,
    pub color: i32,
    pub images: String,
    pub list: bool,
    pub list_content: String,
    pub reminders: String,
    pub hide_content: bool,
    pub lock_note: bool,
    pub uses_biometrics: bool,
    pub deleted: bool,
    pub archived: bool,
    pub synced: bool,
}

/// Struct used for inserting new notes in the db. Note the missing note_id since it will be provided by the db.
#[table_name = "notes"]
#[derive(Queryable, Serialize, Deserialize, Insertable)]
pub struct NewNote {
    pub account_id: i32,
    pub title: String,
    pub content: String,
    pub style_json: String,
    pub starred: bool,
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub creation_date: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub last_modify_date: DateTime<Utc>,
    pub color: i32,
    pub images: String,
    pub list: bool,
    pub list_content: String,
    pub reminders: String,
    pub hide_content: bool,
    pub lock_note: bool,
    pub uses_biometrics: bool,
    pub deleted: bool,
    pub archived: bool,
    pub synced: bool,
}

/// Note as provided by the client when saving. Note the missing account_id since the client doesnt know the id.
#[table_name = "notes"]
#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
pub struct SavingNote {
    pub note_id: i32,
    pub title: String,
    pub content: String,
    pub style_json: String,
    pub starred: bool,
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub creation_date: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub last_modify_date: DateTime<Utc>,
    pub color: i32,
    pub images: String,
    pub list: bool,
    pub list_content: String,
    pub reminders: String,
    pub hide_content: bool,
    pub lock_note: bool,
    pub uses_biometrics: bool,
    pub deleted: bool,
    pub archived: bool,
    pub synced: bool,
}

impl SavingNote {
    /// Convert to Note by adding account_id
    pub fn to_note(&self, account_id: i32) -> Note {
        Note {
            note_id: self.note_id,
            account_id,
            title: self.title.clone(),
            content: self.content.clone(),
            style_json: self.style_json.clone(),
            starred: self.starred,
            creation_date: self.creation_date.clone(),
            last_modify_date: self.last_modify_date.clone(),
            color: self.color,
            images: self.images.clone(),
            list: self.list,
            list_content: self.list_content.clone(),
            reminders: self.reminders.clone(),
            hide_content: self.hide_content,
            lock_note: self.lock_note,
            uses_biometrics: self.uses_biometrics,
            deleted: self.deleted,
            archived: self.archived,
            synced: self.synced,
        }
    }
}

/// Struct used when client needs to specify certain note_id
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct NoteId {
    pub(crate) note_id: i32,
}

/// Struct used when client needs to specify last_updated timestamp
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct NoteLastUpdated {
    pub(crate) last_updated: String
}

/// Struct used when server needs to return list of notes along with status of request
#[derive(Serialize, Deserialize)]
pub struct NoteResponse {
    pub(crate) message: String,
    pub(crate) status: bool,
    pub(crate) notes: Vec<Note>,
}

impl ToString for NoteResponse {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::db;

    use super::*;
}