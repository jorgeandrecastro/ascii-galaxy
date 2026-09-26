//! ============================================================================
//! Name of the  project : ascii-galaxy
//! Authors       : Jorge Andre Castro <georgeandrec@gmail.com>
//! Licence       : MIT (Copyright (c) 2026 Jorge Andre Castro)
//! ============================================================================
use std::env;
use image::GenericImageView;

fn main() {
    // Récupérer les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <chemin-vers-image>", args[0]);
        std::process::exit(1);
    }

    let image_path = &args[1];

    // Charger l'image fournie par l'utilisateur
    let img = match image::open(image_path) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Erreur lors du chargement de l'image '{}': {}", image_path, e);
            std::process::exit(1);
        }
    };

    // Redimensionner pour le terminal (ex: 80 colonnes de large)
    let resized = img.resize_exact(80, 40, image::imageops::FilterType::Lanczos3);

    let ascii_chars = vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

    for (_x, y, pixel) in resized.pixels() {
        if y > 0 && _x == 0 {
            println!();
        }
        
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        let luminance = 0.299 * r + 0.587 * g + 0.114 * b;
        
        let index = ((luminance / 255.0) * (ascii_chars.len() - 1) as f32) as usize;
        print!("{}", ascii_chars[index]);
    }
    println!();
}