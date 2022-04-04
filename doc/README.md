# place archive

~~See the [Releases](https://github.com/woofdoggo/placeclient/releases) page to
download the raw data. The archives contain data from April 2 at circa 2:20 PM
Eastern Time onwards, with some gaps interspersed throughout early on due to
mishaps on my part with the scraper.~~

> The full dataset will be made available once the event ends.

Check out ProstoSanja's [place-2022](https://github.com/ProstoSanja/place-2022)
project for a more complete dataset. Theirs contains full images, while my
dataset contains a sequence of changes to the image at a greater frequency
(and unfortunately has more gaps of downtime.)

# Licensing

- All published data here is licensed under [CC0](https://github.com/woofdoggo/placeclient/blob/main/LICENSE-CC0)
- All source code and examples in **this folder** are under [CC0](https://github.com/woofdoggo/placeclient/blob/main/LICENSE-CC0)

If you use this data, credit would be appreciated (but is not required.)
A link to this page would suffice.

With the exception of [Wuffs](https://github.com/woofdoggo/placeclient/blob/main/preprocessor/wuffs.c),
which is under the [Apache 2.0](https://github.com/woofdoggo/placeclient/blob/main/LICENSE-WUFFS)
license, all other source code contained within this repository is under the
[BSD 2-clause license](https://github.com/woofdoggo/placeclient/blob/main/LICENSE).

# Table of Contents

- [Examples](#examples)
- [Data Format](#data-format)
  - [Diff Blobs](#diff-blobs)
  - [Diff Elements](#diff-elements)
  - [Color Table](#color-table)

# Examples
TODO

# Data Format

All files with the `.bin` extension are uncompressed. `.bin.zstd` files have
been compressed with [Zstandard](https://github.com/facebook/zstd); feel free
to download those if you are able to decompress zstd data and want a faster
download.

## Diff Blobs

The uncompressed data files consist of a series of "diff blobs," containing all
of the changes sent by the Reddit server at a given unix timestamp. These diff
blobs are sorted by timestamp, from least to greatest. Here is a simple mockup
of the "diff blob" struct in C:

```c
typedef struct {
    uint64_t time;          // unix millis timestamp
    uint32_t count;         // the number of diff elements in this blob
    uint32_t diffs[count];  // array of diff elements
} DiffBlob;
```

The file contains nothing but these blobs, laid in sequence one after another.
There is no padding between blobs; all numbers are stored as little-endian.

## Diff Elements

Each diff element contains information about the position and color of the
diff, packed in a simple manner to save space.

- x: bits 0-11
- y: bits 12-23
- color: bits 24-31

Here is a simple snippet of C code to recover these values from a diff element:

```c
void extract_diff(uint32_t diff) {
    uint16_t x     = diff & 0x00000FFF;
    uint16_t y     = (diff & 0x00FFF000) >> 12;
    uint32_t index = (diff & 0xFF000000) >> 24;

    // return or use these values as you see fit
}
```

## Color Table

`index` from the above example is a 0-based index into the following table of
hexadecimal colors, stored in the format RRGGBB:

```cpp
std::unordered_map<uint32_t, uint8_t> colors {
    { 0, 0x6D001A },
    { 1, 0xBE0039 },
    { 2, 0xFF4500 },
    { 3, 0xFFA800 },
    { 4, 0xFFD635 },
    { 5, 0xFFF8B8 },
    { 6, 0x00A368 },
    { 7, 0x00CC78 },
    { 8, 0x7EED56 },
    { 9, 0x00756F },
    { 10, 0x009EAA },
    { 11, 0x00CCC0 },
    { 12, 0x2450A4 },
    { 13, 0x3690EA },
    { 14, 0x51E9F4 },
    { 15, 0x493AC1 },
    { 16, 0x6A5CFF },
    { 17, 0x94B3FF },
    { 18, 0x811E9F },
    { 19, 0xB44AC0 },
    { 20, 0xE4ABFF },
    { 21, 0xDE107F },
    { 22, 0xFF3881 },
    { 23, 0xFF99AA },
    { 24, 0x6D482F },
    { 25, 0x9C6926 },
    { 26, 0xFFB470 },
    { 27, 0x000000 },
    { 28, 0x515252 },
    { 29, 0x898D90 },
    { 30, 0xD4D7D9 },
    { 31, 0xFFFFFF }
};
```
