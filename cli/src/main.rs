
mod pack;

mod term;


fn main() -> std::io::Result<()> {

    let mut stdout = std::io::stdout();

    term::interactive_term(&mut stdout)
}
