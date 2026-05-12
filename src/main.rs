mod app;
mod interpreter;

use std::env;

use crate::interpreter::Interpreter;
use crate::app::App;

fn main() -> std::io::Result<()> {
    let mut app = App::default();
    app.read_source(&env::args().nth(1).expect("must provide the source file"))?;

    let mut interpreter = Interpreter::default();

    interpreter.tokenize(&app.source);
    interpreter.run()?;

    Ok(())
}
