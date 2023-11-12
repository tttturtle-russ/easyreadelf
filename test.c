/* 64位ELF文件读取section name string table */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main(int argc, char *argv[])
{
    /* 打开本地的ELF可执行文件hello */
    FILE *fp = fopen("./test", "rb");
    if(!fp) {
        perror("open ELF file");
        exit(1);
    }

    /* 1. 通过读取ELF header得到section header table的偏移 */
    /* for 64 bit ELF,
       e_ident(16) + e_type(2) + e_machine(2) +
       e_version(4) + e_entry(8) + e_phoff(8) = 40 */
    fseek(fp, 40, SEEK_SET);
    uint64_t sh_off;
    int r = fread(&sh_off, 1, 8, fp);
    if (r != 8) {
        perror("read section header offset");
        exit(2);
    }
    /* 得到的这个偏移值，可以用`reaelf -h hello`来验证是否正确 */
    printf("section header offset in file: %ld (0x%lx)\n", sh_off, sh_off);

    /* 2. 读取每个section header的大小e_shentsize,
       section header的数量e_shnum,
       以及对应section name字符串表的section header的索引e_shstrndx
       得到这些值后，都可以用`readelf -h hello`来验证是否正确 */
    /* e_flags(4) + e_ehsize(2) + e_phentsize(2) + e_phnum(2) = 10 */
    fseek(fp, 10, SEEK_CUR);
    uint16_t sh_ent_size;            /* 每个section header的大小 */
    r = fread(&sh_ent_size, 1, 2, fp);
    if (r != 2) {
        perror("read section header entry size");
        exit(2);
    }
    printf("section header entry size: %d\n", sh_ent_size);

    uint16_t sh_num;            /* section header的数量 */
    r = fread(&sh_num, 1, 2, fp);
    if (r != 2) {
        perror("read section header number");
        exit(2);
    }
    printf("section header number: %d\n", sh_num);

    uint16_t sh_strtab_index;   /* 节名字符串表对应的节头的索引 */
    r = fread(&sh_strtab_index, 1, 2, fp);
    if (r != 2) {
        perror("read section header string table index");
        exit(2);
    }
    printf("section header string table index: %d\n", sh_strtab_index);

    /* 3. read section name string table offset, size */
    /* 先找到节头字符串表对应的section header的偏移位置 */
    fseek(fp, sh_off + sh_strtab_index * sh_ent_size, SEEK_SET);
    /* 再从section header中找到节头字符串表的偏移 */
    /* sh_name(4) + sh_type(4) + sh_flags(8) + sh_addr(8) = 24 */
    fseek(fp, 24, SEEK_CUR);
    uint64_t str_table_off;
    r = fread(&str_table_off, 1, 8, fp);
    if (r != 8) {
        perror("read section name string table offset");
        exit(2);
    }
    printf("section name string table offset: %ld\n", str_table_off);

    /* 从section header中找到节头字符串表的大小 */
    uint64_t str_table_size;
    r = fread(&str_table_size, 1, 8, fp);
    if (r != 8) {
        perror("read section name string table size");
        exit(2);
    }
    printf("section name string table size: %ld\n", str_table_size);

    /* 动态分配内存，把节头字符串表读到内存中 */
    char *buf = (char *)malloc(str_table_size);
    if(!buf) {
        perror("allocate memory for section name string table");
        exit(3);
    }
    fseek(fp, str_table_off, SEEK_SET);
    r = fread(buf, 1, str_table_size, fp);
    if(r != str_table_size) {
        perror("read section name string table");
        free(buf);
        exit(2);
    }
    uint16_t i;
    for(i = 0; i < str_table_size; ++i) {
        /* 如果节头字符串表中的字节是0，就打印`\0` */
        if (buf[i] == 0)
            printf("\\0");
        else
            printf("%c", buf[i]);
    }
    printf("\n");
    free(buf);
    fclose(fp);
    return 0;
}

