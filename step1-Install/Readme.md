### Installation [~5 min]

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


