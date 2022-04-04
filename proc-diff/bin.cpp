#include <algorithm>
#include <cstring>
#include <string>
#include <vector>

#include <errno.h>
#include <dirent.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

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

const char *src = "/mnt/hdd/place-final/";

int main() {
    DIR* dir;
    struct dirent *direntry;
    dir = opendir(src);
    if (dir == NULL) {
        die("fail open src");
    }

    auto files = new std::vector<std::string>();

    // get files
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

    // begin write
    FILE *writefh = fopen("/mnt/hdd/place-diffs.bin", "w");
    if (writefh == NULL) {
        die(strerror(errno));
    }
    
    for (uint64_t i = 0; i < files->size(); i++) {
        auto filename = files->at(i);

        char filetime[14];
        filetime[13] = '\0';
        memcpy(filetime, filename.c_str(), 13);
        uint64_t time = atoll(filetime);
        fwrite((void*)&time, sizeof(uint64_t), 1, writefh);

        char f[512];
        strcpy(f, src);
        strcat(f, filename.c_str());

        FILE *readfh = fopen(f, "r");
        if (readfh == NULL) {
            die(strerror(errno));
        }
        fseek(readfh, 0L, SEEK_END);
        uint32_t size = ftell(readfh);
        fseek(readfh, 0L, SEEK_SET);
        fwrite((void*)&size, sizeof(uint32_t), 1, writefh);

        int c;
        int ia = 0;
        do {
            c = getc(readfh);
            if (c != EOF) {
                putc(c, writefh);
                ia++;
            }
        } while (c != EOF);

        fflush(writefh);
        fclose(readfh);
    }

    fclose(writefh);
}
