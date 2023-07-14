I'm trying to make a simple but flexible cell element that can express most of what we'll need with a minimal ruleset.

A key idea is that any length can be flexible relative to its parent, in addition to being fixed. The flex factors control how much available space the length in question can consume from its parent.

```rs
enum Length {
    Flex {
        grow: f16,
        shrink: f16,
    },
    Fixed(f32),
}
```

Cell sizes are expressed in lengths. This allows a cell to grow or shrink to fill the available space in its parent.

```rs
enum Size {
    width: Length,
    height: Length,
}

struct CellStyle {
    // ...
    size: Size,
    min_size: Size,
    max_size: Size,
    // ...
}
```

But margin and padding are also expressed in terms of these potentially flexible lengths. This could allow an element to float itself to an edge by setting a flexible margin.

```rs
struct CellStyle {
    // ...
    margin: Edges<Length>,
    padding: Edges<Length>,
    // ...
}

struct Edges<Value> {
    top: Value,
    bottom: Value,
    left: Value,
    right: Value,
}
```

Cells can control the layout of other cells or text within them. Children can be laid out left to right, top to bottom, or stacked on top of each other. Children can be aligned along either axis.

```rs
struct CellStyle {
    // ...
    // How are children laid out?
    child_orientation: Orientation,
    child_alignment: Vector2F,
    // Push children apart by this much if not achieved by their margins
    min_child_spacing: Length,
    overflow: Overflow,
    // ...
}

enum Orientation {
    Horizontal,
    Vertical,
    Stacked
}

enum Overflow {
    Hide,
    Scroll {
        x: bool,
        y: bool,
    },
    Wrap,
}
```
