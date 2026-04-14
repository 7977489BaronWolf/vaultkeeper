# vaultkeeper

A lightweight CLI secrets manager that encrypts local env files using [age](https://age-encryption.org/) encryption.

---

## Installation

**From source:**

```bash
cargo install --path .
```

**Via cargo:**

```bash
cargo install vaultkeeper
```

---

## Usage

Initialize vaultkeeper in your project and manage encrypted env files with simple commands.

```bash
# Initialize and generate a key pair
vaultkeeper init

# Encrypt a .env file
vaultkeeper encrypt .env

# Decrypt to a temporary file or stdout
vaultkeeper decrypt .env.age

# Run a command with decrypted secrets injected into the environment
vaultkeeper run -- node server.js
```

Encrypted files (`.env.age`) can be safely committed to version control. Keep your private key out of the repo.

---

## Configuration

Vaultkeeper looks for a `vaultkeeper.toml` in your project root:

```toml
[keys]
public_key = "age1ql3z7hjy54pw3hyww5ayyfg7zqgvc7w3j2elw8zmrj2kg5sfn9aqmcac8p"
encrypted_files = [".env", ".env.production"]
```

---

## License

MIT © [vaultkeeper contributors](LICENSE)