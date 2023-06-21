

use std::fs::remove_file;
use std::io::Write;
use std::io::Read;

fn main() {
  delete_file("pub.key", "priv.key");
  generate();
  let encrypted_message = encrypt();
  decrypt(encrypted_message);
}

fn generate() {
  let p = 11;
  let q = 17;
  let n = p * q;
  let t = (p - 1) * (q - 1);
  
  let mut e = 0;
  for x in 1..t {
    if t % x != 0 {
      e = x;
      break;
    }
  }

  println!("\np = {}", p);
  println!("q = {}", q);
  println!("n = {}", n);
  println!("t = {}", t);
  println!("e = {}", e);

  let mut d = 1;
  let mut x = 1;
  //d∙e = 1 + an integer multiple of t.
  while d == 1 {
    let temp = x*e;
    for count in 2..(temp + 1) {
      if temp == (1 + (count * t)) {
        d = x;
        break;
      }
    }
    x = x + 1;
  }

  println!("d = {}\n", d);

  let mut public = std::fs::File::create("pub.key").unwrap();
  write!(public, "{} {}", e, n).unwrap();
  let mut private = std::fs::File::create("priv.key").unwrap();
  write!(private, "{} {}", d, n).unwrap();
}

fn encrypt() -> Vec<u64> {
  let str = "The greatest discovery of all time is that a person can change his future by merely changing his attitude";
  println!("\n\nOriginal Message \n\n{}\n\n", str);


  let mut file = std::fs::File::open("pub.key").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut split = contents.split(" ");
  let (e_str, n_str) = (split.next().unwrap(), split.next().unwrap());

  let e: u64 = e_str.parse().expect("failed to parse e_str");
  let n: u64 = n_str.parse().expect("Failed to parse n_str");

  println!("\nPublic key: e = {}, n = {}", e, n);

  let mut chars = Vec::new();
  let mut ascii = Vec::new();

  for c in str.chars() {
    chars.push(c);
  }

  for c in str.chars() {
    let char_string = c.to_string();
    let char_slice = char_string.as_bytes(); //&[u8]
    if char_slice[0] >= 65 && char_slice[0] <= 90{
      ascii.push(char_slice[0] - 63);
    }
    else if char_slice[0] >= 97 && char_slice[0] <= 122 {
      ascii.push(char_slice[0] - 70);
    }
    else {
      ascii.push(char_slice[0] - 31);
    }
  }
  

  //C = P^e mod n. 
  let mut encrypted_chars = Vec::new();
  println!("\nEncrypted ascii Message\n");
  for c in 0..ascii.len() {
    let mut temp = ascii[c] as u64;
    temp = temp.pow(e as u32) % n;
    encrypted_chars.push(temp);
    print!("{} ", encrypted_chars[c]);
  }
  println!("\n\nEncrypted Ciphertext\n");
  for i in 0..chars.len() {
    print!("{} ", encrypted_chars[i] as u8 as char);
  }
  encrypted_chars
}

#[allow(dead_code)]
fn decrypt(encrypted_message: Vec<u64>) {

  let mut file = std::fs::File::open("priv.key").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();


  let mut split = contents.split(" ");
  let (d_str, n_str) = (split.next().unwrap(), split.next().unwrap());

  let d: u64 = d_str.parse().expect("failed to parse e_str");
  let n: u64 = n_str.parse().expect("Failed to parse n_str");

  println!("\n\n\n\nPrivate Key: d = {}, n = {}\n", d, n);

  let mut decrypted_chars = Vec::new();
  //P = C^d mod n
  for c in 0..encrypted_message.len() {
    let temp = encrypted_message[c];
    let decrypted_temp = conversion(temp, d, n);
    decrypted_chars.push(decrypted_temp);
  }
  
  println!("Decrypted Message\n");
  for i in 0..decrypted_chars.len() {
    let decrypted_char = if decrypted_chars[i] > 1 &&  decrypted_chars[i] <= 26 {
      (decrypted_chars[i] + 63) as u8
    } else if decrypted_chars[i] >= 27 && decrypted_chars[i] <= 52 {
      (decrypted_chars[i] + 70) as u8
    } else {
      (decrypted_chars[i] + 31) as u8
    };
    print!("{}", decrypted_char as char);
  }
}

fn conversion(base: u64, exponent: u64, n: u64) -> u64 {
  let mut result = 1;
  let mut base = base % n;
  let mut exponent = exponent;

  while exponent > 0 {
    if exponent % 2 == 1 {
      result = (result * base) % n;
    }
    exponent >>= 1;
    base = (base * base) % n;
  }
  result
}

#[allow(dead_code)]
#[allow(unused_must_use)]
fn delete_file(file1: &str, file2: &str) {
  remove_file(file1);
  remove_file(file2);
}