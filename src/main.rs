use array_macro::convert_to_custom_array;
convert_to_custom_array!(

fn main() {
    let asdf = [6; 300; 3];
    let mut qwer: [2; 3; u8] = [2; 3; 2];
    qwer[0][2] = 7;
    println!("{} {}", asdf[3][200], qwer[0][2]);
}

);