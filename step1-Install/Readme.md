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
##### Choice 1:
[Use the website Play with Docker](https://labs.play-with-docker.com/)
- Add a new instance

If the website is not working or saying that too many people are connected:

##### Choice 2 (Github student):
If you have github student on your github account you can use a gitpod instance for free once you register without putting in your credit card information:  
- [Gitpod Student Login](https://www.gitpod.io/github-student-developer-pack)
- Claim Offer Student for free
- Once logged in and in the page [https://gitpod.io/plans/](https://gitpod.io/plans/) upgrade to personnal for free
- Create a workspace [https://gitpod.io/workspaces](https://gitpod.io/workspaces) with the link of this github [https://github.com/GDSC-EPITA/gdsc-docker-epita](https://github.com/GDSC-EPITA/gdsc-docker-epita) and then open it in a browser

##### Choice 3 (VM):
- Follow this [tutorial](https://ubuntu.com/tutorials/how-to-run-ubuntu-desktop-on-a-virtual-machine-using-virtualbox#1-overview) to install a virtual machine running linux (ubuntu)
- Open a console and follow the linux instructions for debian based distributions.

##### Choice 4 (Windows Store - Ubuntu on WSL):
- Activate WSL for windows  [Youtube Link](https://www.youtube.com/watch?v=xel9kZ1J-Q8)
- Install docker [desktop](https://docs.docker.com/desktop/windows/wsl/) - [Youtube Link](https://www.youtube.com/watch?v=5RQbdMn04Oc)
