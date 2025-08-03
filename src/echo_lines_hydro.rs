use hydro_lang::*;
pub fn echo_lines_hydro(process: &Process) {
    process
        .source_iter(q!(std::iter::once("Alice".to_string())))
        .for_each(
            q!(
                | name | { println!("What's your name?"); let name = name.trim();
                println!("Hello, {}!", name); }
            ),
        );
}
