#include <assert.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

char *map_file(const char *file_path, size_t *file_size) {
  int fd = open(file_path, O_RDONLY);
  if (fd == -1) {
    perror("open");
    return NULL;
  }

  struct stat sb;
  if (fstat(fd, &sb) == -1) {
    perror("fstat");
    close(fd);
    return NULL;
  }

  *file_size = sb.st_size;

  void *mapped_file = mmap(NULL, *file_size, PROT_READ, MAP_PRIVATE, fd, 0);
  if (mapped_file == MAP_FAILED) {
    perror("mmap");
    close(fd);
    return NULL;
  }

  close(fd);

  return mapped_file;
}

typedef struct space_t {
  uint32_t pos;
  uint16_t id;
  uint8_t size;
} space_t;

void parse(space_t **free_list, space_t **file_list, size_t *free_list_len,
           size_t *file_list_len, const char *input, size_t file_size) {
  space_t *frees = malloc(sizeof(space_t) * 10 * file_size);
  space_t *files = malloc(sizeof(space_t) * 10 * file_size);
  size_t frees_len = 0;
  size_t files_len = 0;

  uint32_t file_id = 0;

  uint32_t cum_size = 0;

  for (uint32_t file_pos = 0; file_pos < file_size; ++file_pos) {
    char in_c = input[file_pos];
    if (in_c < '0' || in_c > '9') {
      perror("invalid input char");
      exit(-1);
    }

    uint8_t num = in_c - '0';

    if (num == 0) {
      continue;
    }

    if (file_pos & 1) {
      // free
      frees[frees_len].pos = cum_size;
      frees[frees_len].size = num;
      frees_len++;
    } else {
      // file
      files[files_len].pos = cum_size;
      files[files_len].size = num;
      files[files_len].id = file_id++;
      files_len++;
    }

    cum_size += num;
  }

  *free_list = frees;
  *file_list = files;
  *free_list_len = frees_len;
  *file_list_len = files_len;
}

void print_space(space_t *spaces, size_t vec_len) {
  printf("len = %ld\n", vec_len);
  for (size_t k = 0; k < vec_len; ++k) {
    printf("pos = %d, size = %hhd, id = %hd\n", spaces[k].pos, spaces[k].size,
           spaces[k].id);
  }
}

uint64_t solve_a(space_t *frees, space_t *files, size_t frees_len,
                 size_t files_len, size_t file_size) {
  space_t *moved_files = malloc(sizeof(space_t) * 10 * file_size);
  size_t m_files_len = 0;

  const space_t *frees_end = frees + frees_len;

  space_t *free_ptr = frees;
  space_t *files_ptr = files + files_len - 1;
  while (files_ptr >= files && free_ptr < frees_end) {
    space_t file = *files_ptr;
    space_t free = *free_ptr;

    if (free.pos >= file.pos) {
      break;
    }

    moved_files[m_files_len].pos = free.pos;
    moved_files[m_files_len].id = file.id;

    if (free_ptr->size <= file.size) {
      moved_files[m_files_len].size = free.size;

      files_ptr->size -= free.size;

      free_ptr++;

      if (files_ptr->size == 0) {
        files_ptr--;
      }
    } else {
      moved_files[m_files_len].size = file.size;

      free_ptr->pos += file.size;
      free_ptr->size -= file.size;

      files_ptr--;
    }

    m_files_len++;
  }

  space_t *last_unmoved_file = files_ptr;

  uint64_t sum = 0;

  for (space_t *files_ptr = files; files_ptr <= last_unmoved_file;
       files_ptr++) {
    space_t file = *files_ptr;
    uint64_t pos_sum = (2 * (uint64_t)file.pos + (uint64_t)file.size - 1) *
                       (uint64_t)file.size / 2;
    sum += (uint64_t)file.id * pos_sum;
  }

  for (space_t *m_files_ptr = moved_files;
       m_files_ptr < moved_files + m_files_len; m_files_ptr++) {
    space_t file = *m_files_ptr;
    uint64_t pos_sum = (2 * (uint64_t)file.pos + (uint64_t)file.size - 1) *
                       (uint64_t)file.size / 2;
    sum += (uint64_t)file.id * pos_sum;
  }

  free(frees);
  free(files);
  free(moved_files);

  return sum;
}

uint64_t solve_b(space_t *frees, space_t *files, size_t frees_len,
                 size_t files_len, size_t file_size) {
  const space_t *frees_end = frees + frees_len;

  for (space_t *files_ptr = files + files_len - 1; files_ptr >= files;
       files_ptr--) {
    for (space_t *free_ptr = frees; free_ptr < frees_end; ++free_ptr) {
      if (free_ptr->pos >= files_ptr->pos)
        break;
      if (free_ptr->size >= files_ptr->size) {
        files_ptr->pos = free_ptr->pos;
        free_ptr->pos += files_ptr->size;
        free_ptr->size -= files_ptr->size;
        break;
      }
    }
  }

  uint64_t sum = 0;

  for (space_t *files_ptr = files; files_ptr < files + files_len; files_ptr++) {
    space_t file = *files_ptr;
    uint64_t pos_sum = (2 * (uint64_t)file.pos + (uint64_t)file.size - 1) *
                       (uint64_t)file.size / 2;
    sum += (uint64_t)file.id * pos_sum;
  }

  free(frees);
  free(files);

  return sum;
}

int main() {
  size_t file_size = 0;
  //   char *input = map_file("../example.txt", &file_size);
  struct timespec start_input, end_input;
  clock_gettime(CLOCK_MONOTONIC, &start_input);
  char *input = map_file("../input.txt", &file_size);
  clock_gettime(CLOCK_MONOTONIC, &end_input);

  space_t *frees;
  space_t *files;
  size_t frees_len = 0;
  size_t files_len = 0;
  struct timespec start_parsing, end_parsing;
  clock_gettime(CLOCK_MONOTONIC, &start_parsing);
  parse(&frees, &files, &frees_len, &files_len, input, file_size);

  space_t *frees_clone = malloc(sizeof(space_t) * frees_len);
  memcpy(frees_clone, frees, sizeof(space_t) * frees_len);
  space_t *files_clone = malloc(sizeof(space_t) * files_len);
  memcpy(files_clone, files, sizeof(space_t) * files_len);

  clock_gettime(CLOCK_MONOTONIC, &end_parsing);

  struct timespec start_a, end_a;
  clock_gettime(CLOCK_MONOTONIC, &start_a);
  uint64_t output_a = solve_a(frees, files, frees_len, files_len, file_size);
  clock_gettime(CLOCK_MONOTONIC, &end_a);

  struct timespec start_b, end_b;
  clock_gettime(CLOCK_MONOTONIC, &start_b);
  uint64_t output_b =
      solve_b(frees_clone, files_clone, frees_len, files_len, file_size);
  clock_gettime(CLOCK_MONOTONIC, &end_b);

  printf("Task1: %ld\n", output_a);
  printf("Task2: %ld\n", output_b);

  printf("Input loading took: : %lus, %luµs\n",
         end_input.tv_sec - start_input.tv_sec,
         (end_input.tv_nsec - start_input.tv_nsec) / 1000);
  printf("Input parsing took: : %lus, %luµs\n",
         end_parsing.tv_sec - start_parsing.tv_sec,
         (end_parsing.tv_nsec - start_parsing.tv_nsec) / 1000);
  printf("Task 1 took: %lus, %luµs\n", end_a.tv_sec - start_a.tv_sec,
         (end_a.tv_nsec - start_a.tv_nsec) / 1000);
  printf("Task 2 took: %lus, %luµs\n", end_b.tv_sec - start_b.tv_sec,
         (end_b.tv_nsec - start_b.tv_nsec) / 1000);

  if (input == NULL) {
    return EXIT_FAILURE;
  }

  if (munmap(input, file_size) == -1) {
    perror("munmap");
    return EXIT_FAILURE;
  }
}