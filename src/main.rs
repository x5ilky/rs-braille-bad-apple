use std::fs;
use std::fs::File;

const WIDTH: usize = 360;
const HEIGHT: usize = 240;

fn main() {
    let paths = fs::read_dir("./frames").unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    let mut frames: Vec<[[bool; HEIGHT]; WIDTH]> = vec![];

    let mut i = 0;
    let len = paths.len();
    for path in &paths[0..1] {
        let mut pixels = [[false; HEIGHT]; WIDTH];
        let from_path = bytes_from_path(&path);
        println!("{i}/{} | W: {} H: {}", len, from_path.1, from_path.2);
        let split_width = from_path.1 as i32 / WIDTH as i32;
        let split_height = from_path.2 as i32 / HEIGHT as i32;

        for j in 0..((from_path.0.len() as i32 / split_width) - 1) {
            let x_offset = j*split_width;
            for k in 0..(from_path.0.len() as i32 / split_height) {
                let y_offset = k*split_height;
                if from_path.0[(Into::<i32>::into(y_offset * from_path.1 as i32 + x_offset)) as usize] != 0 {
                    pixels[j as usize][k as usize] = true;
                }
            }
        }

        frames.push(pixels);

        i += 1;
    }

    for frame in frames {
        for i in 0..((WIDTH as i32)/2) {
            let x_offset = i*2;
            for j in 0..((HEIGHT as i32)/3) {
                let y_offset = j*3;
                let one = Into::<u32>::into(frame[(x_offset+0) as usize][(y_offset+0) as usize]);
                let two = Into::<u32>::into(frame[(x_offset+0) as usize][(y_offset+1) as usize]);
                let three = Into::<u32>::into(frame[(x_offset+0) as usize][(y_offset+2) as usize]);
                let four = Into::<u32>::into(frame[(x_offset+1) as usize][(y_offset+0) as usize]);
                let five = Into::<u32>::into(frame[(x_offset+1) as usize][(y_offset+1) as usize]);
                let six = Into::<u32>::into(frame[(x_offset+1) as usize][(y_offset+2) as usize]);
                print!("{}", generate_braille(one + two*2 + three*4 + four*8 + five*16 + six*32));
            }
            print!("\n");
        }
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
    let chunks: Vec<&[u8]> = binding.chunks(3).collect();
    return (chunks.iter().map(|c| c[0]).collect(), width, height);
}