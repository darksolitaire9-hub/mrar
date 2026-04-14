# mrar

`mrar` is a small CLI tool that removes metadata from images and writes cleaned copies to an `output` folder.

It is useful for stripping EXIF data such as GPS coordinates, camera details, and other embedded metadata before sharing images.

## Download

### Windows

Download `mrar-windows-x86_64.exe` from the [Releases](https://github.com/darksolitaire9-hub/mrar/releases) page.

Then run it from PowerShell or Command Prompt:

```powershell
mrar-windows-x86_64.exe F:\inputs\mar-inputs\apr
```

Example output:

```text
Found 110 image(s) → output: F:\inputs\mar-inputs\apr\output
manifest written → F:\inputs\mar-inputs\apr\output\manifest.json
Done. 110/110 processed. 2858680 bytes stripped across all images.
```

## Build from source

If you have Rust installed, you can build it yourself:

```bash
git clone https://github.com/darksolitaire9-hub/mrar.git
cd mrar
cargo build --release
```

The binary will be created at:

```text
target/release/mrar.exe
```

Rust is typically installed via `rustup`, which also installs Cargo.

## Usage

Pass a directory containing supported images:

```powershell
mrar-windows-x86_64.exe <input-folder>
```

`mrar` will:

- Find supported images in the input folder.
- Remove metadata from each image.
- Write cleaned files to an `output` subfolder.
- Write a `manifest.json` file with processing details.

## Verify metadata removal

You can visually inspect metadata on Windows by right-clicking an image, opening **Properties**, and checking the **Details** tab; fields like camera model, GPS, and date taken should be empty or absent on processed files.

## Notes

- Originals are not modified; processed files are written to `output`.
- This is a local/offline tool; images are processed on your machine.