# 🎞️ video_to_ascii_colored

A terminal-based Rust application that converts videos into colorful ASCII art and plays them directly in your terminal.

---

## ✨ Features

- 🎥 **Video to ASCII**: Convert videos of various formats into ASCII art.
- 🌈 **Colored Rendering**: Retains original video color information for vibrant terminal playback.
- 🖥️ **Terminal Playback**: Watch your favorite videos in ASCII form, directly in your terminal.

---

## ⚙️ Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [FFmpeg](https://ffmpeg.org/download.html)

---

## 🛠️ Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/kamalnayan10/video_to_ascii_colored.git
   cd video_to_ascii_colored
   ```

2. **Install FFmpeg**:

   On Ubuntu:

   ```bash
   sudo apt update
   sudo apt install ffmpeg
   ```

   *(Use your distribution’s package manager if not on Ubuntu.)*

3. **Build the project**:

   ```bash
   cargo build --release
   ```

---

## ▶️ Usage

```bash
cargo run --release {path_to_video}
```
---

## 🐧 Troubleshooting

If you run into build issues (especially on a fresh system), run:

```bash
sudo apt-get update
sudo apt install build-essential
```

---

## 📽️ Demo

https://github.com/kamalnayan10/video_to_ascii_colored/assets/22148142/31847f24-5cc4-49f1-b05c-352afce2defa

---

DEMO
    

## 👥 Contributors

- [@kamalnayan10](https://github.com/kamalnayan10)
- [@Plasmakatana](https://github.com/Plasmakatana)

---

## 📄 License

This project is licensed under the **MIT License**.

You are free to use, modify, and distribute this software for both personal and commercial purposes, provided proper attribution is given.
