# Composite Buffers

## Requirements

- From the editor's perspective this should feel similar to opening a file, so the underlying abstraction should expose a buffer-like API, e.g.:
  - You CAN create anchors at any location and an appropriate anchor should be created using an actual `Buffer` instance
  - You CAN edit at any location and an appropriate edit should be proxied to an actual `Buffer` instance
  - You CAN'T create a selection that straddles across two buffers (but this seems like an editor concern rather than something we should bake into the abstraction)
- You can collaboratively add and remove individual buffers at any location, so it most likely is a sequence-CRDT
- You can add a buffer + an anchor range into that buffer and it should be possible to extend or shrink that range after adding the buffer (this is to expand/shrink context lines)
- As edits happen in the underlying buffers, we should propagate them up for:
  - The composite buffer to keep its internal index up-to-date
  - The display layers (wraps, folds, tabs) to keep their indices up-to-date
- Some APIs that are currently global, like `Buffer::language()` probably need to become `Buffer::language(position)`, which is okay and actually useful for when we'll have multiple languages even for a single buffer (e.g., markdown).
- Rows can be reported in a non-contiguous fashion given that each buffer has its own region that it tracks
- What about item-related features?
  - `is_dirty`: we could OR each `Buffer::is_dirty`
  - `has_conflict`: same
  - `save`: I guess they save the buffer under the cursor? Or should it save all the underlying buffers at once? Whatever we do should be consistent with `is_dirty` and maybe it makes sense for now to save everything
  - `title`: each composite buffer will also store a user-defined string upon creation (e.g., References: `symbol-name`, Search: `query`, Diagnostics)
  - `save_as`: disabled maybe?
