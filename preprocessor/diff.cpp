// this file is based off of wuffs/example/imageviewer/imageviewer.cc
//
// Copyright 2020 The Wuffs Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <errno.h>
#include <dirent.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <algorithm>
#include <cstring>
#include <unordered_map>
#include <vector>

#define WUFFS_IMPLEMENTATION
#define WUFFS_CONFIG__MODULES
#define WUFFS_CONFIG__MODULE__ADLER32
#define WUFFS_CONFIG__MODULE__AUX__BASE
#define WUFFS_CONFIG__MODULE__AUX__IMAGE
#define WUFFS_CONFIG__MODULE__BASE
#define WUFFS_CONFIG__MODULE__CRC32
#define WUFFS_CONFIG__MODULE__DEFLATE
#define WUFFS_CONFIG__MODULE__LZW
#define WUFFS_CONFIG__MODULE__PNG
#define WUFFS_CONFIG__MODULE__ZLIB
#include "wuffs.c"

void die(std::string msg) {
    fprintf(stderr, "%s\n", msg.c_str());
    exit(1);
}

bool sortbyname(std::string a, std::string b) {
    char filetime[14];
    filetime[13] = '\0';

    memcpy(filetime, a.c_str(), 13);
    uint64_t at = atoll(filetime);

    memcpy(filetime, b.c_str(), 13);
    uint64_t bt = atoll(filetime);

    return at < bt;
}

int main(int argc, char *const argv[]) {
    std::unordered_map<uint32_t, uint8_t> colors {
        { 0xFF6D001A, 0 },
        { 0xFFBE0039, 1 },
        { 0xFFFF4500, 2 },
        { 0xFFFFA800, 3 },
        { 0xFFFFD635, 4 },
        { 0xFFFFF8B8, 5 },
        { 0xFF00A368, 6 },
        { 0xFF00CC78, 7 },
        { 0xFF7EED56, 8 },
        { 0xFF00756F, 9 },
        { 0xFF009EAA, 10 },
        { 0xFF00CCC0, 11 },
        { 0xFF2450A4, 12 },
        { 0xFF3690EA, 13 },
        { 0xFF51E9F4, 14 },
        { 0xFF493AC1, 15 },
        { 0xFF6A5CFF, 16 },
        { 0xFF94B3FF, 17 },
        { 0xFF811E9F, 18 },
        { 0xFFB44AC0, 19 },
        { 0xFFE4ABFF, 20 },
        { 0xFFDE107F, 21 },
        { 0xFFFF3881, 22 },
        { 0xFFFF99AA, 23 },
        { 0xFF6D482F, 24 },
        { 0xFF9C6926, 25 },
        { 0xFFFFB470, 26 },
        { 0xFF000000, 27 },
        { 0xFF515252, 28 },
        { 0xFF898D90, 29 },
        { 0xFFD4D7D9, 30 },
        { 0xFFFFFFFF, 31 }
    };

#ifdef READ_TRUTH
    if (argc < 5) {
#else
    if (argc < 4) {
#endif
        die("not enough arguments");
    }

    FILE *truthwrite = fopen(argv[3], "w");
    if (truthwrite == NULL) {
        die(strerror(errno));
    }
    printf("opened truthwrite\n");

    FILE *writefh = fopen(argv[2], "w");
    if (writefh == NULL) {
        die(strerror(errno));
    }
    printf("opened writefh\n");

    DIR* dir;
    struct dirent *direntry;
    dir = opendir(argv[1]);
    if (dir == NULL) {
        die(strerror(errno));
    }
    printf("opened directory\n");

    auto files = new std::vector<std::string>();
    uint32_t *truth = (uint32_t*)malloc(sizeof(uint32_t) * 2000 * 2000);
    if (truth == NULL) {
        die("fail alloc image");
    }

#ifdef READ_TRUTH
    FILE *truthfh = fopen(argv[4], "r");
    if (truthfh == NULL) {
        die("fail read truth");
    }

    fread(truth, sizeof(uint32_t), 2000 * 2000, truthfh);
    printf("read truth source\n");
    fclose(truthfh);
#endif

    // get PNGs
    while (1) {
        direntry = readdir(dir);
        if (direntry == NULL) {
            break;
        }

        if (direntry->d_name[0] == '.') {
            continue;
        }

        files->push_back(std::string(direntry->d_name));
    }

    std::sort(files->begin(), files->end(), sortbyname);
    uint32_t processed = 0;

    // generate diffs
    for (uint64_t a = 0; a < files->size(); a++) {
        auto diffs = new std::vector<uint32_t>();
        diffs->reserve(8192);

        const char *n = files->at(a).c_str();
        char filename[512];
        strcpy(filename, argv[1]);
        strcat(filename, n);

        FILE* file = fopen(filename, "rb");
        if (file == NULL) {
            printf("failed to read %s\n", filename);
            die(strerror(errno));
        }
        
        wuffs_aux::DecodeImageCallbacks callbacks;
        wuffs_aux::sync_io::FileInput input(file);
        wuffs_aux::DecodeImageResult img = wuffs_aux::DecodeImage(callbacks, input);

        if (!img.error_message.empty()) {
            fprintf(stderr, "invalid image %s\n", n);
            printf("%s\n", img.error_message.c_str());
            fclose(file);
            continue;
        }

        if (!img.pixbuf.pixel_format().is_interleaved()) {
            die("not interleaved");
        }

        if (!img.pixbuf.pixcfg.is_valid()) {
            die("invalid image");
        }

        // full or diff image
        wuffs_base__table_u8 tab = img.pixbuf.plane(0);

        char num_canvas = n[14];
        int xoff = 0;
        int yoff = 0;

        switch (num_canvas) {
            case '0':
                break;
            case '1':
                xoff = 1000;
                break;
            case '2':
                yoff = 1000;
                break;
            case '3':
                xoff = 1000;
                yoff = 1000;
                break;
            default:
                die("invalid canvas num");
        }

        // process diff image - apply only opaque pixels
        for (uint32_t x = 0; x < 1000; x++) {
            for (uint32_t y = 0; y < 1000; y++) {
                int index = x + y * 1000;
                int cindex = (x + xoff) + (y + yoff) * 2000;
                uint32_t color = ((uint32_t*) tab.ptr)[index];

                if ((color & 0xFF000000) != 0 && truth[cindex] != color) {
                    uint32_t diffint = 0;
                    diffint |= (x + xoff) & 0x0FFF;
                    diffint |= ((y + yoff) & 0x0FFF) << 12;
                    diffint |= colors[color] << 24;

                    diffs->push_back(diffint);
                    truth[cindex] = color;
                }
            }
        }

        // write time
        char filetime[14];
        filetime[13] = '\0';
        memcpy(filetime, n, 13);
        uint64_t time = atoll(filetime);
        fwrite((void*)&time, sizeof(uint64_t), 1, writefh);

        // write size
        uint32_t s = diffs->size();
        fwrite((void*)&s, sizeof(uint32_t), 1, writefh);

        // write diffs
        size_t written = fwrite((void*)diffs->data(), sizeof(uint32_t), diffs->size(), writefh);
        if (written != diffs->size()) {
            die(strerror(errno));
        }

        fflush(writefh);
        printf("num: %u, diffs: %lu, done: %s\n", processed, diffs->size(), n);

        fclose(file);
        delete diffs;

        processed++;
        if (processed % 8192 == 0) {
            fwrite(truth, sizeof(uint32_t), 2000 * 2000, truthwrite);
            fflush(truthwrite);
            fseek(truthwrite, 0L, SEEK_SET);
            printf("saved progress\n");
        }
    }

    // write truth state
    fwrite(truth, sizeof(uint32_t), 2000 * 2000, truthwrite);
    fclose(truthwrite);

    // cleanup
    fclose(writefh);
    closedir(dir);
    printf("done\n");
    return 0;
}
