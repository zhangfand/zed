##



### Linear issue tree


- Conversation pane
    - Display a channel's conversation when the channel is opened
    - Display short title of current conversation
    - Show/search recent conversations on title click
    - Can show conversations in #channels, as well as with one or more @humans and /assistants.
- Basic conversation editor
    - Conversation is divided into blocks
    - Each block displays its author(s)
    - When a block has multiple authors, their relative contribution levels are indicated.
    - When an author in a collaborative block is hovered, their current contributions are highlighted.
    - *Private compose mode* allows users to defer sharing of edits.
    - Hitting cmd-enter inserts a new block beneath the current block.
    - When privately composing a message, cmd-enter shares it.
    - When speaking with an assistant cmd-enter submits it to the model.
- Threads
    - Hitting tab at the start of a block moves it into a thread.
    - Threads can be pinned, so they show up in recent threads for a conversation.
    - By default, threads are displayed as the root message plus a summary.
    -

-----

## Essential Features:


- Channels panel
- Stacks
- Add user to call
- Project menu
    - Open projects
    - Recent projects
    - If streaming, projects shared in the call
- Share/currently sharing split button
    - Share/unshare the project by clicking on the main button
    - Right side launches popover:
        - The popover lists every project/terminal/screen you're sharing.
        - Below the list there is a button to stop sharing everything "Stop sharing all".
- Mute button
    - Only visible when on a call
    - Mute or unmute the user's audio on toggle
    - When not muted, you see a mic icon in the same color as icons are by default ("variant")
    - When muted, you see a red ("negative") mic icon with a slash through it and a background.
- Screen sharing button
    - When sharing, you see a blue ("accent") screen icon and it has a background
    - When not sharing, you see a screen icon in the same color as icons are by default ("variant") You do *not* see a screen with a line through it, just a normal screen.
- Hang up button



https://www.figma.com/file/pLq7dvhx2mFeWFOedXpUQ5/Project-%E2%80%93-Collaboration-UI?type=design&node-id=178%3A9017&t=OOgXyPZoMJZfwUdf-1



----

- Conversations view for chat
    - Can a conversation be a document in version 0.0.1 of CRDB?
    - Keep the entire structure in RAM
    - Periodically serialize to the database or R2?
    - Integrate it with assistants is also presented as a conversation... like a DM channel with a person, except a language model
- Iterate on the titlebar/sharing/collaboration UI
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
