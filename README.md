# secure-password-manager

secure-password-manager is a cross-platform desktop application built with Tauri, React, and Rust for securely storing sensitive information locally. The application is designed with a local-first approach, ensuring that user data never leaves the device.

## Features

- Master password with local encryption (Argon2id + XChaCha20-Poly1305)
- Automatic re-lock on inactivity, manual lock (Ctrl+L), and lock on exit
- Optional "remember on this device" via the OS keychain
- Encrypted backups (`.vault`) and master-password rotation
- Website, server, database, email, API, and license credentials
- Environment-variable items with `.env` import / export
- Custom fields on any item
- Secret generator (passwords, passphrases, hex, base64, UUID)
- Copy with clipboard auto-clear; copy as `export KEY=VALUE` or a connection string
- Secure notes and entry-specific notes
- Global search
- Cross-platform support

## Technology Stack

- Tauri
- Rust
- React
- Vite

## Requirements

Before running the application, make sure the following software is installed:

- Node.js 20 or later
- Rust
- Cargo
- Tauri CLI

## Installation

Clone the repository:

```bash
git clone https://github.com/samuelhuicab/secure-password-manager.git
cd secure-password-manager
```

Install dependencies:

```bash
npm install
```

Start the development environment:

```bash
npm run tauri dev
```

## Building

To generate a production build:

```bash
npm run tauri build
```

The compiled binaries will be available in:

```
src-tauri/target/release
```

## Project Structure

```
src/
├── components/
├── hooks/
├── pages/
├── services/
├── stores/
├── types/
└── utils/

src-tauri/
├── src/
├── Cargo.toml
└── tauri.conf.json
```

## Security

Secure Vault follows a local-first architecture. User information is stored
exclusively on the local device and is never transmitted to external servers.

The vault file (`vault.json` in the app config directory) is encrypted at rest:

- The master password is stretched with **Argon2id** (OWASP parameters, stored
  in the file envelope so the cost can be raised later).
- The vault is sealed with **XChaCha20-Poly1305** (authenticated encryption, a
  fresh 24-byte nonce per save). Tampering with the file is detected on unlock.
- The derived key lives only in the Rust process memory (wrapped in `Zeroizing`)
  and, only if the user opts in, in the OS keychain.
- Writes are atomic (temp file + rename). The first run migrates an older
  plaintext vault only after a round-trip verification and keeps the original as
  `vault.v1.bak.json`.

## For developers

Day-to-day helpers built in:

- **`.env` items** — keep a project's environment in one encrypted entry;
  export it back to a `.env` file or import an existing one.
- **Connection strings** — copy a ready `postgresql://…` / `mysql://…` /
  `mongodb://…` URL from a database entry, or an `ssh user@host -p port` line.
- **`export KEY=VALUE`** — copy any field in shell-ready form.
- **Clipboard auto-clear** — secrets are wiped from the clipboard after a
  configurable delay.
- **Secret generator** — passwords, passphrases, hex/base64 tokens and UUIDs
  from a CSPRNG, with an entropy estimate.
- **One-click SSH** — open a terminal session to a server entry (host/user are
  validated to avoid shell injection).

## Contributing

Contributions are welcome.

1. Fork the repository.
2. Create a feature branch.
3. Commit your changes.
4. Open a Pull Request.

Please ensure that new code follows the project's coding conventions and includes appropriate documentation.

## License

This project is licensed under the MIT License.
