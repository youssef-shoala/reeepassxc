Name: **Youssef Shoala**
Student ID: **1006220144**
Email: **youssef.shoala\@mail.utoronto.ca**

# Motivation

In today's digital landscape, individuals and organizations face a significant challenge in managing secure access to numerous accounts. Storing and maintaining complex, unique passwords for each account is essential for strong security, yet it is often difficult to achieve without a reliable password management tool. While existing password managers, such as KeepassXC (written in C++), offer functionality, no well-established password managers are currently written in Rust—a language specifically designed with memory safety and concurrency safety in mind. 

Given that password managers are a prime target for attackers, Rust’s robust safety features, such as its memory safety and concurrency guarantees, provide a unique advantage for minimizing vulnerabilities and preventing exploits. Writing a password manager in Rust reduces the attack surface of the codebase, enhancing security. This project addresses a notable gap in the Rust ecosystem by delivering a secure, open-source password manager built from the ground up in Rust.

# Objective and Key Features

The project aims to develop a fully featured CLI password manager with end-to-end encryption. The tool should securely store, retrieve, and delete passwords locally from a password file. All data will be encrypted using a master password, ensuring that it remains protected even in cases of device theft or unauthorized access.

The project will implement the following key features:

1. **Secure Storage and Encryption**:
   - Username, password, and a service name will be stored as required fields.
       - Option to add URL, notes, and tags.
   - Passwords and sensitive data will be encrypted using the AES-256 encryption standard.
   - All encryption and decryption operations will occur locally on the user’s device.
   - Passwords will be stored in an encrypted file format and it's path can be determined by the user.
       - Multiple password file management is supported. 

2. **Master Password-Based Key Derivation**:
   - The master password will be used to derive a cryptographic key using a secure hashing algorithm.
   - Only this master key will be able to decrypt the stored passwords, ensuring that even if the data file is accessed, it remains protected without the master password.

3. **Password Generation**: 
   - Built-in tool to generate strong, random passwords. 

4. **CLI for Password Management**:
   - A straightforward command-line interface will allow users to interact with password file and perform operations like adding, retrieving, and deleting passwords. 
       - The tool should automatically prompt for the master password similar to executing the sudo command. 

# Installation/Reproducibility Guide

To use ReeePassXC, you can clone and build the code from github. In the future, several package managers will be implemented to add the binaries to PATH. 

Clone source code: 

`$ git clone https://github.com/youssef-shoala/reeepassxc.git`

Move into working directory: 

`$ cd reeepassxc`

Build binaries: 

`$ cargo build --release`

Use (shows help message):

`$ ./target/debug/reeepassxc`

# User Guide

## Create Vault

A vault is an abstraction of a folder that has been compressed using the Deflate compression algorithm and encrypted with a user set password using the AES-256 encryption algorithm. The Deflate compression algorithm was specifically chosen for its compatibility and implementation for many operating systems. The AES-256 encryption algorithm was chosen for its large key size and security. 

`./target/debug/reeepassxc create <vault_name> <Optional: group_name>`

## List Vaults

The encrypted vaults living in the vaults folder will be read and returned by using the following command. A group can be optionally provided to only list vaults of that group. 

`./target/debug/reeepassxc list <Optional: group_name>`

## Delete Vault

To delete a vault, run the following command. A group can be provided optionally. If a vault lives in a group, the group must be provided to delete it, otherwise it will try to delete a vault of the same name in the root vaults folder. 

`./target/debug/reeepassxc delete <vault_name> <Optional: group_name>`

## Open Vault

Unencrypt & decompress a vault, creating an OpenVault object that is used to manipulate the vault entries. Enters CLI mode on execution. Automatically closes by compressing and encrypting, also cleans up open vault working directory and components with sensitive data. 

`./target/debug/reeepassxc open <vault_name> <Optional: group_name>`

CLI mode: 
- `list`: list entries in vault
- `add`: add entry to vault (password generation option exists here)
- `delete`: delete entry in vault by username

# Video Demo

[Link](https://www.youtube.com/watch?v=ZZz4yn1QjOQ)

# Contribution

The project was completed in full by myself, Youssef Shoala ID:1006220144. 

This project will be continued as an open source project under the MIT license. 

## Pull Requests

1. **Fork the Repository:**

    - Click the "Fork" button on the GitHub page.
    
2. **Create a Branch:**
    
`git checkout -b feature-branch`
    
3. **Make Changes and Commit:**
    
`git add .`
    
`git commit -m "Description of changes"`
    
4. **Push to the Branch:**
    
`git push origin feature-branch`
    
5. **Create a Pull Request:**
    
    - Go to the repository on GitHub and click "New Pull Request".

# Lessons Learned

## KBDX Format

The project involved a deep study of the KBDX format, which is commonly used for secure password storage, as seen in tools like KeePassXC. While the format provides a robust structure for storing encrypted password databases, we discovered that careful handling of its internal components—such as headers, encryption keys, and compressed payloads—is critical for ensuring both security and interoperability with other tools. Although the features are not yet in place, a plan to increase compression and encryption compatibility for ReeePassXC such that conversion between the KBDX and RBDX format is possible.   

Proposed RBDX Header Fields: 

- Outer header (raw bytes, unencrypted)
    - Headerfield N 
        - type: Uint8  
        - size: Uint32  
        - data: byte\[Size]  
    - outer header field types:
        - required: 
            - id: 0: end of header (bytes:0d 0a 0d 0a)
            - id: 1:  name (String)
            - id: 4:  CipherID (UUID 16B) (default = chacha20 w/o poly1305 or aes256cbc)
            - id: 5:  Compression (UInt32)
            - id: 6:  KDF-Seed (byte\[32]) (random data for key der)
            - id: 7:  Encryption IV (byte\[]) (size variable to cipher)
            - id: 8:  KDF parameters (VariantMap)
        - optional: 
            - id: 2:  group (String) (default = None)
            - id: 3:  tag (Vec\<String>) (default = None)

- Inner header (encrypted)
     - Headerfield N 
        - type: Uint8 
        - size: Uint32 
        - data: byte\[Size] 
    - inner header field types: 
        - required: 
            - id: 0: end of header (bytes:0d 0a 0d 0a)
            - id: 1:  protected cipher
            - id: 2:  protected key (derived from either master pass or protected pass)
        - optional: 
            - id: 3:  totp cipher
            - id: 4:  totp key

Additionally, adhering to the KBDX specification highlighted the importance of maintaining consistent encryption and integrity mechanisms across various operations. For future iterations, validating the implementation against existing KeePassXC files and adhering to newer specifications of KBDX would improve the reliability of the project.

## Rust Memory Protections

While Rust was chosen to reduce the attack surface through memory safety guarantees, we identified several areas where sensitive data handling could be further improved:

- **Secure Memory Management:**  
    By default, Rust's memory handling does not ensure that sensitive data (like passwords and cryptographic keys) is wiped after use. Libraries such as [RustCrypto](https://github.com/RustCrypto) crates `zeroize` and `secrecy` are valuable tools for securely managing sensitive data. For example, the `zeroize` crate ensures that values are erased from memory upon deallocation, preventing data remnants that could be exploited.
    
- **Avoiding Memory Duplication:**  
    String manipulation in Rust can inadvertently duplicate sensitive data in memory due to intermediate values or string copies. To mitigate this risk:
    - Use **byte slices (`[u8]`)** rather than `String` or `Vec<u8>` where possible.
    - Minimize intermediate computations and values when handling secrets.
    - Lock memory pages to prevent swapping by leveraging crates like [`memsec`](https://docs.rs/memsec/latest/memsec/).
    
- **Block Scoping:**  
    Limiting the lifetime of sensitive variables using block scoping helped reduce the exposure of secrets in memory.
    
- **Secure Random Number Generation:**  
    For generating cryptographic keys, we integrated a secure random number generator like `rand::rngs::OsRng`, which leverages the operating system’s entropy for reliable randomness. This ensures that secrets are securely generated without predictable patterns.
    

Overall, while Rust provides a safer programming foundation, explicit tools and practices are still required to enhance the handling of sensitive data and prevent leakage.


## Final Thoughts

This project provided valuable insights into building a secure password manager using Rust. While Rust’s memory safety guarantees offer significant advantages, explicit measures—such as secure memory handling, encryption best practices, and external audits—are necessary to ensure robust security. Future iterations will aim to integrate more advanced protections, such as authenticated encryption, Argon2 key derivation, and formal security audits, to align with industry best practices.
