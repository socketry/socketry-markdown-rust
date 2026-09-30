use bake::{Registry, Result};
use bake_license as _;

fn main() -> Result<()> {
    Registry::discover()?.run()
}
