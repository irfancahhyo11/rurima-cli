#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>
#include <sys/stat.h>

#define MAX_INPUT 256
#define MAX_COMMAND 512
#define MAX_LINE 1024

// Функция для выполнения команды с записью в лог и вывода последних 3 строк
int execute_command_with_log(const char* command, const char* log_file) {
    char full_command[MAX_COMMAND * 2];
    char line[MAX_LINE];
    char last_lines[3][MAX_LINE];
    int line_count = 0;
    int current_line = 0;
    
    // Открываем файл для записи
    FILE* log_fp = fopen(log_file, "w");
    if (!log_fp) {
        printf("Error: Cannot create log file %s\n", log_file);
        return -1;
    }
    
    // Создаем команду с перенаправлением через tee для одновременного вывода и записи
    snprintf(full_command, sizeof(full_command), "%s 2>&1 | tee %s", command, log_file);
    
    // Выполняем команду
    FILE* fp = popen(full_command, "r");
    if (!fp) {
        printf("Error: Cannot execute command\n");
        fclose(log_fp);
        return -1;
    }
    
    // Читаем вывод построчно (уже выводится через tee)
    while (fgets(line, sizeof(line), fp)) {
        // Сохраняем последние 3 строки
        strcpy(last_lines[current_line % 3], line);
        current_line++;
        if (line_count < 3) line_count++;
    }
    
    int result = pclose(fp);
    fclose(log_fp);
    
    // Добавляем последние 3 строки в конец лог файла
    log_fp = fopen(log_file, "a");
    if (log_fp) {
        fprintf(log_fp, "\n# Last 3 lines:\n");
        int start = (current_line >= 3) ? (current_line % 3) : 0;
        for (int i = 0; i < line_count; i++) {
            int idx = (start + i) % 3;
            fprintf(log_fp, "# %s", last_lines[idx]);
        }
        fclose(log_fp);
    }
    
    // Делаем файл исполняемым
    chmod(log_file, 0755);
    
    return result;
}

// Функция для получения ввода пользователя
void get_input(const char* prompt, char* buffer, int max_size) {
    printf("%s", prompt);
    fflush(stdout);
    if (fgets(buffer, max_size, stdin)) {
        // Удаляем символ новой строки
        size_t len = strlen(buffer);
        if (len > 0 && buffer[len-1] == '\n') {
            buffer[len-1] = '\0';
        }
    }
}

int main() {
    char num[10];
    char docker_name[MAX_INPUT];
    char docker_tag[MAX_INPUT]; 
    char docker_path[MAX_INPUT];
    char lxc_option[10];
    char lxc_image_name[MAX_INPUT];
    char lxc_image_tag[MAX_INPUT];
    char lxc_image_path[MAX_INPUT];
    char pull_target[MAX_INPUT];
    char command[MAX_COMMAND];
    char log_file[MAX_INPUT + 20];
    
    printf("installing rurima\n");
    
    // Установка rurima
    system("curl -sL https://get.ruri.zip/rurima | bash > /dev/null");
    
    sleep(1);
    printf("\n\n");
    
    while (1) {
        printf("rurima-cli \n select \n 1. Docker \n 2. LXC \n 3. Pull \n 4. Exit \n select number\n");
        get_input("> ", num, sizeof(num));
        
        if (strcmp(num, "1") == 0) {
            get_input("Input docker image name:\n> ", docker_name, sizeof(docker_name));
            get_input("Input docker image tag:\n> ", docker_tag, sizeof(docker_tag));
            get_input("Input docker image name (to save):\n> ", docker_path, sizeof(docker_path));
            
            printf("Pulling image!\n");
            
            snprintf(command, sizeof(command), "rurima docker pull -i %s -t %s -s ./%s", 
                    docker_name, docker_tag, docker_path);
            snprintf(log_file, sizeof(log_file), "%s_log.sh", docker_path);
            
            execute_command_with_log(command, log_file);
            printf("Image pulled. to get start command check %s\n", log_file);
            
        } else if (strcmp(num, "2") == 0) {
            printf("1. List LXC containers 2. Pull LXC image\n");
            get_input("Input LXC option number:\n> ", lxc_option, sizeof(lxc_option));
            
            if (strcmp(lxc_option, "1") != 0 && strcmp(lxc_option, "2") != 0) {
                printf("Invalid option. Please select 1 or 2.\n");
                continue;
            }
            
            if (strcmp(lxc_option, "1") == 0) {
                system("rurima lxc list");
                continue;
            }
            
            if (strcmp(lxc_option, "2") == 0) {
                get_input("Input LXC image name:\n> ", lxc_image_name, sizeof(lxc_image_name));
                get_input("Input LXC image tag:\n> ", lxc_image_tag, sizeof(lxc_image_tag));
                get_input("Input LXC image name (to save):\n> ", lxc_image_path, sizeof(lxc_image_path));
                
                snprintf(command, sizeof(command), "rurima lxc pull -i %s -t %s -s ./%s", 
                        lxc_image_name, lxc_image_tag, lxc_image_path);
                snprintf(log_file, sizeof(log_file), "%s_log.sh", lxc_image_path);
                
                execute_command_with_log(command, log_file);
                printf("Image pulled. to get start command check %s\n", log_file);
                continue;
            }
            
        } else if (strcmp(num, "3") == 0) {
            printf("Pull option selected\n");
            get_input("Input what to pull:\n> ", pull_target, sizeof(pull_target));
            
            snprintf(command, sizeof(command), "rurima pull %s", pull_target);
            snprintf(log_file, sizeof(log_file), "%s_log.sh", pull_target);
            
            execute_command_with_log(command, log_file);
            printf("Command executed. Log saved to %s\n", log_file);
            
        } else if (strcmp(num, "4") == 0) {
            printf("Exiting...\n");
            break;
        } else {
            printf("Invalid option. Please select 1-4.\n");
        }
    }
    
    return 0;
}
