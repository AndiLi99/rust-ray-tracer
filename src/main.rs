fn main() {
    
    // Image

    let image_width: i64 = 256;
    let image_height: i64 = 256;

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for y in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", y);
        for x in 0..image_width {
            let r : f64 = (x as f64) / ((image_height - 1) as f64);
            let g : f64 = (y as f64) / ((image_width - 1) as f64);
            let b : f64 = 0.25;

            let ir: u8 = (r * 255 as f64) as u8;
            let ig: u8 = (g * 255 as f64) as u8;
            let ib: u8 = (b * 255 as f64) as u8;

            println!("{} {} {}", ir, ig, ib)
        }
    }
    eprintln!("\nDone.");
}
