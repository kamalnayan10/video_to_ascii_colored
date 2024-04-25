use std::process::Command;
use image::GenericImageView;
use colored::Colorize;
use crossterm::terminal;
use std::{thread,time};


fn get_ascii(color_avg:u8) -> &'static str{
    let idx = color_avg/52;

    let ascii_char = ["." , "#" , "%" , "$" , "@"];

    return ascii_char[idx as usize]
}

fn image_to_ascii(path:String){
    let (term_w , term_h) = terminal::size().unwrap();
    
    let img = image::open(path).unwrap();

    let resized_img = img.resize((term_w) as u32, (term_h) as u32,image::imageops::FilterType::Nearest);
    
    let (width , height) = resized_img.dimensions();

    for y in 0..height{
        for x in 0..width{
            let pixels = resized_img.get_pixel(x,y);

            let mut color_avg: u8 = pixels[0]/3 + pixels[1]/3 + pixels[2]/3;

            if pixels[3] == 0{
                color_avg = 0;
            }

            let ascii = get_ascii(color_avg);

            
            print!("{:^2}" , ascii.truecolor(pixels[0],pixels[1],pixels[2]));
        }
        println!("");
    }
}

fn main() {
    // Input video file
    let input_file = "input.mp4";

    // Output image files
    let output_pattern = "frames/output_%04d.jpg";

    // Construct ffmpeg command to extract frames as JPEG images
    let mut ffmpeg_command = Command::new("ffmpeg");
    ffmpeg_command
        .arg("-i")
        .arg(input_file)
        .args(&["-vf", "fps=1"])
        .arg("-q:v")
        .arg("2")
        .arg(output_pattern);

    // Execute the ffmpeg command
    let output = ffmpeg_command.output().expect("Failed to execute command");

    // Check if the command was successful
    if output.status.success() {
        println!("Conversion successful!");
        for i in 1..12{
            let mut to_add = format!("0{i}");
            if i >= 10{
                to_add = String::from(format!("{i}"));
            }
            image_to_ascii(format!("frames/output_00{to_add}.jpg"));
            let ten_millis = time::Duration::from_millis(1000);
            thread::sleep(ten_millis);
            std::process::Command::new("clear").status().unwrap();
        
        }

    } else {
        println!(
            "Conversion failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}