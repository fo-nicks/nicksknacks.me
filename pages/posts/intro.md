# Yet another static file website

Today marks my 3rd (possibly 4th) attempt to create my own static file site. If there's one thing I've learned about myself, I never tire of finding new shiny ways to do the same thing. The flavor of this half-decade (lustrum?) is Rust paired with a nifty little HTML macro crate by the name of [Maud](https://maud.lambda.xyz/). 

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

The usage of Axum is so minimal it's probably not worth talking about. It's _so_ minimal that I really want to talk about it:
```rust
#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback_service(ServeDir::new("static"));

    let port = port();
    let (cert, key) = certs();
    let config = 
        RustlsConfig::from_pem_file(cert, key)
        .await.unwrap();

    let address = 
        SocketAddr::from(([0, 0, 0, 0], port));
    axum_server::bind_rustls(address, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```
And not an annotation-designated handler or heavy DI framework in sight! Beautiful.


> But wait! How can you use Maud to serve static files, when the Maud macros have to be compiled? 

The key here is to generate the files beforehand. One could potentially create a separate Cargo project for spitting out the raw HTML, but I've opted to use a Cargo build script instead. The idea behind Cargo build scripts is there might be some preliminary dependencies you want to create prior to building your main executable (or library). The usual candidates here are C or C++ dependencies that must be built before being linked to your Rust binary. 

I'm stretching (butchering probably) the definition of a Cargo build dependency by using a build script to create HTML, which one could argue is a runtime dependency. As tacky as it may be, I'm sticking with the build script because,

 - A single `cargo build` command builds everything
 - The number of HTML files to generate is small
 - I'm doubtful anyone will ever build this besides me

 ## Writing Posts

As concise as Maud's macros are, I prefer not to think in HTML when writing posts. I've opted to use the [`pulldown-cmark`](https://github.com/pulldown-cmark/pulldown-cmark) crate to parse markdown files and generate HTML post files, one for every markdown file in a flat source directory. I like this approach because the file name can be used to ensure unique URL handles.

There's only one issue: how do we store and display post meta such as the publishing date? I had a few ideas about this. The first was to lean on file properties. Maybe use the file creation timestamp as the publishing date. The main problem here is that there's a good chance I end up restoring these files from a backup or moving them, and then extra care would be needed to ensure this timestamp is preserved--not ideal. Also, file properties won't help us store arbitrary properties like a tag list. 

Another thought I had revolved around using markdown meta blocks to store the extra post information. `pulldown-cmark` supports this behind a feature flag, as it isn't in the markdown spec. I like this way the most, as all post information can be obtained from a single file.
 
Unfortunately, I lack the time to dig into the nuts and bolts of `pulldown-cmark` parsing. So, this gets to be a future enhancement, and I have somewhat begrudgingly settled on using a JSON file to house the post meta. 

By way of example, if I have a post with the path `posts/hello-world.md`, then right alongside it, I can add a `posts/hello-world.json`:
 ```
 {
  "date": "2025-04-17",
  "tags": [
    "Rust",
    "Web"
  ]
}
```
We can lean on good ol' `serde_json` here to read and deserialize the meta file. The build script can then use this information when generating the HTML for the associated post. 
 

## In Practice

It's working great so far! There are a few odds and ends I've neglected to mention, which probably warrant another post. In the meantime, feel free to check out the [source](https://github.com/fo-nicks/nicksknacks.me). 