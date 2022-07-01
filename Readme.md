# Exif GPS Coord extractor

This little tool extracts GPS coordinates from images (JPeg and Tiff).

## Usage

```bash
coord-extractor <image.jpg> <exit-file.txt>
```

## Behaviour

- It reads the EXIF info from the image file and writes the coordinates to the exit file.
- If the exit file already exists, it will be overwritten.
- If the image file has no EXIF information, an error is thrown and the program exits. No file is written.
- If the image has an EXIF information, but it does not have a GPS coordinates, no error is thrown and an empty file is written.
- If you don't pass the 2 filenames, an usage message is shown.
- If the image file does not exist, an error is thrown and the program exits.
- If the image cannot be read, an error is thrown and the program exits.
