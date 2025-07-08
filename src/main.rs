slint::include_modules!();

fn main() {
    env_logger::init();

    // 会阻塞线程，实际开发中需要自行调整
    let _ = MainWindow::new().expect("Could not create window").run();
}
