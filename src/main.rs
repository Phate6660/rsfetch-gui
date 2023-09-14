use eframe::egui;
use nixinfo::{cpu, device, distro, env, environment, gpu,
              memory_used, memory_total, music, packages,
              temp, uptime};

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
        std::fs::read(&image_path).unwrap()
    } else {
        [].to_vec()
    };

    // Set all options to default for now
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1142.0, 532.0)), // Perfect size to fit all info
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
                // Place the specified image (the second CLI arg), to the left of the separated info.
                // Create the texture from the image.
                if ! image_bytes.is_empty() {
                    let image_texture = egui_extras::RetainedImage::from_image_bytes(
                        &image_path, 
                        &image_bytes
                    ).unwrap();

                    ui.image(image_texture.texture_id(ctx), [455.0, 455.0]); // Hardcoded image size
                                                                             // to make it match the size
                                                                             // of the information.
                }
                ui.vertical_centered_justified(|ui| {
                    ui.heading("CPU");
                    ui.add(egui::TextEdit::singleline(&mut cpu.to_owned())
                        .horizontal_align(eframe::emath::Align::Center)
                        .desired_width(f32::INFINITY)
                        .code_editor()
                    );
                    ui.heading("Device");
                    ui.add(egui::TextEdit::singleline(&mut device.to_owned())
                        .horizontal_align(eframe::emath::Align::Center)
                        .desired_width(f32::INFINITY)
                        .code_editor()
                    );
                    ui.heading("Distro");
                    ui.add(egui::TextEdit::singleline(&mut distro.to_owned())
                        .horizontal_align(eframe::emath::Align::Center)
                        .desired_width(f32::INFINITY)
                        .code_editor()
                    );
                    ui.heading("Environment");
                    ui.add(egui::TextEdit::singleline(&mut environment.to_owned())
                        .horizontal_align(eframe::emath::Align::Center)
                        .desired_width(f32::INFINITY)
                        .code_editor()
                    );
                    ui.heading("GPU(s)");
                    ui.add(egui::TextEdit::multiline(&mut gpu_string_nf.trim().to_owned())
                        .horizontal_align(eframe::emath::Align::Center)
                        .desired_width(f32::INFINITY)
                        .code_editor()
                    );
                    ui.heading("Memory");
                    ui.add(egui::TextEdit::singleline(&mut memory_string.to_owned())
                        .horizontal_align(eframe::emath::Align::Center)
                        .desired_width(f32::INFINITY)
                        .code_editor()
                    );
                    ui.heading("Music");
                    ui.add(egui::TextEdit::singleline(&mut music.to_owned())
                        .horizontal_align(eframe::emath::Align::Center)
                        .desired_width(f32::INFINITY)
                        .code_editor()
                    );
                    ui.heading("Packages");
                    ui.add(egui::TextEdit::singleline(&mut packages.to_owned())
                        .horizontal_align(eframe::emath::Align::Center)
                        .desired_width(f32::INFINITY)
                        .code_editor()
                    );
                    ui.heading("Uptime");
                    ui.add(egui::TextEdit::singleline(&mut uptime.to_owned())
                        .horizontal_align(eframe::emath::Align::Center)
                        .desired_width(f32::INFINITY)
                        .code_editor()
                    );
                    ui.heading("User");
                    ui.add(egui::TextEdit::singleline(&mut user.to_owned())
                        .horizontal_align(eframe::emath::Align::Center)
                        .desired_width(f32::INFINITY)
                        .code_editor()
                    );
                });
            });
        });
    })
}
