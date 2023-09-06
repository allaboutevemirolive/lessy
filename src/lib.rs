mod init;
pub mod process;
pub mod util;
use init::*;
use util::FileManager;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {

    let text_to_be_replaced = "Click here to view code image";
    let words_to_delete = vec![r#"\[[0-9]+\]"#];
    let file_manager = FileManager::new("input.txt", "output.md");
    let skip_code = false;
    // ignore after_dot_sign

    let main_init = Initializer::new(
        file_manager,
        text_to_be_replaced.to_string(),
        words_to_delete,
        skip_code,
    );

    main_init.run_init()?;

    Ok(())
}
