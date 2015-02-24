extern crate rand;
extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha512;

use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

use rand::{ Rng, OsRng };


fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));

        final_result.push_all(write_buffer.take_read_buffer().take_remaining());

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.push_all(write_buffer.take_read_buffer().take_remaining());
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}



fn square(x: u32) ->  u32 {
    x * x
  }

fn main() {
  let y = 2;
  println!("y is {} square of y is : {}  \n", y, square(y));
for y in (1..9) {
 match y % 2 == 0 {
   true => println!("Even y = {} \n", y),
   false => println!("Odd y = {} \n", y)
   }
}
}

fn rng_ex() {
  let mut rng = rand::thread_rng();
  let mut nums = Vec::<u32>::new();
  let size = 1000000usize;
  nums.reserve(size);
  for _ in range(0usize, size) {
    nums.push(rng.gen::<u32>());
  }
  
}

#[test]
fn test_hash_sha_512() {
// create a Sha512 object
let mut hasher = Sha512::new();
// write input message
hasher.input_str("abc");
// read hash digest
let hex = hasher.result_str();
assert_eq!(hex.as_slice(),
    concat!("ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a",
"2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"));
}

#[test]
fn test_aes_cbc() {
    let message = "Hello World!";

    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];

    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);

    let encrypted_data = encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
    let decrypted_data = decrypt(&encrypted_data[..], &key, &iv).ok().unwrap();

    assert!(message.as_bytes() == &decrypted_data[..]);
}
