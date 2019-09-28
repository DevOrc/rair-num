mod io;

use io::FileData;

const CHAR_IMG_SIZE: usize = 28;

pub struct CharImage{
    data: Vec<f32>,
    answer: u8,
}

fn main() {
   let mut file_data = io::load_data().unwrap();

   for _ in 0..20{
        let img = file_data.load_image().unwrap();

        print_image(&img);
   }

   println!("Answer Count: {}", file_data.answers.len());
}

fn print_image(img: &CharImage){
    println!("---------------------------------");
    
    for row in 0..CHAR_IMG_SIZE{
        for column in 0..CHAR_IMG_SIZE{
            let index = (row * CHAR_IMG_SIZE) + column;
            let value = img.data[index];

            if value >= 0.1{
                print!("██");
            }else{
                print!("  ");
            }
        }

        println!();
    }

    println!("Answer: {}", img.answer);
    println!("---------------------------------")
}