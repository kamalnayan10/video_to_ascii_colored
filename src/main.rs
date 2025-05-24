use std::{process::{Command,Stdio}, time::Duration};
use image::GenericImageView;
use colored::Colorize;
use crossterm::terminal;
use std::{fs,path::Path , time , thread,env};

fn get_ascii(color_avg:u8) -> &'static str{
    //for 5 color levels
    //let idx = color_avg/52;
    //for 8 color levels
    let idx = color_avg/32;
    //for 5 color levels
    //let ascii_char = ["." , "#" , "%" , "$" , "@"];
    //for 8 color levels
    //let ascii_char = ["*" , "*" , "*" ,"*","*","*","*","*"];
    let ascii_char = ["." , "-" , "*" ,"=","o","#","%","$"];//redefined according to appropriate thickness of characters
    //let ascii_char = [".","▁" , "▂" , "▃" ,"▄","▅","▆","▇"];
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

            let mut color_avg: u8 = (pixels[0]/3 + pixels[1]/3 + pixels[2]/3);

            if pixels[3] == 0{
                color_avg = 0;
            }
            else{
                color_avg*=pixels[3]/255;
            }

            let ascii = get_ascii(color_avg);

            
            print!("{:2}" , ascii.truecolor(pixels[0],pixels[1],pixels[2]));
        }
        println!("");
    }
}



fn get_video_frame_rate(file_path: &str) -> Option<f32> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=r_frame_rate",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            file_path,
        ])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute ffprobe command");

    if let Ok(output_text) = String::from_utf8(output.stdout) {
        let frame_rate_parts: Vec<&str> = output_text.trim().split('/').collect();
        if frame_rate_parts.len() == 2 {
            if let (Ok(num), Ok(den)) = (frame_rate_parts[0].parse::<f32>(), frame_rate_parts[1].parse::<f32>()) {
                return Some(num / den);
            }
        }
    }
    None
}


fn main() {

    let args: Vec<String> = env::args().collect();
    
    let input_file = &args[1];

    let _ = fs::create_dir("frames");

    let output_pattern = "frames/output_%d.jpg";

    let mut fps: f32 = 1.00;

    if let Some(frame_rate) = get_video_frame_rate(&input_file){
        fps = frame_rate;
    }


    // Construct ffmpeg command to extract frames as JPEG images
    let mut ffmpeg_command = Command::new("ffmpeg");
    ffmpeg_command
        .arg("-i")
        .arg(input_file)
        .args(&["-vf", &format!("fps={fps}")])
        .arg("-q:v")
        .arg("2")
        .arg(output_pattern);
    
    
    // Execute the ffmpeg command
    let output = ffmpeg_command.output().expect("Failed to execute command");

    if output.status.success() {
        println!("Conversion successful!");
           
        let frames_directory = Path::new("frames");
        let mut frame_count = 1;
        if let Ok(entries) = fs::read_dir(&frames_directory) {
        for entry in entries.flatten() {
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                
                frame_count += 1; 
            }
        }
        print!("\x1b[?25l");//hide cursor
        for i in 1..frame_count{
            image_to_ascii(format!("frames/output_{i}.jpg"));
            print!("\x1b[2J"/* OR "{esc}c", esc = 27 as char*/);//optimised clear terminal 
            let _ = fs::remove_file(format!("frames/output_{i}.jpg"));  
        }         
        //std::process::Command::new("clear").status().unwrap();
        print!("\x1b[?25h");//unhide cursor
    }

    } else {
        println!(
            "Conversion failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
