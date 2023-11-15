# GPUI: A Performant Desktop App Dev Tool

Welcome to GPUI! This new open-source tool, engineered in Rust, is designed for native desktop applications. Created in response to performance issues with Electron during the development of Atom, GPUI is a game-changer.

## GPUI's Special Edge

GPUI's principle is similar to React but with a twist: it exhibits elements as they're created, which accelerates the rendering process. In addition, it rasterizes the entire window at a staggering 120fps on the GPU, leaving the CPU unburdened by element modifications.

## Familiar and Easy to Use

GPUI is feature-rich but maintains familiarity for those with web tech experience. Using a web-compatible layout engine and styled with Tailwind CSS, it's intuitive to use. Here's a glimpse of GPUI:

```rust
fn main() {
    App::new().run(|cx| {
        cx.open_window(|cx| cx.new_view(|_cx| Hello("World".into()));
    })
}

struct Hello(SharedString);

// A view can be any render-able state piece.
impl Render for Hello {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Component {
        div()
            .p_5() // 5px padding
            .child("Hello")
            .child(self.0.clone())
    }
}
```

## Contribute to GPUI

We're tirelessly improving GPUI, and help with Linux and Windows support is welcomed. GPUI's amazing features include:

- Speed: Memory can be shared across different threads, no waiting on the garbage collector.
- User-friendly: No complex templating language, Rust knowledge suffices.
- Web-compatible: Packs a layout engine akin to web and CSS stylized by Tailwind.

Here's a GPUI sample:

```rust
fn main() {
    App::new().run(|cx| {
        cx.open_window(|cx| cx.new_view(|_cx| Hello("World".into()));
    })
}

struct Hello(SharedString);

// A view can be any render-able state piece.
impl Render for Hello {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Component {
        div()
            .p_5() // 5px padding
            .child("Hello")
            .child(self.0.clone())
    }
}
```


# Understand GPUI: Simplifying Desktop App Development

Hello there. Let's talk about GPUI. It's a fresh, open-source tool to build native desktop applications, and it's all done in Rust.

![GPUI Image](https://placeholder.com)

## What is GPUI?

So here's the backstory. We wanted a better, faster way to build desktop apps. You see, we were using Electron to build Atom, but we weren't happy with how it performed. That's when GPUI came into picture.

## GPUI: What's Different?

You might have heard about React. GPUI works on a similar concept, but with a slight variation. What it does is directly display the elements as they are created, instead of displaying them after a series of changes have been made to it. This gives it a speed boost.

Not just that, GPUI can draw the entire window at a super-fast speed of 120fps on the GPU. That's why the CPU doesn't need to worry about any changes made to the elements.

## Easy to Get Started

GPUI has amazing features. But, the best part? If you've worked with web technology before, you'll find GPUI easy to use. We've used a layout engine that's compatible with the web and styled it using Tailwind CSS. Here's a sneak peek of a program in GPUI.

```rust
fn main() {
    App::new().run(|cx| {
        cx.open_window(|cx| cx.new_view(|_cx| Hello("World".into()));
    })
}

struct Hello(SharedString);

// A view is any piece of state that can be rendered.
impl Render for Hello {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Component {
        div()
            .p_5() // 5px padding
            .child("Hello")
            .child(self.0.clone())
    }
}
```

## Join Us

We're looking to make GPUI better and we need your help. If you can help us support Linux or Windows you're more than welcome. Be a part of this new journey in building native desktop apps using GPUI.

Currently, we're fully supporting Mac. For Linux and Windows, we could use some help!

GPUI stands out because it's:

- Fast: No waiting for the Garbage collector to keep things speedy. And memory can be shared across different threads.
- User-friendly: No more learning another complicated templating language. If you know Rust, you're good to go.
- Web-compatible: Comes with a layout engine that works like a web and CSS styled by Tailwind.

To give it a shot, here's a simple program in GPUI.

```rust
fn main() {
    App::new().run(|cx| {
        cx.open_window(|cx| cx.new_view(|_cx| Hello("World".into()));
    })
}

struct Hello(SharedString);

// A view is any piece of state that can be rendered.
impl Render for Hello {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Component {
        div()
            .p_5() // 5px padding
            .child("Hello")
            .child(self.0.clone())
    }
}
```



# GPUI: The New Horizon for Desktop Application Development

Welcome to the GPUI project - an innovative open-source User Interface (UI) creation framework designed to develop native desktop applications purely in Rust.

![GPUI Image](https://placeholder.com)

## About GPUI

Initiated out of the need for better performance and enhanced efficiency, GPUI redefines the way desktop applications are built. This project was born out of the lingering frustration with the performance of the web platform while using Electron as the foundation of Atom. So, we decided to shift gears and start anew, this time with a closer association with the core – the metal.

## Why GPUI

GPUI portrays the UI as a function of its model state, a concept much similar to React. However, a striking point of difference is its focus on direct rendering of element trees, as opposed to React's method of differing trees to mutate a stateful document object model. This unique approach opens up new possibilities in performance enhancement.

GPUI holds the capability to rasterize the entire window at an impressive rate of 120fps on the GPU, hereby obviating the need for diffing trees on CPU.

## Familiar yet Innovative

While GPUI is packed with powerful features and benefits, it has been designed intentionally to be familiar with users accustomed to web technology. It utilizes a web-compatible layout engine and bases its styling system on Tailwind CSS, thus minimizing the learning curve.

Here is a simple sketch of a program created using GPUI, to give you an idea of what working with this framework looks like.

```rust
fn main() {
    App::new().run(|cx| {
        cx.open_window(|cx| cx.new_view(|_cx| Hello("World".into()));
    })
}

struct Hello(SharedString);

// A view is any piece of state that can be rendered.
impl Render for Hello {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Component {
        div()
            .p_5() // 5px padding
            .child("Hello")
            .child(self.0.clone())
    }
}
```

## Join our Community

GPUI is an open-source project and we would be thrilled to have you as a part of our community. We're looking for help to expand our reach to Linux and Windows from the existing Mac platform.

Hop on board to be a part of this breakthrough in native desktop application development, and help us in making the project better.


GPUI is a framework for building native desktop applications in pure Rust.

Platforms:
- Mac - In production
- Linux, Windows: Help wanted!

- Fast - Never drop a frame waiting for the garbage collector. Share memory across multiple threads.
- Familiar - Web-compatible layout engine plus a styling system inspired by Tailwind CSS.
- Intuitive - Express element trees in plain Rust with method chaining rather than learning yet another templating language or macro.

After creating Electron as the foundation of Atom, we grew frustrated with the performance of the web platform, so we decided to start over, this time closer to the metal.

GPUI expresses UI as a function of model state, much like React. Whereas React diffs trees of elements to mutate a stateful document object model, GPUI focuses on rendering element trees directly. If we can rasterize the whole window at 120fps on the GPU, why bother diffing trees on the CPU?

GPUI should be productive for anyone familiar with web technology that's willing to learn Rust. We use a web-compatible layout engine and our styling system is based on Tailwind CSS.

Here's a simple program (I have not actually run this yet, just a sketch)

```rust
fn main() {
    App::new().run(|cx| {
        cx.open_window(|cx| cx.new_view(|_cx| Hello("World".into()));
    })
}

struct Hello(SharedString);

// A view is any piece of state that can be rendered.
impl Render for Hello {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Component {
        div()
            .p_5() // 5px padding
            .child("Hello")
            .child(self.0.clone())
    }
}
```
