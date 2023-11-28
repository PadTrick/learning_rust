use std::process::{Command, exit};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let app = Application::new(
        Some("com.example.app"),
        Default::default(),
    );

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("SANDISK CLIP SPORT PLUS MOUNTER");
        window.set_default_size(350, 70);

        let button = Button::with_label("MOUNT MP3 PLAYER");
        button.connect_clicked(|_| {
            let commands = vec![
                "pkexec parted -l && lsblk -f",
            ];

            for cmd in commands {
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output()
                    .expect("failed to execute process");

                println!("{}", String::from_utf8_lossy(&output.stdout));
            }

            // Beende das Programm nach Ausführung der Befehle
            exit(0);
        });

        window.set_child(Some(&button));
        window.show_all();
    });

    app.run();
}