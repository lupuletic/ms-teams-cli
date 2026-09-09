use serde::{Deserialize, Serialize};

use crate::models::message::{ChatMessageFrom, ItemBody};

/// Microsoft Graph Chat resource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<String>,
    /// Present only when the listing expands it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_preview: Option<ChatMessageInfo>,
}

/// Microsoft Graph chatMessageInfo resource: the preview of a chat's newest message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ItemBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<ChatMessageFrom>,
}

/// Request body for creating a new chat.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatCreateRequest {
    pub chat_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    pub members: Vec<crate::models::member::AddMemberRequest>,
}

/// Request body for updating a chat.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

/// Request body for hide/unhide chat.
#[derive(Debug, Clone, Serialize)]
pub struct ChatUserAction {
    pub user: ChatUserRef,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatUserRef {
    pub id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_roundtrip_omits_an_absent_preview() {
        let chat = Chat {
            id: Some("chat1".into()),
            topic: Some("Project X".into()),
            chat_type: Some("group".into()),
            last_updated_date_time: Some("2024-01-01T00:00:00Z".into()),
            last_message_preview: None,
        };
        let json = serde_json::to_string(&chat).unwrap();
        assert!(!json.contains("lastMessagePreview"));
        let parsed: Chat = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.topic.as_deref(), Some("Project X"));
    }

    #[test]
    fn chat_reads_the_expanded_last_message_preview() {
        // The shape Graph returns for `/me/chats?$expand=lastMessagePreview`.
        let json = serde_json::json!({
            "id": "19:abc@thread.v2",
            "topic": "Project X",
            "chatType": "group",
            "lastUpdatedDateTime": "2026-06-10T12:42:02.463Z",
            "lastMessagePreview": {
                "id": "1788942159890",
                "createdDateTime": "2026-09-09T08:22:39.89Z",
                "isDeleted": false,
                "messageType": "message",
                "body": {"contentType": "text", "content": "Hey both!"},
                "from": {"user": {"id": "u-1", "displayName": "Catalin Lupuleti", "userIdentityType": "aadUser"}}
            }
        });
        let chat: Chat = serde_json::from_value(json).unwrap();
        let out = serde_json::to_value(&chat).unwrap();
        assert_eq!(out["lastMessagePreview"]["id"], "1788942159890");
        let preview = chat.last_message_preview.expect("preview");
        assert_eq!(
            preview.created_date_time.as_deref(),
            Some("2026-09-09T08:22:39.89Z")
        );
        assert_eq!(preview.message_type.as_deref(), Some("message"));
        let from = preview
            .from
            .and_then(|f| f.user)
            .and_then(|u| u.display_name);
        assert_eq!(from.as_deref(), Some("Catalin Lupuleti"));
    }
}
