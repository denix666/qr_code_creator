#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use qrcode_generator::QrCodeEcc;
use egui_macroquad::*;
use macroquad::prelude::*;
use egui_extras::RetainedImage;

fn window_conf() -> Conf {
    let mut title = String::from("QR code creator v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: 300,
        window_height: 310,
        window_resizable: false,
        ..Default::default()
    }
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path).unwrap().decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut data: String = "".to_string();
    let requested_file_name = format!("output.png");
    let qr_file = Path::new(requested_file_name.as_str());

    loop {
        clear_background(BLACK);

        egui_macroquad::ui(|ctx| {
            egui::TopBottomPanel::top("top").show(&ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("Write text here to generate QR code online:");
                    if ui.add(egui::TextEdit::multiline(&mut data).desired_rows(6).desired_width(300.0)).changed() {
                        qrcode_generator::to_png_to_file(&data, QrCodeEcc::Low, 200, "output.png").unwrap();
                    };
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.add_space(40.0);
                        match std::fs::File::open(qr_file) {
                            Ok(_) => {
                                let qr_img = RetainedImage::from_color_image("qr_image",load_image_from_path(qr_file).unwrap());
                                qr_img.show(ui);
                            },
                            Err(_) => {},
                        };
                    });
                    ui.add_space(10.0);
                });
            });
        });

        egui_macroquad::draw();
        next_frame().await
    }
}