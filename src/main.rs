use eframe::egui;
use nixinfo::{cpu, device, distro, env, environment, gpu,
              memory_used, memory_total, music, packages,
              temp, uptime};

#[macro_export]
/// This macro adds the UI elements for the separate pieces of information being displayed.
macro_rules! add_info {
    ($h:expr, $b:expr, $ui:expr) => {
        {
            $ui.heading($h);
            $ui.add(egui::TextEdit::singleline(&mut $b.to_owned())
                .horizontal_align(eframe::emath::Align::Center)
                .desired_width(f32::INFINITY)
                .code_editor()
            );
        }
    };
}

fn main() -> Result<(), eframe::Error> {
    let manager = if std::env::args().nth(1).is_some() {
        std::env::args().nth(1).unwrap()
    } else {
        "UNKNOWN".to_string()
    };

    let image_path = if std::env::args().nth(2).is_some() {
        std::env::args().nth(2).unwrap()
    } else {
        "N/A".to_string()
    };

    let image_bytes = if image_path != "N/A" {
        std::fs::read(&image_path).unwrap() // Read the bytes of the file and return it to `image_bytes`.
    } else {
        [].to_vec() // To make this work, both return types needed to be the same.
                    // So I return an empty vector if an image path wasn't specified.
    };

    // Set the size of the window to 1142x532, and leave the rest of the options as the defaults.
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1142.0, 532.0)), // Perfect size to fit all info + image.
        ..Default::default() 
    };

    // Gather information
    let cpu           = cpu().unwrap();
    let device        = device().unwrap();
    let distro        = distro().unwrap();
    let editor        = env("EDITOR").unwrap();
    let environment   = environment().unwrap();
    let gpus          = gpu().unwrap();
    let mut gpu_string: String = String::new();
    let mut gpu_string_nf: String = String::new();
    for gpu in gpus {
        // TODO: Trim the final '\n' from the String.
        let string = format!("GPU:         {gpu}\n");
        let string_nf = format!("{gpu}\n");
        gpu_string.push_str(&string);
        gpu_string_nf.push_str(&string_nf);
    }
    let memory_total  = memory_total().unwrap();
    let memory_used   = memory_used().unwrap();
    let memory_string = format!("{memory_used}/{memory_total}");
    
    #[cfg(any(feature = "music_mpd", feature = "music_playerctl"))]
    let music         = music().unwrap();

    #[cfg(not(any(feature = "music_mpd", feature = "music_playerctl")))]
    let music         = music();

    let packages      = packages(&manager).unwrap();
    // temp() returns a vector of tuples, with each tuple containing a device name and temperature
    // respectively. The CPU is typically the first one, so we grab the first element of the vector
    // and the second element of the tuple.
    let temp          = &temp().unwrap()[0].1;
    let uptime        = uptime().unwrap();
    let user          = env("USER").unwrap();
    let output = format!("\
        CPU:         {cpu} [{temp}*C]\n\
        Device:      {device}\n\
        Distro:      {distro}\n\
        Editor:      {editor}\n\
        Environment: {environment}\n\
        {gpu_string}\
        Memory:      {memory_string}\n\
        Music:       {music}\n\
        Packages:    {packages} [{manager}]\n\
        Uptime:      {uptime}\n\
        User:        {user}"
    );

    // Run the UI
    eframe::run_simple_native("rsfetch", options, move |ctx, _frame| {
        // Panel at the top, with a collapsed element containing all information.
        egui::TopBottomPanel::top("All Information").show(ctx, |ui| {
            ui.collapsing("All Information", |ui| {
                // The desired width is INFINITY to disable line wrapping.
                ui.add(egui::TextEdit::multiline(&mut output.to_owned()).desired_width(f32::INFINITY).code_editor());
            });
        });
        // The central panel containing all individual pieces of information.
        // TODO: Organize the information better.
        // TODO: Align the text in the center to make room for an image to the left.
        // TODO: Potentially change the background of the window to the image as an option.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                // If an image is supplied, it will be displayed to the left of information content.
                if ! image_bytes.is_empty() {
                    // Create the image texture required to display the image.
                    let image_texture = egui_extras::RetainedImage::from_image_bytes(
                        &image_path, 
                        &image_bytes
                    ).unwrap();

                    // Display the image with a hardcoded size of 455x455 to make it match
                    // the size of information being displayed to the right.
                    ui.image(image_texture.texture_id(ctx), [455.0, 455.0]);
                }
                ui.vertical_centered_justified(|ui| {
                    add_info!("CPU", cpu, ui);
                    add_info!("Device", device, ui);
                    add_info!("Distro", distro, ui);
                    add_info!("Environment", environment, ui);
                    add_info!("GPU(s)", gpu_string_nf, ui);
                    add_info!("Memory", memory_string, ui);
                    add_info!("Music", music, ui);
                    add_info!("Packages", packages, ui);
                    add_info!("Uptime", uptime, ui);
                    add_info!("User", user, ui);
                });
            });
        });
    })
}
