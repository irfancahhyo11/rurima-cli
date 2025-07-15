import os
import subprocess

print("installing rurima")

subprocess.run("curl -sL https://get.ruri.zip/rurima | bash > /dev/null")
while True:
	print("rurima-cli \n select \n 1. Docker \n 2. LXC \n 3. Pull \n 4. Exit \n select number")
	num = input("> ")
	if num == "1":
		print("Input docker image name:")
		docker_name = input("> ")
		print("Input docker image tag:")
		docker_tag = input("> ")
		print("Input docker image name (to save):")
		docker_path = input("> ")
		
		command = f"rurima docker pull -i {docker_name} -t {docker_tag} -s ./{docker_path}"
		
		with open(f"{docker_path}_log.sh", "w") as f:
			f.write("#!/bin/bash\n")
			f.write(f"# Pull command log\n")
			
			result = subprocess.run(command, shell=True, capture_output=True, text=True)
			
			f.write(f"echo 'Return code: {result.returncode}'\n")
			if result.stdout:
				f.write(f"echo 'STDOUT:'\ncat << 'EOF'\n{result.stdout}\nEOF\n")
			if result.stderr:
				f.write(f"echo 'STDERR:'\ncat << 'EOF'\n{result.stderr}\nEOF\n")

		print(f"Image pulled. to get start command check {docker_path}_log.sh")
		
	elif num == "2":
		print("1. List LXC containers 2. Pull LXC image")
		print("Input LXC option number:")
		lxc_option = input("> ")
		if lxc_option not in ["1", "2"]:
			print("Invalid option. Please select 1 or 2.")
			continue
		if lxc_option == "1":
			os.system("rurima lxc list")
			continue
		if lxc_option == "2":
			print("Input LXC image name:")
			lxc_image_name = input("> ")
			print("Input LXC image tag:")
			lxc_image_tag = input("> ")
			print("Input LXC image name (to save):")
			lxc_image_path = input("> ")
			
			command = f"rurima lxc pull -i {lxc_image_name} -t {lxc_image_tag} -s ./{lxc_image_path}"
			
			with open(f"{lxc_image_path}_log.sh", "w") as f:
				f.write("#!/bin/bash\n")
				f.write(f"# LXC pull command log\n")
				
				result = subprocess.run(command, shell=True, capture_output=True, text=True)
				
				f.write(f"echo 'Return code: {result.returncode}'\n")
				if result.stdout:
					f.write(f"echo 'STDOUT:'\ncat << 'EOF'\n{result.stdout}\nEOF\n")
				if result.stderr:
					f.write(f"echo 'STDERR:'\ncat << 'EOF'\n{result.stderr}\nEOF\n")
			
			print(f"Image pulled. to get start command check {lxc_image_path}_log.sh")
			continue
		
	elif num == "3":
		print("Pull option selected")
		print("Input what to pull:")
		pull_target = input("> ")
		command = f"rurima pull {pull_target}"
		
		with open("pull_log.sh", "w") as f:
			f.write("#!/bin/bash\n")
			f.write(f"# Pull command log\n")
			
			result = subprocess.run(command, shell=True, capture_output=True, text=True)
			
			f.write(f"echo 'Return code: {result.returncode}'\n")
			if result.stdout:
				f.write(f"echo 'STDOUT:'\ncat << 'EOF'\n{result.stdout}\nEOF\n")
			if result.stderr:
				f.write(f"echo 'STDERR:'\ncat << 'EOF'\n{result.stderr}\nEOF\n")
		
		print(f"Command executed. Log saved to pull_log.sh")
		print(f"Return code: {result.returncode}")
		
	elif num == "4":
		print("Exiting...")
		break
	else:
		print("Invalid option. Please select 1-4.")