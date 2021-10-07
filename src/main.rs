use std::cell::RefCell;
use std::rc::Rc;

use std::io;

mod emoto;
mod util;

fn main() -> Result<(), io::Error> {
    let emoto = Rc::new(RefCell::new(emoto::Emoto::new()));
    util::start_ui(emoto)?;
    Ok(())
}
