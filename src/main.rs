use xcb_test_wm::WindowManager;

fn main() {
    let mut wm = WindowManager::new();

    wm.run();
}
