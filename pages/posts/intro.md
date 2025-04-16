# Yet another static file website

Today marks my 3rd (possibly 4th) attempt to create my own static file site. If there's one thing I've learned about myself is I never seem to tire of finding new shiny ways to do the same thing. The flavor of this half-decade (lustrum?) is Rust paired with a nifty little HTML macro crate by the name of [Maud](https://maud.lambda.xyz/). 

Maud is great. It's so great that I prefer using it's `html!` macro over writing actual HTML. Here's a couple lines to give you a taste:
```rust
html! {
    div class="some-container" {
        h1 {"foo" }
        p {"bar" }
    }
}
```
Super succinct, so less keystrokes. It's also nice not having to think about balancing closing HTML tags.

The usage of Axum is so minimal it's probably not worth talking about. So minimal that I really want to talk about it:
```rust
#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0")
        .await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
```
10 lines is all that's needed to host this site! And not an annotation based handler or heavy DI framework in sight. Beautiful.


> But wait! How can you use Maud to serve static files, when the Maud macros have to be compiled? 

The key here is to generate the files beforehand. One could potentially create a separate Cargo project for spitting out the raw HTML, but I've opted to use a Cargo build script instead. The idea behind Cargo build scripts is there might be some preliminary dependencies you want to create prior to building your main executable (or library). The usual candidates here are C or C++ dependencies that need to be built prior to them being linked to your Rust binary. 

I'm stretching (butchering probably) the definition of a Cargo build dependency by using a build script to create HTML, which one could argue is a runtime dependency. There's also this strongly worded bit of advice in the official Cargo Book:

> Build scripts may save any output files or intermediate artifacts in the directory specified in the OUT_DIR environment variable. **Scripts should not modify any files outside of that directory.**

The idea here is that Cargo is more efficient when it is intimately aware of all changes in the dependency graph, which is important for things like incremental builds and caching.

Why that matters less here:

