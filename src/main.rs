use eframe::egui;
use nixinfo::{cpu, device, distro, env, environment,
              memory_used, memory_total, music, packages,
              temp, uptime};

fn main() -> Result<(), eframe::Error> {
    let manager = if std::env::args().nth(1).is_some() {
        std::env::args().nth(1).unwrap()
    } else {
        "UNKNOWN".to_string()
    };

    // Set all options to default for now
    let options = eframe::NativeOptions {
        ..Default::default() 
    };

    // Gather information
    let cpu           = cpu().unwrap();
    let device        = device().unwrap();
    let distro        = distro().unwrap();
    let editor        = env("EDITOR").unwrap();
    let environment   = environment().unwrap();
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
            ui.heading("CPU");
            ui.add(egui::TextEdit::singleline(&mut cpu.to_owned()).desired_width(f32::INFINITY));
            ui.heading("Device");
            ui.add(egui::TextEdit::singleline(&mut device.to_owned()).desired_width(f32::INFINITY));
            ui.heading("Distro");
            ui.add(egui::TextEdit::singleline(&mut distro.to_owned()).desired_width(f32::INFINITY));
            ui.heading("Environment");
            ui.add(egui::TextEdit::singleline(&mut environment.to_owned()).desired_width(f32::INFINITY));
            ui.heading("Memory");
            ui.add(egui::TextEdit::singleline(&mut memory_string.to_owned()).desired_width(f32::INFINITY));
            ui.heading("Music");
            ui.add(egui::TextEdit::singleline(&mut music.to_owned()).desired_width(f32::INFINITY));
            ui.heading("Packages");
            ui.add(egui::TextEdit::singleline(&mut packages.to_owned()).desired_width(f32::INFINITY));
            ui.heading("Uptime");
            ui.add(egui::TextEdit::singleline(&mut uptime.to_owned()).desired_width(f32::INFINITY));
        });
    })
}
