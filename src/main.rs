use std::io::{self, Write, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn execute_command_with_log(command: &str, log_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Создаем файл для записи лога
    let log = Arc::new(Mutex::new(File::create(log_file)?));
    let last_lines = Arc::new(Mutex::new(Vec::<String>::new()));
    
    // Запускаем команду
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    
    let log_clone = Arc::clone(&log);
    let last_lines_clone = Arc::clone(&last_lines);
    
    // Обрабатываем stdout в отдельном потоке
    let stdout_handle = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
                
                // Записываем в лог
                if let Ok(mut log_file) = log_clone.lock() {
                    let _ = writeln!(log_file, "{}", line);
                }
                
                // Сохраняем последние строки
                if let Ok(mut lines) = last_lines_clone.lock() {
                    lines.push(line);
                    if lines.len() > 3 {
                        lines.remove(0);
                    }
                }
            }
        }
    });
    
    let log_clone2 = Arc::clone(&log);
    let last_lines_clone2 = Arc::clone(&last_lines);
    
    // Обрабатываем stderr в отдельном потоке
    let stderr_handle = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                eprintln!("STDERR: {}", line);
                
                // Записываем в лог
                if let Ok(mut log_file) = log_clone2.lock() {
                    let _ = writeln!(log_file, "STDERR: {}", line);
                }
                
                // Сохраняем последние строки
                if let Ok(mut lines) = last_lines_clone2.lock() {
                    lines.push(format!("STDERR: {}", line));
                    if lines.len() > 3 {
                        lines.remove(0);
                    }
                }
            }
        }
    });
    
    // Ждем завершения всех потоков
    let _ = stdout_handle.join();
    let _ = stderr_handle.join();
    
    // Ждем завершения процесса
    child.wait()?;
    
    // Добавляем последние 3 строки в конец файла
    if let Ok(mut log_file) = log.lock() {
        let _ = writeln!(log_file, "\n# Command:");
        if let Ok(lines) = last_lines.lock() {
            for line in lines.iter() {
                let _ = writeln!(log_file, "# {}", line);
            }
        }
    }
    
    // Делаем файл исполняемым (chmod +x)
    let metadata = std::fs::metadata(log_file)?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(log_file, permissions)?;
    
    Ok(())
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("installing rurima");
    
    // Установка rurima
    Command::new("sh")
        .arg("-c")
        .arg("curl -sL https://get.ruri.zip/rurima | bash > /dev/null")
        .status()?;
    
    thread::sleep(Duration::from_secs(1));
    println!("\n\n");
    
    loop {
        println!("rurima-cli \n select \n 1. Docker \n 2. LXC \n 3. Pull \n 4. Exit \n select number");
        let num = get_input("> ");
        
        match num.as_str() {
            "1" => {
                let docker_name = get_input("Input docker image name:\n> ");
                let docker_tag = get_input("Input docker image tag:\n> ");
                let docker_path = get_input("Input docker image name (to save):\n> ");
                
                println!("Pulling image!");
                
                let command = format!("rurima docker pull -i {} -t {} -s ./{}", 
                                    docker_name, docker_tag, docker_path);
                let log_file = format!("{}_log.sh", docker_path);
                
                if let Err(e) = execute_command_with_log(&command, &log_file) {
                    eprintln!("Error executing command: {}", e);
                } else {
                    println!("Image pulled. to get start command check {}", log_file);
                }
            },
            
            "2" => {
                println!("1. List LXC containers 2. Pull LXC image");
                let lxc_option = get_input("Input LXC option number:\n> ");
                
                match lxc_option.as_str() {
                    "1" => {
                        Command::new("sh")
                            .arg("-c")
                            .arg("rurima lxc list")
                            .status()?;
                        continue;
                    },
                    "2" => {
                        let lxc_image_name = get_input("Input LXC image name:\n> ");
                        let lxc_image_tag = get_input("Input LXC image tag:\n> ");
                        let lxc_image_path = get_input("Input LXC image name (to save):\n> ");
                        
                        let command = format!("rurima lxc pull -i {} -t {} -s ./{}", 
                                            lxc_image_name, lxc_image_tag, lxc_image_path);
                        let log_file = format!("{}_log.sh", lxc_image_path);
                        
                        if let Err(e) = execute_command_with_log(&command, &log_file) {
                            eprintln!("Error executing command: {}", e);
                        } else {
                            println!("Image pulled. to get start command check {}", log_file);
                        }
                    },
                    _ => {
                        println!("Invalid option. Please select 1 or 2.");
                        continue;
                    }
                }
            },
            
            "3" => {
                println!("Pull option selected");
                let pull_target = get_input("Input what to pull:\n> ");
                
                let command = format!("rurima pull {}", pull_target);
                let log_file = format!("{}_log.sh", pull_target);
                
                if let Err(e) = execute_command_with_log(&command, &log_file) {
                    eprintln!("Error executing command: {}", e);
                } else {
                    println!("Command executed. Log saved to {}", log_file);
                }
            },
            
            "4" => {
                println!("Exiting...");
                break;
            },
            
            _ => {
                println!("Invalid option. Please select 1-4.");
            }
        }
    }
    
    Ok(())
}
