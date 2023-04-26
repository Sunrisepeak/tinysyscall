#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <stdio.h>

struct stat file_info;

int main() {
    if (stat("sal-test.data", &file_info) == -1) {
        perror("stat");
        return 1;
    }
    printf("%ld\n", sizeof(file_info));
    printf("File size: %ld bytes\n", file_info.st_size);
    printf("Number of links: %ld\n", file_info.st_nlink);
    printf("File inode: %ld\n", file_info.st_ino);
    printf("File permissions: %o\n", file_info.st_mode);
    printf("File owner: %d\n", file_info.st_uid);
    printf("File group: %d\n", file_info.st_gid);
    printf("Last access time: %ld\n", file_info.st_atime);
    printf("Last modification time: %ld\n", file_info.st_mtime);
    printf("Last status change time: %ld\n", file_info.st_ctime);
    return 0;
}