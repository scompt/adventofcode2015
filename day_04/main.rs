use md5;

fn main() {
    let secret_key = "yzbqklnj";

    let mut index = 0;
    loop {
        index += 1;
        let input = format!("{}{}", secret_key, index);
        let digest = md5::compute(input.as_bytes());

        if digest[0] == 0 && digest[1] == 0 && digest[2] < 16 {
            println!("{:?} {}", digest, index);
            if digest[2] == 0 {
                return;
            }
        }
    }
}
