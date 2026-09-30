// Released under the MIT License.
// Copyright, 2022, by Bernhard Berger.
// Copyright, 2022-2025, by Titus Wormer.
// Copyright, 2026, by Samuel Williams.

fn main() -> Result<(), socketry_markdown::message::Message> {
    // Turn on debugging.
    // You can show it with `RUST_LOG=debug cargo run --features log --example lib`
    env_logger::init();

    // Safely turn (untrusted?) markdown into HTML.
    println!("{:?}", socketry_markdown::to_html("## Hello, *world*!"));

    // Turn trusted markdown into HTML.
    println!(
        "{:?}",
        socketry_markdown::to_html_with_options(
            "<div style=\"color: goldenrod\">\n\n# Hi, *Saturn*! 🪐\n\n</div>",
            &socketry_markdown::Options {
                compile: socketry_markdown::CompileOptions {
                    allow_dangerous_html: true,
                    allow_dangerous_protocol: true,
                    ..socketry_markdown::CompileOptions::default()
                },
                ..socketry_markdown::Options::default()
            }
        )
    );

    // Support GFM extensions.
    println!(
        "{}",
        socketry_markdown::to_html_with_options(
            "* [x] contact ~Mercury~Venus at hi@venus.com!",
            &socketry_markdown::Options::gfm()
        )?
    );

    // Access syntax tree and support MDX extensions:
    println!(
        "{:?}",
        socketry_markdown::to_mdast(
            "# <HelloMessage />, {username}!",
            &socketry_markdown::ParseOptions::mdx()
        )?
    );

    Ok(())
}
