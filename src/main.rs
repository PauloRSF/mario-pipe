use std::error::Error;

use mario::{MARIO_GOING_DOWN_PIPE, MARIO_JUMPING};

fn main() -> Result<(), Box<dyn Error>> {
    // If we're going down a pipe hehe
    if !atty::is(atty::Stream::Stdout) {
        print!("{MARIO_GOING_DOWN_PIPE}");
    } else {
        print!("{MARIO_JUMPING}");
    }

    Ok(())
}
