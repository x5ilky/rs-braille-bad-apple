use std::io::{Write, Read};
use std::{fs, thread, time, env};
use std::fs::File;

const HEIGHT: usize = 100;
const WIDTH: usize = 210;

fn main() {
    let args: Vec<String> = env::args().collect();
        let paths = fs::read_dir("./frames").unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_string_lossy().into_owned())
            .collect::<Vec<_>>();
    
        let mut frames: Vec<[[bool; WIDTH]; HEIGHT]> = vec![];
    
        let mut i = 0;
        let len = paths.len();
        for path in &paths {
            let mut pixels = [[false; WIDTH]; HEIGHT];
            let from_path = bytes_from_path(&path);
            if i % 100 == 0 {println!("[INFO] converting: {i}/{}", len);}
            let split_width = from_path.1 as i32 / WIDTH as i32;
            let split_height = from_path.2 as i32 / HEIGHT as i32;
    
            for k in 0..HEIGHT as i32 {
                let y_offset = k*split_height;
                for j in 0..WIDTH as i32 {
                    let x_offset = j*split_width;
                    if from_path.0[((Into::<i32>::into(y_offset * from_path.1 as i32 + x_offset))*3) as usize] != 0 {
                        pixels[k as usize][j as usize] = true;
                    }
                }
            }
    
            frames.push(pixels);
    
            i += 1;
        }
        for frame in frames {
            print!("\x1b[1;1H");
            for j in 0..((HEIGHT as i32)/3)-1 {
                let y_offset = j*3;
                for i in 0..((WIDTH as i32)/2)-1 {
                    let x_offset = i*2;
                    let one = Into::<u32>::into(frame[(y_offset+0) as usize][(x_offset+0) as usize]);
                    let two = Into::<u32>::into(frame[(y_offset+1) as usize][(x_offset+0) as usize]);
                    let three = Into::<u32>::into(frame[(y_offset+2) as usize][(x_offset+0) as usize]);
                    let four = Into::<u32>::into(frame[(y_offset+0) as usize][(x_offset+1) as usize]);
                    let five = Into::<u32>::into(frame[(y_offset+1) as usize][(x_offset+1) as usize]);
                    let six = Into::<u32>::into(frame[(y_offset+2) as usize][(x_offset+1) as usize]);
                    print!("{}", generate_braille(one + two*2 + three*4 + four*8 + five*16 + six*32));
                }
                print!("\n");
            }
            thread::sleep(time::Duration::from_millis(1000/60));
        }
}

fn generate_braille(num: u32) -> char {
    return std::char::from_u32(num + 0x2800).unwrap()
}

fn bytes_from_path(path: &String) -> (Vec<u8>, u32, u32) {
    let file = File::open(path).unwrap();
    let mut decoder = png::Decoder::new(&file);
    let header_info = &mut decoder.read_header_info().unwrap();
    let (width, height) = (header_info.width, header_info.height);
    let reader = &mut decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = &mut reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];
    let binding = bytes.to_vec();
    return (binding, width, height);
}

// fn parse_frames() -> Vec<[[bool; WIDTH]; HEIGHT]> {
//     let mut frames: Vec<[[bool; WIDTH]; HEIGHT]> = vec![];
//     let mut filestr = String::from("");
//     let _ = File::open("frames_cache").expect("expected frames_cache file").read_to_string(&mut filestr);
// 
//     let afterfile = &filestr[8..].to_string();
//     let frame_count = &filestr[0..8].parse::<u32>().unwrap();
//     println!("{}", frame_count);
// 
//     let chars = afterfile.as_bytes().to_vec();
//     let offset = ((WIDTH*HEIGHT + 7) / 8)*8;
// 
//     let mut c = 0;
// 
//     for chunk in chars.chunks(offset) {
//         let ch = chunk.to_vec();
//         let mut frame: Vec<bool> = vec![];
//         for u in ch {
//             let b1 = if u & 0b00000001 != 0 { true } else { false };
//             let b2 = if u & 0b00000010 != 0 { true } else { false };
//             let b3 = if u & 0b00000100 != 0 { true } else { false };
//             let b4 = if u & 0b00001000 != 0 { true } else { false };
//             let b5 = if u & 0b00010000 != 0 { true } else { false };
//             let b6 = if u & 0b00100000 != 0 { true } else { false };
//             let b7 = if u & 0b01000000 != 0 { true } else { false };
//             let b8 = if u & 0b10000000 != 0 { true } else { false };
//             frame.push(b1);
//             frame.push(b2);
//             frame.push(b3);
//             frame.push(b4);
//             frame.push(b5);
//             frame.push(b6);
//             frame.push(b7);
//             frame.push(b8);
//         }
//         let mut frame_r: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];
//         for i in 0..HEIGHT {
//             for j in 0..WIDTH {
//                 frame_r[i][j] = frame.first().unwrap().to_owned();
//                 frame.remove(0);
//             }
//         }
//         frames.push(frame_r);
//         if c % 10 == 0 { println!("[INFO] parsing: {}/{}", c, chars.len()) }
//         c+=1;
// 
//     }
// 
//     return frames;
// }