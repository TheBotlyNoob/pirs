fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let mut rl = rustyline::Editor::<()>::new();
  
  let rl_history_file = format!(
    "{}/.pirs_history",
    dirs::home_dir().unwrap().display()
  );

  rl.load_history(&rl_history_file)
  .ok();

  let mut pirs = pirs::Pirs::new(|code| std::process::exit(code), pirs::LogLevel::Info);

  loop {
    match rl.readline(&pirs.state.prompt) {
      Ok(input) => {
        rl.add_history_entry(&input);
        pirs.handle_command(input);
      }
      _ => ()
  }
  
  rl.save_history(&rl_history_file).ok();
}}
