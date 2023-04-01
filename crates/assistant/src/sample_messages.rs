use crate::{ListItem, Role};
use unindent::Unindent;

#[allow(dead_code)]
pub fn build_sample_messages() {
    vec![
        ListItem::Header(Role::User),
        ListItem::Message {
            content: r#"
                How can I add another message to this list in Rust?
            "#.unindent(),
            role: Role::User,
        },
        ListItem::CodeMessage {
            content: r#"
                let messages = vec![
                    Message {
                        text: "So... what do you think of this code.".to_owned(),
                        from_assistant: false,
                    },
                    Message {
                        text: "It's okay, for a human.".to_owned(),
                        from_assistant: true,
                    },
                ];
            "#.unindent(),
            role: Role::User,
            language: Some("rust".to_owned()),
        },
        ListItem::Message {
            content: "Thank you!".to_owned(),
            role: Role::User,
        },
        ListItem::Header(Role::Assistant),
        ListItem::Message {
            content: r#"
                You can add another message to the `messages` vector by creating a new `Message` struct and pushing it onto the vector using the `push` method.
                Here's an example:
            "#.unindent(),
            role: Role::Assistant,
        },
        ListItem::CodeMessage {
            content: r#"
                let new_message = Message {
                    text: "I think we can improve this code by using Rust macros.".to_owned(),
                    from_assistant: true,
                };

                messages.push(new_message);
            "#.unindent(),
            role: Role::Assistant,
            language: Some("rust".to_owned()),
        },
        ListItem::Message {
            content: r#"
                This creates a new `Message` struct with the text "I think we can improve this code by using Rust macros." and sets the `from_assistant` field to `true`. Then, it uses the `push` method to add the new message to the end of the `messages` vector.
            "#.unindent(),
            role: Role::Assistant,
        },
    ];
}
