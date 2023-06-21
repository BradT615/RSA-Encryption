# RSA-Encryption

This program demonstrates RSA encryption and decryption in Rust. It generates public and private key pairs, encrypts a message using the public key, and then decrypts the encrypted message using the private key.

## Dependencies

This program uses the `std::fs` module from the Rust standard library to interact with the file system and perform file operations.

## Usage

To use this program, follow these steps:

1. Make sure you have Rust installed on your system. If not, you can install it from the official Rust website: [https://www.rust-lang.org](https://www.rust-lang.org).

2. Create a new Rust project or navigate to an existing Rust project's directory.

3. Create a new Rust source file (e.g., `main.rs`) and copy the contents of this program into the file.

4. Save the file and compile the program by running the following command in your project's directory:

   ```shell
   $ cargo build
   ```

5. Run the program using the following command:

   ```shell
   $ cargo run
   ```

## Program Explanation

The program performs the following steps:

1. The `delete_file` function is called to delete any existing `pub.key` and `priv.key` files from previous runs.

2. The `generate` function generates a pair of public and private keys using RSA. It calculates the values of `p`, `q`, `n`, `t`, `e`, and `d` based on specific mathematical calculations and writes them to the `pub.key` and `priv.key` files, respectively.

3. The `encrypt` function reads the public key from the `pub.key` file and encrypts a predefined message using the RSA algorithm. It converts the characters of the message to ASCII values, performs modular exponentiation, and stores the encrypted values in a vector. It then prints the encrypted ASCII values and the corresponding ciphertext.

4. The `decrypt` function reads the private key from the `priv.key` file and decrypts the encrypted message received as input. It performs modular exponentiation to obtain the original ASCII values and converts them back to characters. Finally, it prints the decrypted message.

5. The `conversion` function is a helper function used for modular exponentiation.

6. The `delete_file` function deletes the `pub.key` and `priv.key` files.

## Notes

- This program is for educational purposes and may not provide a production-ready implementation of RSA encryption. It is recommended to use well-tested libraries for cryptographic operations in real-world applications.

- Make sure to keep the generated private key (`priv.key`) secure, as it is crucial for decrypting messages.

- The current implementation uses small prime numbers (`p = 11` and `q = 17`) for simplicity. In practice, much larger prime numbers are used for enhanced security.

- The program assumes the presence of the `pub.key` and `priv.key` files in the working directory. If they don't exist, the program will create them. If they exist, the program will overwrite their contents.

- The predefined message used in the `encrypt` function can be modified to encrypt different messages.

- The `delete_file` function is provided for convenience to delete the key files. If you don't want to delete the files, you can comment out the function call.

- The `allow(dead_code)` and `allow(unused_must_use)` attributes are added to suppress compiler warnings for unused code. They can be removed if desired.

## License

This program is released under the [MIT License](LICENSE). Feel free to modify and distribute it as needed.
