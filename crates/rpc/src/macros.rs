#[macro_export]
macro_rules! messages {
    ($(($name:ident, $priority:ident)),* $(,)?) => {
        pub fn build_typed_envelope(sender_id: ConnectionId, envelope: Envelope) -> Option<Box<dyn AnyTypedEnvelope>> {
            match envelope.payload {
                $(Some(envelope::Payload::$name(payload)) => {
                    Some(Box::new(TypedEnvelope {
                        sender_id,
                        original_sender_id: envelope.original_sender_id.map(|original_sender| PeerId {
                            owner_id: original_sender.owner_id,
                            id: original_sender.id
                        }),
                        message_id: envelope.id,
                        payload,
                    }))
                }, )*
                _ => None
            }
        }

        $(
            impl EnvelopedMessage for $name {
                const NAME: &'static str = std::stringify!($name);
                const PRIORITY: MessagePriority = MessagePriority::$priority;

                fn into_envelope(
                    self,
                    id: u32,
                    response_data: Option<(u32, bool)>,
                    original_sender_id: Option<PeerId>,
                ) -> Envelope {
                    Envelope {
                        id,
                        response_data: response_data.map(|(responding_to, is_last_message)| ResponseData {
                            responding_to, is_last_message
                        }),
                        original_sender_id,
                        payload: Some(envelope::Payload::$name(self)),
                    }
                }

                fn from_envelope(envelope: Envelope) -> Option<Self> {
                    if let Some(envelope::Payload::$name(msg)) = envelope.payload {
                        Some(msg)
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! request_messages {
    ($(($request_name:ident, $response_name:ident)),* $(,)?) => {
        $(impl RequestMessage for $request_name {
            type Response = $response_name;
        })*
    };
}

#[macro_export]
macro_rules! entity_messages {
    ($id_field:ident, $($name:ident),* $(,)?) => {
        $(impl EntityMessage for $name {
            fn remote_entity_id(&self) -> u64 {
                self.$id_field
            }
        })*
    };
}
