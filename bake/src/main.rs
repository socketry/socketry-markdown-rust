// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use bake::{Registry, Result};

fn main() -> Result<()> {
    Registry::discover()?.run()
}

#[path = "__bake_generated_tasks/mod.rs"]
mod __bake_generated_tasks;
