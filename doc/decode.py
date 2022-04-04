from dataclasses import dataclass
from typing import List
import sys

place_colors = [
    0x6D001A,
    0xBE0039,
    0xFF4500,
    0xFFA800,
    0xFFD635,
    0xFFF8B8,
    0x00A368,
    0x00CC78,
    0x7EED56,
    0x00756F,
    0x009EAA,
    0x00CCC0,
    0x2450A4,
    0x3690EA,
    0x51E9F4,
    0x493AC1,
    0x6A5CFF,
    0x94B3FF,
    0x811E9F,
    0xB44AC0,
    0xE4ABFF,
    0xDE107F,
    0xFF3881,
    0xFF99AA,
    0x6D482F,
    0x9C6926,
    0xFFB470,
    0x000000,
    0x515252,
    0x898D90,
    0xD4D7D9,
    0xFFFFFF
]

@dataclass
class PlaceBlob:
    time: int
    size: int
    diffs: List

@dataclass
class PlaceDiff:
    x: int
    y: int
    color: int

class PlaceDecoder:
    def __init__(self, path):
        self.path = path
        self.file = open(path, "rb")

    def __del__(self):
        self.file.close()

    def next_blob(self):
        # create Blob
        time = int.from_bytes(self.file.read(8), byteorder="little")
        size = int.from_bytes(self.file.read(4), byteorder="little")

        if time == 0 or size == 0:
            return None

        diffs = []
        for _ in range(size):
            data = int.from_bytes(self.file.read(4), byteorder="little")
            x = data & 0xFFF
            y = (data & 0xFFF000) >> 12;
            c = (data & 0xFF000000) >> 24;

            diffs.append(PlaceDiff(x, y, place_colors[c]))

        return PlaceBlob(time, size, diffs)

# example usage
if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("expected a file")
        sys.exit()

    d = PlaceDecoder(sys.argv[1])
    blobs = 0

    while True:
        blob = d.next_blob()
        if blob == None:
            break

        for diff in blob.diffs:
            print(f"diff (x: {diff.x}, y: {diff.y}, color: {hex(diff.color)})");

        print(f"blob (time: {blob.time}, size: {blob.size})")
        blobs += 1

    print(f"blob count: {blobs}")
