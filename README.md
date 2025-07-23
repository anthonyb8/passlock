# 🔐 Passlock

**Passlock** is a simple, secure CLI tool for managing your passwords locally. It helps you store, encrypt, and retrieve credentials without relying on the cloud.

Passlock isn't a password manager in the traditional sense—it’s a lightweight, command-line vault designed for developers and privacy-conscious users.

## ✨ Features

- 🔒 Encrypted local storage (AES-GCM)
- 🧰 Command-line interface for storing and retrieving passwords
- 📋 Clipboard support for easy pasting
- 👥 Supports multiple credentials per platform

Store with confidence. Access with ease. All on your own machine.

## 🔧 Install

- To install or update run the below command in your terminal.

```bash
curl -sSfL https://raw.githubusercontent.com/anthonyb8/passlock/main/scripts/install.sh | bash
```

## :notebook: Commands

#### Create Project

** Creates directory & set-up **

```bash
passlock create
```

** Create set-up in current directory **

```bash

passlock update/insert <name>
```

#### Build

```bash
passlock delete
```

## 📦 Status

Early development — contributions welcome!

## License

This project is licensed under the Apache 2.0 License. See the [LICENSE](LICENSE) file for details.
