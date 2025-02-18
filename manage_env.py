import os
import subprocess
import sys
import logging
from abc import ABC, abstractmethod

"""
This module encapsulates the management logic of the python virtual environment
necessary for the proper functioning of the project wAIves.
"""

def setup_logger(log_file):
    """Configure the logger to write logs to a file."""
    logging.basicConfig(
        filename=log_file,
        level=logging.INFO,
        format='%(asctime)s - %(levelname)s - %(message)s',
    )

class OS :

    def __init__(self):
        pass

    @abstractmethod
    def exist_env(self, name_virtual_env:str):
        pass

    def create_env(self, name_virtual_env:str):
        """Create a new virtual environment in the current directory."""
        try:
            # Create a new virtual environment using the venv module
            subprocess.run([sys.executable, '-m', 'pip', 'install', 'virtualenv'], check=True)
            subprocess.run([sys.executable, '-m', 'virtualenv', name_virtual_env], check=True)
            print(f"Virtual environment '{name_virtual_env}' successfully created.")
        except subprocess.CalledProcessError as e:
            print(f"ERROR: Virtual environment '{name_virtual_env}' can't be created.")
            print(f"Details: {e}")
        except Exception as e:
            print(f"An unexpected error occurred: {e}")

    @abstractmethod
    def run_in_env(self, name_virtual_env:str, command:str):
        """
        name_virtual_env (str): The name or path of the virtual environment.
        command (str): The command to execute in the virtual environment.
        """
        pass

    @abstractmethod
    def install_dependencies(self, name_virtual_env:str):
        """Installing the necessary python dependencies."""
        pass

class Linux(OS) :

    def __init__(self):
        super().__init__()

    def exist_env(self, name_virtual_env:str):
        """Verify if the virtual environment ```name_virtual_env``` exist in the current directory."""
        env_path = os.path.join(os.getcwd(), name_virtual_env, 'bin', 'activate')
        return os.path.exists(env_path)

    def run_in_env(self, name_virtual_env:str, command:str):
        """
        name_virtual_env (str): The name or path of the virtual environment.
        command (str): The command to execute in the virtual environment.
        """
        activate_script = os.path.join(name_virtual_env, 'bin', 'activate')
        if not os.path.exists(activate_script):
            return f"Error: Virtual environment '{name_virtual_env}' not found."

        try:
            full_command = f"source {activate_script} && {command}"
            result = subprocess.run(full_command, shell=True, check=True, text=True, capture_output=True, executable='/bin/bash')
            return result.stdout
        except subprocess.CalledProcessError as e:
            return f"Error: {e.stderr}"

    def install_dependencies(self, name_virtual_env:str):
        """Installing the necessary python dependencies."""
        dependencies = [
            ("neo4j==5.28.1"), ("psycopg2-binary")
        ]
        # path of the Python interpreter of the virtual environment
        python_executable = os.path.join(name_virtual_env, 'bin', 'python')

        # Create a command to automate the installations
        command = ""
        for module in dependencies:
            command += f" {python_executable} -m pip install {module} &&"
        command = command[:-2]
        print(f"Command executed : {command}\n")

        # Running installation commands in the virtual environment
        state = self.run_in_env(name_virtual_env, command)
        print(f"State of installation of dependencies :\n\n{state}")

class Windows(OS) :

    def __init__(self):
        super().__init__()

    def exist_env(self, name_virtual_env:str):
        """Verify if the virtual environment ```name_virtual_env``` exist in the current directory."""
        env_path = os.path.join(os.getcwd(), name_virtual_env, 'Scripts', 'activate')
        return os.path.exists(env_path)

    def run_in_env(self, name_virtual_env:str, command:str):
        """
        name_virtual_env (str): The name of the virtual environment.
        command (str): The command to execute in the virtual environment.
        !!! When you need to use the python of the virtual environnement, use a powershell commande like :
        pythonENV -c "print('Hello world!')"
        """
        log_file = "server_log.txt"
        setup_logger(log_file)

        python_executable = os.getcwd() + f"\\{name_virtual_env}\\Scripts\\python.exe"
        command = command.replace("pythonENV", python_executable).strip()
        full_command = f"{command}"

        try:
            print(f"full_command :\n{full_command}\n\n")
            result = subprocess.run(full_command, shell=True, check=True, text=True, capture_output=True)
            logging.info(f"Command executed successfully:\n{result.stdout}")
            return result.stdout
        except subprocess.CalledProcessError as e:
            logging.error(f"Error occurred while executing command:\n{e.stderr}")
            return f"Error: {e.stderr}\n\nfull_command :\n{full_command}\n\n"

    def install_dependencies(self, name_virtual_env:str):
        """Installing the necessary python dependencies."""
        dependencies = [
            ("neo4j==5.28.1"), ("psycopg2-binary")
        ]
        # path of the Python interpreter of the virtual environment
        python_executable = os.getcwd() + f"\\{name_virtual_env}\\Scripts\\python.exe"

        # Create commands to automate the installations
        print("\n\n-- Install dependencies --\nThis operation could take few minutes. Don't panic and don't shutdown your computer.\n")
        for module in dependencies :
            try:
                command = f"{python_executable} -m pip install {module}"
                result = subprocess.run(command, shell=True, check=True, text=True, capture_output=True)
                print(f"Successfully install : {module[0]}")
            except subprocess.CalledProcessError as e:
                print(f"Error: {e.stderr}\n\nfull_command :\n{command}\n\n")

if __name__ == '__main__' :
    ENV_NAME = "db"

    if os.name == 'posix' :
        current_os = Linux()
    else :
        current_os = Windows()
    
    if current_os.exist_env(ENV_NAME) == False :
        current_os.create_env(ENV_NAME)
        current_os.install_dependencies(ENV_NAME)