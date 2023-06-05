## Essential Features:

Primitives added:
- Buffer in the sky
- Voice chat
- Channels / Conversations panel

- Conversations view for chat
    - Can a conversation be a document in version 0.0.1 of CRDB?
    - Keep the entire structure in RAM
    - Periodically serialize to the database or R2?
    - Integrate it with assistants is also presented as a conversation... like a DM channel with a person, except a language model
- Share UI 1.0
    - Screen sharing and audio controls in the top middle
    - Share button and collaborators on right
        - Click share button
            - Not streaming?
                - See channels, contacts
                - Search a Zed user by name and stream to them right away
                - Copy link, on click, create a new channel
            - Already streaming?
                - Do you want to share /project in #channel? -> Yes/No
        - Channels panel
            - On left. Displays channels, contacts, and assistants.
            - For each channel
                - Who is active in the text based discussion?
                - Who is streaming?
            - Copy link?
            - Collapse tree structure?
            - Create, join, and leave channels
            - Search for channels
        - Web UI:
            - Simplified, read-only overview of what's going on at Zed, with a
            download link and links to channels.
        - URLs for opening Zed channels `zed://`
- Authorization
    - Channel permissions
        - Access the channel
        - Edit other people's messages
        - Write in a streamed project
        - Need a ban or delete-all feature for channel moderation during livestreams
    - Audit log
- Voice
    - Controls in the middle
    - Part of LiveKit
- Integrated scheduling. Schedule a stream in advance. People can subscribe and be notified within Zed.
