mod args;
mod cmds;
mod select;
mod exec;

fn main() {
    // @see https://github.com/mitsuhiko/dialoguer/issues/77
    ctrlc::set_handler(move || {
        let _ = console::Term::stdout().show_cursor();
    }).unwrap();

    let content = args::get_content();
    let commands = cmds::Command::get_commands(content);
    let command = select::select_commands(&commands);
    println!("execute command: {}", command);
    exec::execute(&command.executable());
}
