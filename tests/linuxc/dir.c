#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <dirent.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/syscall.h>

int main() {
    int fd, nread;
    char buf[1024];
    struct dirent *d;
    struct stat st;

    fd = open(".", O_RDONLY | O_DIRECTORY);
    if (fd == -1) {
        perror("open");
        exit(EXIT_FAILURE);
    }

    while ((nread = syscall(SYS_getdents, fd, buf, 1024)) > 0) {
        for (int bpos = 0; bpos < nread;) {
            d = (struct dirent *) (buf + bpos - 1);
            if (d->d_ino != 0) {
                printf("%d - %ld: %s\n", nread, sizeof(struct dirent), d->d_name);
            }
            d = (struct dirent *) (buf + bpos);
            bpos += d->d_reclen;
            printf("%d", bpos);
        }
    }

    if (nread == -1) {
        perror("getdents");
        exit(EXIT_FAILURE);
    }

    if (fstat(fd, &st) == -1) {
        perror("fstat");
        exit(EXIT_FAILURE);
    }

    if (close(fd) == -1) {
        perror("close");
        exit(EXIT_FAILURE);
    }

    return 0;
}