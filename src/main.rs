mod file_utils;
mod hashlib;

use std::path::Path;

use file_utils::read_sum_file;
use hashlib::HashAlgorithm;
use native_dialog::FileDialog;
use slint::Color;
use std::str::FromStr;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_file_button_clicked({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            // Choose File
            let path = FileDialog::new()
                .set_location("~")
                .show_open_single_file()
                .unwrap();

            let path = match path {
                Some(path) => path,
                None => return,
            };
            // set CheckSum
            let algo_ui_value = ui.get_algo();
            let algo_option = HashAlgorithm::from_str(algo_ui_value.as_str());
            let compute_hash;

            match algo_option {
                Ok(algo) => compute_hash = algo,
                Err(_) => {
                    ui.set_file("Algorithm not supported".into());
                    ui.set_result("Algorithm not supported".into()); 
                    return;
                }
            }

            let compute_hash = hashlib::hash(&path, compute_hash);
            match compute_hash {
                Err(err) => ui.set_fileSum(err.to_string().into()),
                Ok(digest) => ui.set_fileSum(digest.into()),
            }

            // Set InputText to choosen file
            ui.set_file(path.display().to_string().into());
        }
    });

    ui.on_checksum_file_button_clicked({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            // Choose Sum File
            let path = FileDialog::new()
                .set_location("~")
                .show_open_single_file()
                .unwrap();

            let path = match path {
                Some(path) => path,
                None => return,
            };
            // Set InputText to choosen file
            ui.set_checksum(path.display().to_string().into());

            // let selected_file = path.file_name().map(|name| name.to_string_lossy().into_owned());
            let selected_file_path = ui.get_file().to_string();
            let selected_file = Path::new(&selected_file_path)
                .file_name()
                .map(|name| name.to_string_lossy().into_owned());
            if selected_file.is_some() {
                let sum = read_sum_file(path, selected_file.unwrap());
                match sum {
                    Ok(sumvalue) => ui.set_checksum(sumvalue.into()),
                    Err(err) => ui.set_checksum(err.to_string().into()),
                }
            }
        }
    });

    ui.on_check_button_clicked({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            
            let file_sum = ui.get_fileSum();
            let check_sum = ui.get_checksum();

            println!("Filesum:  {}", file_sum);
            println!("Checksum: {}", check_sum);

            if file_sum.len() < 1 || check_sum.len() < 1 {
                println!("Filesum or checksum < 1");
                return
            }

            if file_sum.eq_ignore_ascii_case(&check_sum) {
                ui.set_result("OK!".into());
                ui.set_backgroundColor(slint::Brush::SolidColor(Color::from_rgb_u8(0, 255, 0)));
            } else {
                ui.set_backgroundColor(slint::Brush::SolidColor(Color::from_rgb_u8(255, 0, 0)));
                ui.set_result("NO MATCH!".into());
            }

        }
    });

    ui.run()
}
