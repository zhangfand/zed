// data/conversations.ts

export interface Comment {
    id: number;
    author: string;
    message: string;
    timestamp: string;
}

export const sampleComments: Comment[] = [
    {
        id: 1,
        author: "Alice",
        message: "I came across the `refresh_room` function, which has parameters `room_id` and `new_server_id`. Can someone explain the role of `new_server_id`? Here's the function signature for reference:\n\n```rust\nfn refresh_room(room_id: RoomId, new_server_id: ServerId) -> Result<RoomGuard, Error>;\n```",
        timestamp: "2022-01-01T12:00:00Z",
    },
    {
        id: 2,
        author: "Bob",
        message: "new_server_id is used to identify the active server handling the request. It helps determine which participants are outdated and need to be removed. Check out its usage on [`db.rs:95`](#95).",
        timestamp: "2022-01-01T12:05:00Z",
    },
    {
        id: 3,
        author: "Charlie",
        message: "I see that a room_transaction is created for database operations within the function. Can someone elaborate on its purpose? Here's the relevant code snippet:\n\n```rust\nlet room_transaction = RoomTransaction::new(&conn)?;\n```",
        timestamp: "2022-01-01T12:10:00Z",
    },
    {
        id: 4,
        author: "Dave",
        message: "`room_transaction` ensures atomic execution of database operations, meaning all operations either succeed together or fail together. This maintains data consistency. More information can be found in the [official documentation](https://docs.rs/diesel/1.4.8/diesel/transaction/index.html).",
        timestamp: "2022-01-01T12:15:00Z",
    },
    {
        id: 5,
        author: "Eve",
        message: "I noticed the function returns a `RoomGuard` object. Can anyone explain its purpose and provide an example of its usage?",
        timestamp: "2022-01-01T12:15:00Z",
    },
    {
        id: 6,
        author: "Frank",
        message: "The RoomGuard object is used to manage and protect the state of a room. It provides an API for updating room properties and ensures that concurrent updates don't lead to inconsistencies. For instance, when a user joins a room, the RoomGuard can be used like this:\n\n```rust\nlet room_guard = refresh_room(room_id, server_id)?;\nroom_guard.add_participant(user_id)?;\n```",
        timestamp: "2022-01-01T12:20:00Z",
    },
    {
        id: 7,
        author: "Grace",
        message: "It's worth noting that `RoomGuard` also implements the Drop trait, which ensures that any pending changes are automatically committed to the database when the guard goes out of scope. This helps guarantee data consistency without the need for manual transaction management. You can see the implementation details in the `room_guard.rs` file.",
        timestamp: "2022-01-01T12:25:00Z",
    }
]
