// have seen the npm install work on windows
// now trying for Mac OS

// starting with v0.2.1 -a

mod actions;
mod types;

use crate::types::Crust;

fn main() {
    Crust::run(None);
}
