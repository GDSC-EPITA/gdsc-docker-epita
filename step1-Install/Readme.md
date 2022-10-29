### Installation [~5 min]

#### Linux (Recommended)
- get `Docker`:
    - Debian based:
        ```
        sudo apt install -y docker docker.io
        sudo groupadd docker
        sudo usermod -aG docker $USER
        newgrp docker
        reboot
        ```
    - CentOS based:
        ```
        yum install -y yum-utils
        yum-config-manager \
            --add-repo \
            https://download.docker.com/linux/centos/docker-ce.repo
        yum install --allowerasing -y docker-ce docker-ce-cli containerd.io
        sudo groupadd docker
        sudo usermod -aG docker $USER
        systemctl enable docker
        systemctl start docker
        newgrp docker
        reboot
        ```
    - Arch Linux based:
        ```
        pacman -Syu docker docker-compose
        sudo groupadd docker
        sudo usermod -aG docker $USER
        systemctl enable docker
        systemctl start docker
        newgrp docker
        reboot
        ```
- Test your installation:
    - `docker run hello-world` (no `sudo`, It's annoying but secured)
- If anything fails during the installation, do not panic and call an Assistant

#### Mac / Windows
[Use the website Play with Docker](https://labs.play-with-docker.com/)
- Add a new instance

If the website is not working or saying that too many people are connected:
- Follow this [tutorial](https://ubuntu.com/tutorials/how-to-run-ubuntu-desktop-on-a-virtual-machine-using-virtualbox#1-overview) to install a virtual machine running a linux (ubuntu)
- Open a console and follow the linux instructions for debian based distributions.
