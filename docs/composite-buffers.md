# Composite Buffers

## Requirements

- From the editor's perspective this should feel similar to opening a file, so the underlying abstraction should expose a buffer-like API, e.g.:
  - You CAN create anchors at any location and an appropriate anchor should be created using an actual `Buffer` instance
  - You CAN edit at any location and an appropriate edit should be proxied to an actual `Buffer` instance
  - You CAN create transactions across buffers, so multi-cursor editing should be possible and undoing should perform an undo across all tracked buffers that have undergone an edit.
  - You CAN'T create a selection that straddles across two buffers (but this seems like an editor concern rather than something we should bake into the abstraction)
  - Similarly, we should panic if `edit(range)` is called with a `range` that spans two or more buffers. The editor should never get into such situation due the point above.
- You can collaboratively add and remove individual buffer excerpts at any location, so it most likely is a sequence-CRDT
  - The outer CRDT is modeled as a sequence of excerpts which can be addressed locally via an offset or associated with anchors (versioned offsets)
  - These offsets can be used to retrieve excerpt metadata (the tracked buffer and anchor range)
- You can add a buffer + an anchor range into the composite buffer and it should be possible to extend or shrink that range after adding the buffer (this is to expand/shrink context lines)
- As edits happen in the underlying buffers, we should propagate them up for:
  - The composite buffer to keep its internal index up-to-date
  - The composite buffer to acquire a snapshot and store it in a `SumTree` that contains the latest snapshot for all the tracked buffers (so that it's efficient to call `CompositeBuffer::snapshot` when there are lots of buffers)
  - The display layers (wraps, folds, tabs) to keep their indices up-to-date
- Some APIs that are currently global, like `Buffer::language()` probably need to become `Buffer::language(position)`, which is okay and actually useful for when we'll have multiple languages even for a single buffer (e.g., markdown).
- Rows can be reported in a non-contiguous fashion given that each buffer has its own region that it tracks
- Each buffer should have its own header, so:
  - Have an API to report those locations (probably the current buffer rows could be adapted to report them)
  - Note how each header will affect the scroll height, so maybe the editor could inject a block line for each header or delete them if the buffer is removed from the composite buffer
  - Allow `BlockMap` to be associated with excerpt anchors in addition to text-level anchor
- `AnchorRangeMap`s could be represented as:
  ```rust
  struct AnchorRangeMap {
    ranges: Vec<(ExcerptOffset, BufferAnchorRangeMap)>>,
    version: clock::Global
  }
  ```
- `Item`-specific features:
  - `is_dirty`: we could OR each `Buffer::is_dirty`
  - `has_conflict`: same
  - `save`: I guess they save the buffer under the cursor? Or should it save all the underlying buffers at once? Whatever we do should be consistent with `is_dirty` and maybe it makes sense for now to save everything
  - `title`: each composite buffer will also store a user-defined string upon creation (e.g., References: `symbol-name`, Search: `query`, Diagnostics)
  - `save_as`: disabled maybe?
