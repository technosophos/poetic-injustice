# Poetic Injustice

This is a simple web server with a chaotic twist. In normal operation, it prints out a line of poetry by a given poet from a given poem (`/`). But the health check (`/healthz`) randomly fails.

I use this for simple chaos testing in a Kubernetes cluster, including this image in testing configurations for different resource types.

## Environment Variables:

- POEM: Title of a poem
- POET: Name of a poet
- QUOTE: Line of poetry

## Building

- Use Rust 2018ed or later
- Run `cargo run` (or `cargo build`)
- ???
- Profit