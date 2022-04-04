from decode import *
import numpy as np
from PIL import Image
import sys

if len(sys.argv) < 4:
    print("not enough arguments")
    sys.exit()

diffs_path = sys.argv[1]
image_path = sys.argv[2]
timestamp = int(sys.argv[3])

decoder = PlaceDecoder(diffs_path)

grid = np.empty((2000, 2000), dtype=int)

while True:
    blob = decoder.next_blob()
    if blob == None or blob.time > timestamp:
        break

    for diff in blob.diffs:
        grid[diff.x, diff.y] = diff.color

image = Image.new("RGB", (2000, 2000))
pixbuf = image.load()

for x in range(2000):
    for y in range(2000):
        color = grid[x, y]
        r = (color & 0xFF0000) >> 16
        g = (color & 0xFF00) >> 8
        b = color & 0xFF
        pixbuf[x, y] = (r, g, b)

image.save(image_path)
