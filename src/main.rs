
fn char_to_number(c: char) -> i32{
    let res = (c as u8 - 'a' as u8) as i32;
    return res;
}

fn number_to_char(x: i32) -> char{
    return (x + 'a' as u8 as i32) as u8 as char
}


fn multiply_vec_matrix(part: Vec<i32>, matrix: &Vec<i32>) -> Vec<i32> {
    let res = vec!(part[0] * matrix[0] + part[1] * matrix[2], part[0] * matrix[1] + part[1] * matrix[3]);
    return res.into_iter().map(|x| x.rem_euclid(26)).collect::<Vec<i32>>();
    // return res;
}

fn encode_hill(key: Vec<i32>, text: &String) -> String{

    let text_as_numbers = text.chars().map(|c| char_to_number(c)).collect::<Vec<i32>>();
    let part1 = vec!(text_as_numbers[0], text_as_numbers[1]);
    let part2  = vec!(text_as_numbers[2], text_as_numbers[3]);

    let encrypted_part1 = multiply_vec_matrix(part1, &key);
    let encrypted_part2 = multiply_vec_matrix(part2, &key);
    let res =  encrypted_part1.into_iter().chain(encrypted_part2.into_iter())
        .map(|x| number_to_char(x))
        .map(|c| c.to_string())
        .reduce(|acc, c| [acc, c].join(""))
        .expect("this should never happen");
    return res;
}

fn find_det(matrix: &Vec<i32>) -> i32{
    return 1;
}

fn inverse_matrix(matrix: &Vec<i32>) -> Vec<i32>{
    let inverse_det = 1;
    let res =  vec!(matrix[3], -1 * matrix[1], -1 * matrix[2], matrix[0]).into_iter()
        .map(|x| x * inverse_det).collect::<Vec<i32>>();
    return res;
    // return vec!();
}

fn z2(){
    let matrix: Vec<i32> = vec!(11, 8, 3, 7);
    println!("matrix: {:?}", matrix);

    let matrix_inv = vec!(7, 18, 23, 11);
    let matrix_inv2 = inverse_matrix(&matrix);
    println!("matrix_inv: {:?}", matrix);
    println!("matrix_inv: {:?}", matrix);

    println!("Encryption");

    let plaintext = "july".to_string();

    println!("Plaintext: {plaintext}");
    let cyphertext = encode_hill(matrix, &plaintext);
    println!("Cyphertext: {}", cyphertext.to_uppercase());

    println!();
    println!("Decryption");
    let decrypted = encode_hill(matrix_inv2, &cyphertext);
    println!("Plaintext: {decrypted}");
    println!("Cyphertext: {}", cyphertext.to_uppercase());

}

fn main() {
    z2();



}
