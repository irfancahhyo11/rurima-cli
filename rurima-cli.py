import os
import subprocess

print("installing rurima")

os.system("curl -sL https://get.ruri.zip/rurima | bash > /dev/null")
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
		
		command = f"rurima docker pull -i {docker_name} -t {docker_tag} -s ./{docker_path} > {docker_path}_log.sh"

		os.system(command)
		
		

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
			
			command = f"rurima lxc pull -i {lxc_image_name} -t {lxc_image_tag} -s ./{lxc_image_path} > {lxc_image_path}_log.sh"
			
			os.system(command)
			
			print(f"Image pulled. to get start command check {lxc_image_path}_log.sh")
			continue
		
	elif num == "3":
		print("Pull option selected")
		print("Input what to pull:")
		pull_target = input("> ")
		command = f"rurima pull {pull_target} > {pull_target}_log.sh"
		
		os.system(command)
		
		print(f"Command executed. Log saved to pull_log.sh")
		print(f"Return code: {result.returncode}")
		
	elif num == "4":
		print("Exiting...")
		break
	else:
		print("Invalid option. Please select 1-4.")
