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
#include <string.h>
#include <dirent.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include <algorithm>
#include <cstring>
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

typedef struct {
    uint32_t c;
    uint16_t x;
    uint16_t y;
} ImgDiff;

const char *src = "/mnt/hdd/place-ext/\0";
const char *dst = "/mnt/hdd/place-final/\0";

bool sortbyname(std::string a, std::string b) {
    char filetime[14];
    filetime[13] = '\0';

    memcpy(filetime, a.c_str(), 13);
    uint64_t at = atoll(filetime);

    memcpy(filetime, b.c_str(), 13);
    uint64_t bt = atoll(filetime);

    return at < bt;
}

int main() {
    DIR* dir;
    struct dirent *direntry;
    dir = opendir(src);
    if (dir == NULL) {
        die("fail open src");
    }

    auto files = new std::vector<std::string>();

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

    // generate diffs
    for (uint64_t a = 0; a < files->size(); a++) {
        auto diffs = new std::vector<ImgDiff>();
        diffs->reserve(8192);

        const char *n = files->at(a).c_str();
        char filename[512];
        strcpy(filename, src);
        strcat(filename, n);

        FILE* file = fopen(filename, "rb");
        if (file == NULL) {
            die(strerror(errno));
        }
        
        wuffs_aux::DecodeImageCallbacks callbacks;
        wuffs_aux::sync_io::FileInput input(file);
        wuffs_aux::DecodeImageResult img = wuffs_aux::DecodeImage(callbacks, input);

        if (!img.error_message.empty()) {
            die(img.error_message);
        }

        if (!img.pixbuf.pixel_format().is_interleaved()) {
            die("not interleaved");
        }

        if (!img.pixbuf.pixcfg.is_valid()) {
            die("invalid image");
        }

        // full or diff image
        bool full = strstr(filename, "-f-");
        wuffs_base__table_u8 tab = img.pixbuf.plane(0);

        /*
            // process full image - check every single pixel
            for (uint32_t x = 0; x < 1000; x++) {
                for (uint32_t y = 0; y < 1000; y++) {
                    int index = x + y * 1000;
                    int cindex = (x + xoff) + (y + yoff) * 1000;
                    uint32_t color = ((uint32_t*) tab.ptr)[index];
                    
                    if ((color & 0xFF000000) != 0 && refimg[cindex] != color) {
                        ImgDiff d;
                        d.x = 0;
                        d.y = 0;
                        d.time = 0;
                        d.c = 0;

                        diffs->push_back(d);
                        refimg[cindex] = color;
                    }
                }
            }
        */


        if (!full) {
            // process diff image - apply only opaque pixels
            for (uint32_t x = 0; x < 1000; x++) {
                for (uint32_t y = 0; y < 1000; y++) {
                    int index = x + y * 1000;
                    uint32_t color = ((uint32_t*) tab.ptr)[index];

                    if ((color & 0xFF000000) != 0) {
                        ImgDiff d;
                        d.x = x;
                        d.y = y;
                        d.c = color;

                        diffs->push_back(d);
                    }
                }
            }

            // write pixels in diff image
            strcpy(filename, dst);
            char filename2[32];
            memcpy(filename2, n, 26);
            filename2[26] = '\0';
            strcat(filename, filename2);

            FILE* writefh = fopen(filename, "w");
            if (writefh == NULL) {
                die(strerror(errno));
            }
            
            size_t written = fwrite((void*)diffs->data(), sizeof(ImgDiff), diffs->size(), writefh);
            if (written != diffs->size()) {
                die(strerror(errno));
            }

            fflush(writefh);
            fclose(writefh);

            printf("diffs: %lu, done: %s\n", diffs->size(), n);
        }

        fclose(file);
        delete diffs;
    }

    // cleanup
    closedir(dir);
    printf("done\n");
    return 0;
}
