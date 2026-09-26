# ASCII Galaxy Intro

A high-performance, lightweight command-line utility written in **Rust** that transforms any image (PNG, JPEG) into stunning ASCII art. Perfect for customizing your Linux terminal startup sequence, creating retro aesthetics, or integrating into your `~/.bashrc`.

---

## Features

- **Dynamic Image Conversion:** Pass any image path as an argument to convert it on the fly.

- **High Performance:** Built in Rust for lightning-fast execution and minimal resource usage.

- **Terminal Friendly:** Automatically resizes and maps RGB pixels to a gradient ASCII character set based on luminance.

- **Shell Integration:** Seamlessly chain it with tools like `fastfetch` or `btop` for a custom startup experience.

---

## Prerequisites

To compile and run this project from source, you need:

- **Rust** (Cargo) installed on your system.

- Basic image libraries handled automatically by Cargo.

---

## Installation

### 1. Clone the Repository
```bash
git clone https://github.com/votre-nom-utilisateur/ascii-galaxy.git
cd ascii-galaxy
```

### 2. Build the Release Binary
Compile the project in optimized release mode:
```bash
cargo build --release
```

### 3. Install Globally (Optional)
Move the compiled binary to your system's PATH so you can run it from anywhere:
```bash
sudo cp target/release/ascii-galaxy /usr/local/bin/
```

---

## Usage

Run the utility by passing the path to your target image:

```bash
ascii-galaxy path/to/your/image.png
```

### Integrating into your Shell (`~/.bashrc`)

To use your custom ASCII art as a startup intro every time you open your terminal:

1. Open your configuration file using an editor like `micro`:
   ```bash
   micro ~/.bashrc
   ```
2. Add the following lines at the bottom of the file:
   ```bash
   # Custom ASCII image intro followed by system info
   ascii-galaxy ~/Pictures/galaxy.png
   fastfetch
   ```
3. Save and restart your terminal.

---

## License

This project is open source and available under the terms of the **MIT License**. 
Copyright (c) 2026 Jorge Andre Castro

```text
MIT License

Copyright (c) 2026 ASCII Galaxy Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```