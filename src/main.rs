slint::include_modules!();

mod onani;

fn main() {
    env_logger::init();

    let main_window = MainWindow::new().expect("Could not create window");

    main_window.on_calculate_onani(|input| match input.trim().parse::<i32>() {
        Ok(n) if n >= 0 => onani::onani_number(n).into(),
        Ok(_) => "Please enter a non-negative integer".into(),
        Err(_) => "Invalid input. Please enter a valid number".into(),
    });

    let _ = main_window.run();
}
