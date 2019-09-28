use std::fs::File;
use std::io::Read;
use super::{CharImage, CHAR_IMG_SIZE};

pub struct FileData{
    pub answers: Vec<u8>,
    current_char: usize,
    pub image_count: i32,
    image_file: File,
}

impl FileData{

    pub fn load_image(&mut self) -> Option<CharImage> {
        if let Some(answer) = self.answers.get(self.current_char){
            let answer = *answer;
            let data = self.load_image_data();
            self.current_char += 1;

            return Some(CharImage{data, answer});
        }

        None
    }

    fn load_image_data(&mut self) -> Vec<f32>{
        let pixels_per_char = CHAR_IMG_SIZE * CHAR_IMG_SIZE;
        let mut buf: Vec<u8> = vec![0u8; pixels_per_char];

        self.image_file.read_exact(&mut buf).expect("Failed to do read image file");

        buf.iter().map(|val| *val as f32 / 255.0).collect()
    }
}

pub fn load_data() -> std::io::Result<FileData>{
    println!("Loading files");

    let answers = load_answers()?;
    let mut image_file = load_file("data/train-images.idx3-ubyte", 2051)?;
    let image_count = read_i32(&mut image_file)?;
    let image_width =  read_i32(&mut image_file)?;
    let image_height =  read_i32(&mut image_file)?;

    assert_eq!(image_width, CHAR_IMG_SIZE as i32);
    assert_eq!(image_height, CHAR_IMG_SIZE as i32);

    Ok(FileData {
        answers, 
        current_char: 0,
        image_count,
        image_file: image_file})
}

fn load_answers() -> std::io::Result<Vec<u8>> {
    let mut file = load_file("data/train-labels.idx1-ubyte", 2049)?;
    let answer_count = read_i32(&mut file)?;
    let mut buffer = Vec::with_capacity(answer_count as usize);

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn load_file(name: &str, expected_num: i32) -> std::io::Result<File>{
    let mut file = File::open(name)?;
    let magic_num = read_i32(&mut file)?;
    
    if magic_num != expected_num {
        panic!("Unexpected Magic Number: {}. Expected = {}", magic_num, expected_num);
    }

    Ok(file)
}

fn read_i32(file: &mut dyn Read) -> std::io::Result<i32>{
    let mut buf = [0; 4];

    file.read_exact(&mut buf)?;

    let val = ((buf[0] as i32) << 24) +
    ((buf[1] as i32) << 16) +
    ((buf[2] as i32) <<  8) +
    ((buf[3] as i32) <<  0);

    Ok(val)
}