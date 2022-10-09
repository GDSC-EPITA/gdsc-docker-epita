---
title: TP Docker
tags: TP Assistants
---

### I. Docker
![](https://www.docker.com/sites/default/files/d8/2019-07/vertical-logo-monochromatic.png)

<!-->5 min lecture + 5 min présentation + questions<-->

#### What is Docker ?
Docker is an open platform for developing, shipping, and running applications. Docker enables you to separate your applications from your infrastructure so you can deliver software quickly. With Docker, you can manage your infrastructure in the same ways you manage your applications. By taking advantage of Docker’s methodologies for shipping, testing, and deploying code quickly, you can significantly reduce the delay between writing code and running it in production.
![](https://docs.docker.com/engine/images/architecture.svg)
When you use Docker, you are creating and using images, containers, networks, volumes, plugins, and other objects.
##### Images
An image is a read-only template with instructions for creating a Docker container. Often, an image is based on another image, with some additional customization. For example, you may build an image which is based on the ubuntu image, but installs the Apache web server and your application, as well as the configuration details needed to make your application run.

##### Containers
A container is a runnable instance of an image. You can create, start, stop, move, or delete a container using the Docker API or CLI. You can connect a container to one or more networks, attach storage to it, or even create a new image based on its current state.

A container is defined by its image as well as any configuration options you provide to it when you create or start it. When a container is removed, any changes to its state that are not stored in persistent storage disappear.

#### Why using Containerization ?
![](https://images.contentstack.io/v3/assets/blt300387d93dabf50e/bltb6200bc085503718/5e1f209a63d1b6503160c6d5/containers-vs-virtual-machines.jpg)

##### Virtual Machines
Virtual machines (VMs) are an abstraction of physical hardware turning one server into many servers. The hypervisor allows multiple VMs to run on a single machine. Each VM includes a full copy of an operating system, the application, necessary binaries and libraries - taking up tens of GBs. VMs can also be slow to boot.

##### Containers
Containers are an abstraction at the app layer that packages code and dependencies together. Multiple containers can run on the same machine and share the OS kernel with other containers, each running as isolated processes in user space. Containers take up less space than VMs (container images are typically tens of MBs in size), can handle more applications and require fewer VMs and Operating systems.

##### A simple example
- Developers write code locally and share their work with their colleagues using Docker containers.
- They use Docker to push their applications into a test environment and execute automated and manual tests.
- When developers find bugs, they can fix them in the development environment and redeploy them to the test environment for testing and validation.
- When testing is complete, getting the fix to the customer is as simple as pushing the updated image to the production environment. When an image is recreated, only the modified layers are updated.

<!-->5 min installation<-->
### II. Requirements:
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

### III. TP

Now that you begin to understand how Docker works let's try to play with some containers.

<!-->2 min<-->
#### Run your first Containers

First, type the following command:

```
docker run --name name_of_your_container -d nginx
```

This command should start a new Nginx container.
Docker starts by pulling a Nginx image from Docker's official registries and then run this image to create a container.
To start another Nginx container, Docker will use the local image previously downloaded.
The --name option allows to name your container.

`run`:

- Allows to launch containers from images
- It has multiple options:
    - `-p` allows to bind your container's ports to your PC 

    - `-d`  allows to launch your container in detached mode (similar to a background mode)

    - `-it` allows to launch the container in interactive mode (stdin, stdout, stderr linked to your terminal)

    - `-e` allows to define environment variables

    - `-v` mount a volume in the container (we will develop this notion later)

    - `--name` give a name to the container

    - `--rm` when a containers exits this option clean the container and its associated data and virtual volumes

If you want more informations about the `run` command and its associated options in the [documentation](https://docs.docker.com/engine/reference/run/).
The documentation is quite complete so don't hesitate to have a look on it.

<!-->5 min<-->
#### Monitor your Images and Containers

- In order to insure that your container is running, you can type the following command:

```
docker ps
```

This command should display your running containers.
If you also want to show the stopped containers you can add the option: `-a` or `--all`.

The `ps` command has more options which can be checked in the [documentation](https://docs.docker.com/engine/reference/commandline/ps/)

- To display all images contained in your local registry type the following command:

```
docker image ls
```

You can see the Ubuntu image downloaded before is listed by the command.
You can check the [documentation](https://docs.docker.com/engine/reference/commandline/image_ls/) for the `docker image ls` command. 

You can also use:

```
docker images
```

It is just an alias for the `docker image ls` command.

- To display the resource usage of your container use the following command:

```
docker stats [name_of_your_container...]
```

You may close the interaction by pressing `CTRL+C`.

You can check the [documentation](https://docs.docker.com/engine/reference/commandline/stats/) for the `docker stats` command.

If you run this command with your Nginx container, you should get real time information about the ressourses used by your container. It should look like this:

```
CONTAINER ID   NAME                     CPU %     MEM USAGE / LIMIT     MEM %     NET I/O       BLOCK I/O     PIDS
7b43a902c841   name_of_your_container   0.00%     1.961MiB / 15.29GiB   0.01%     9.24kB / 0B   0B / 8.19kB   2
```

- To display low-level informations about your container such as the IP address you can use the following command:

```
docker inspect [options] name_of_your_container [name_of_your_container...]
```

You can check the [documentation](https://docs.docker.com/engine/reference/commandline/inspect/) for this command.

If you run this command with your Nginx container, you should get a similar output:

```
[
    {
        "Id": "7b43a902c8414fcabf561ad94740d25d8132a825771248b0d64178c4f3590179",
        "Created": "2021-03-04T10:49:04.167682403Z",
        "Path": "/docker-entrypoint.sh",
        "Args": [
            "nginx",
            "-g",
            "daemon off;"
        ],
        "State": {
            "Status": "running",
            "Running": true,
            "Paused": false,
            "Restarting": false,
            "OOMKilled": false,
            "Dead": false,
            "Pid": 40789,
            "ExitCode": 0,
            "Error": "",
            "StartedAt": "2021-03-04T10:49:04.85190149Z",
            "FinishedAt": "0001-01-01T00:00:00Z"
        },
        "Image": "sha256:35c43ace9216212c0f0e546a65eec93fa9fc8e96b25880ee222b7ed2ca1d2151",
...
```

<!-->10 min<-->
#### Control your containers

Now that you are sure that your container is running correctly, there is several commands that allows you to control your running containers.

- To stop your container you can use the command:

```
docker stop name_of_your_container [name_of_your_container...]
```

This command sends a SIGTERM signal to your container and if your container does not stop, it sends a SIGKILL signal.
A SIGKILL terminates the process without asking its permission whereas a SIGTERM asks a process to terminate itself. A process may (but should not) refuse to terminate itself when it receives a SIGTERM.
You can use command `-t` or `--time` to add a countdown to the container's termination.

An alternative version of docker stop is `docker kill` which, by default, directly send a SIGKILL signal to your container.
You can add the `-s` or `--signal` option to choose the signal to be sent.

You can check the [documentation](https://docs.docker.com/engine/reference/commandline/stop/) of the `docker stop` command.

- To restart a stopped container, use the following command:

```
docker start name_of_your_container [name_of_your_container...]
```


This command has several interesting options which are:
    `-a` or `--attach` binds stdout and stderr to your terminal
    `-i` or `--interactive` binds stdin to your terminal

You can check the [documentation](https://docs.docker.com/engine/reference/commandline/start/) of the `docker start` command.

- To stop and restart a running container, use the following command:

```
docker restart name_of_your_container [name_of_your_container...]
```

This command combines a docker stop and a docker start command.

You can check the [documentation](https://docs.docker.com/engine/reference/commandline/restart/) of the `docker restart` command.

Try stoping and restarting the container you ran before with these commands.

- You can pause a container at any time and unpause it with the following commands:

```
docker pause name_of_your_container [name_of_your_container...]
```

```
docker unpause name_of_your_container [name_of_your_container...]
```

You may ask yourself what is the difference between `docker stop / docker start` and `docker pause / docker unpause`.
`docker stop` is stopping the container entirely so it will not use ressources from your PC until restarted. The process is not running on the PC.
`docker pause` will stop the container but it will keep the resource allocated until unpaused. The process is still running on the PC.

Try pausing and unpausing your container.

You can check the [documentation](https://docs.docker.com/engine/reference/commandline/pause/) of the `docker pause` command and the [documentation](https://docs.docker.com/engine/reference/commandline/unpause/) of the `docker unpause` command.

Now it's time to delete our fellow container (T_T).

- To remove one or more containers, you can use the following command:

```
docker rm [name_of_your_container] [name_of_your_container...]
```

Note that you can only remove a stopped container.
When you want to remove a running container, you can use the `-f` or `--force` option of `docker rm` that will stop the container before removing it.

- There is an equivalent command for images which is:

```
docker rmi [name_of_your_image] [name_of_your_image...]
```

Delete your container and the Nginx image previously downloaded with these commands.
If you run `docker ps -a` and `docker images`, nothing should be listed anymore.

You can check both the [documentation](https://docs.docker.com/engine/reference/commandline/rm/) of the `docker rm` command and the [documentation](https://docs.docker.com/engine/reference/commandline/rmi/) of the `docker rmi` command..

<!-->8 min<-->
#### Execute commands in Containers

Before executing commands inside a container, we need a new container running on our system, for that run a new Nginx container.

- To execute commands in your containers use the following command:

```
docker exec [options] name_of_your_container command [command_options]
```

This command has several interesting options:
    `-ti` allows to launch the container in interactive mode (stdin, stdout, stderr linked to your terminal), 
    `-e` allows to define environment variables, 
    `-d` allows to launch the command in detached mode

Try executing the `ls -a` command inside your running container, you should get the following output:
```
.
..
.dockerenv
bin
boot
dev
docker-entrypoint.d
docker-entrypoint.sh
etc
home
lib
lib64
media
mnt
opt
proc
root
run
sbin
srv
sys
tmp
usr
var
```

We can also run a command inside a container during the execution of the docker run command, for example:

```
docker run ubuntu ls -a
```

Now try running a Ubuntu container in interactive mode with the `bash` command. You should use the correct option so that your container is deleted when closed. Try some shell commands that you learned last year. To quit the interactive mode, you can just use `ctrl+d`.

You can check the [documentation](https://docs.docker.com/engine/reference/commandline/exec/) of the `docker exec` command.

- To bind your stdin, stdout and stderr with your container you can use the following: 

```
docker attach [options] name_of_your_container
```

When you close close the interaction with the container, it is killed and needs to be restarted. Attach is mostly used for testing, debugging and monitoring the applications running in your container. It is in fact no commonly used and reserved to specific use cases. You usually want to use `docker exec` and `docker logs` when you want to log what your application is doing.

You can check the [documentation](https://docs.docker.com/engine/reference/commandline/attach/) of the `docker attach` command.

<!-->2 min<-->
#### Debug your containers

- To monitor all processes running in your container you can type the command:

```
docker top name_of_your_container
```

Run the command on your running container. You should get the following output:

```
UID                 PID                 PPID                C                   STIME               TTY                 TIME                CMD
root                43762               43741               0                   12:31               ?                   00:00:00            nginx: master process nginx -g daemon off;
101                 43830               43762               0                   12:31               ?                   00:00:00            nginx: worker process
```

You can check the [documentation](https://docs.docker.com/engine/reference/commandline/top/) of the `docker top` command.

- If you want to display logs of your containers, use the following command:

```
docker logs [options] [name_of_your_container]
```

You can use the options:
    `--details` to add several informations, 
    `-n` to restrict to a precise number of line from the end of the logs, 
    `--since` which restrict the logs display from a specific date.

After running the command on your container, you should get the following output:
```
/docker-entrypoint.sh: /docker-entrypoint.d/ is not empty, will attempt to perform configuration
/docker-entrypoint.sh: Looking for shell scripts in /docker-entrypoint.d/
/docker-entrypoint.sh: Launching /docker-entrypoint.d/10-listen-on-ipv6-by-default.sh
10-listen-on-ipv6-by-default.sh: info: Getting the checksum of /etc/nginx/conf.d/default.conf
10-listen-on-ipv6-by-default.sh: info: Enabled listen on IPv6 in /etc/nginx/conf.d/default.conf
/docker-entrypoint.sh: Launching /docker-entrypoint.d/20-envsubst-on-templates.sh
/docker-entrypoint.sh: Launching /docker-entrypoint.d/30-tune-worker-processes.sh
/docker-entrypoint.sh: Configuration complete; ready for start up
```

You can check the [documentation](https://docs.docker.com/engine/reference/commandline/logs/) of the `docker logs` command.

<!-->25 min<-->
#### Build your first images with Dockerfiles

Now that you know how to run and manage your images and containers, let's see how to create your own images.
Dockerfiles are a specific type of text files to optimize images creation.
A Dockerfile contains a sequence of command lines executed in order that Docker uses to build images automatically.

To build an image using a Dockerfile, use the following command:

```
docker build [options] path|url
```

By default, the build command searches for a file named Dockerfile in the current directory.
The path or url argument is the path where the image should be built. It can be a local path or a git repository for example.
Everything that is located in the given path will be sent to the Docker daemon. You can use `.dockerignore` files to create exceptions for some files.

You can also use these options:
    `-t` or `--tag` for tagging an image, 
    `-f` or `--file` allows to specify the `Dockerfile` you want to use.

The docker build command has a lot of interesting options, we invite you to check the [documentation](https://docs.docker.com/engine/reference/commandline/build/) and tests some of them. .

- Tagging an image

```
docker tag source_image[:tag] target_image[:tag]
```

This command creates a tag target_image that refers to the source image (it is basically a rename of the image):

    - registery : the server of images to use

    - author : the author of the image (if the author is _, it means that Docker is the author and it can be omitted)

    - name : the image's name

    - tag : the tag of the tag is used to know the version number of the tag. Default tag is "latest". For the images associated to:

            - languages, such as the haskell image, the tag matches the language version : haskell:8.8.4

            - services, such as the gitlab-runner image, the tag corresponds to the service version and to the version of underlying layers : gitlab/gitlab-runner:alpine-bleeding, gitlab/gitlab-runner:alpine-v13.5.0-rc2, gitlab/gitlab-runner:ubuntu-v13.4.0-rc1

Let's explore some of the most useful instructions to build an image with Dockerfiles. You can find the [spec](https://docs.docker.com/engine/reference/builder/) online.

##### `FROM`
To create a valid `Dockerfile`, this instruction is **mandatory**. A docker image always starts from an existing layer.
It allows to know the basic image in which you build your own. (You can find all the available base images on [repository docker](https://hub.docker.com/search/?q=&type=image)).
And it looks like that:
```dockerfile
FROM <image>[:<tag>] [AS <name>]
```

Let's try to:
 - Put this in a `Dockerfile` file:
     ```dockerfile
     FROM ubuntu:19.04
     ```
 - then build the image:
    ```bash
    docker build .
    ```

We obtain this:
```bash
> docker build .
Sending build context to Docker daemon  2.048kB
Step 1/1 : FROM ubuntu:19.04
 ---> 9f3d7c446553
Successfully built 9f3d7c446553
```
We can observe that Docker will "send" the Dockerfile to the docker daemon in order to build it.
This daemon will then execute the only step, returning the digest of the layer and finishing to build the image.

<!-- TODO: Parler de la taille d'une image-->

##### `RUN`
It may look awesome to use a basic image, but how can we execute commands during building ? For that, we can use the `RUN` instruction : It allows to execute shell commands during building.

There is either the "simple" version:
```dockerfile
RUN <command>
```
It will execute the command with default OS shell: `/bin/sh -c` for Linux or `cmd /S /C ` on Windows.

or the "exec" version:
```dockerfile
RUN ["executable", "param1", "param2", ...]
```

Tips: we can use backslash `\` to create a `RUN` instruction on multiple lines.

Let's try:
 - We start from the last time `Dockerfile`:
     ```dockerfile
     FROM ubuntu:19.04
     ```
 - And we add:
     ```dockerfile
     FROM ubuntu:19.04
     RUN cat /etc/os-release
     ```
 - Then build the image:
    ```bash
    docker build .
     ```

We obtain:
```bash
> docker build .
Sending build context to Docker daemon  2.048kB
Step 1/2 : FROM ubuntu:19.04
 ---> 9f3d7c446553
Step 2/2 : RUN cat /etc/os-release
 ---> Running in 41ff7a13ca27
NAME="Ubuntu"
VERSION="19.04 (Disco Dingo)"
ID=ubuntu
ID_LIKE=debian
PRETTY_NAME="Ubuntu 19.04"
VERSION_ID="19.04"
HOME_URL="https://www.ubuntu.com/"
SUPPORT_URL="https://help.ubuntu.com/"
BUG_REPORT_URL="https://bugs.launchpad.net/ubuntu/"
PRIVACY_POLICY_URL="https://www.ubuntu.com/legal/terms-and-policies/privacy-policy"
VERSION_CODENAME=disco
UBUNTU_CODENAME=disco
Removing intermediate container 41ff7a13ca27
 ---> 92b8b7f95f77
Successfully built 92b8b7f95f77
```

We may observe instructions caching when doing a rebuild:
```bash
> docker build .
Sending build context to Docker daemon  2.048kB
Step 1/2 : FROM ubuntu:19.04
 ---> 9f3d7c446553
Step 2/2 : RUN cat /etc/os-release
 ---> Using cache
 ---> 92b8b7f95f77
Successfully built 92b8b7f95f77
```

We can see that no layers were rebuilt as expected.
This feature allows to gain time and resources when building huge images.

##### `COPY`, `ADD`
Now that we can execute shell commands in container, we might as well not stay in a pure fonctionnal world and let's try to make some side-effects on files.

```dockerfile
# ADD
ADD <src>... <dest>
ADD ["<src>",... "<dest>"]

# COPY
COPY <src>... <dest>
COPY ["<src>",... "<dest>"]
```

The difference between both instructions is that `ADD` allows adding files from different sources and manipulating distant files (via URLs) while `COPY` only add local files or directories.

Lets try to do a small hello-world in Python:
 - We create a small Python file:
     ```bash
     echo "print('hello world')" > app.py
     ```
 - We will start from a `python` image for the `Dockerfile`:
     ```dockerfile
     FROM python:3.8
     COPY app.py .
     RUN python app.py
     ```
 - Then build the image:
    ```bash
    docker build .
     ```

We obtain:
```bash
> docker build .
Sending build context to Docker daemon   5.12kB
Step 1/3 : FROM python:3.8
 ---> 28a4c88cdbbf
Step 2/3 : COPY app.py .
 ---> f6db079cdc6c
Step 3/3 : RUN python app.py
 ---> Running in 972191f3bec2
Hello, World!
Removing intermediate container 972191f3bec2
 ---> be8feedcb09d
Successfully built be8feedcb09d
```

##### `ENV` vs `ARG`
For more flexible configuration, Docker loves environment variables.
These are variables where value can only be known at runtime.

For example, we could store the login of a database or the port number to bind in environment variables.

In the Dockerfile, these variables are indicated with the `ENV` instruction:
```dockerfile
ENV DB_PASS=super_secure_password
ENV PORT=8080
```

These variables can also be changed during the use of the container.

Tips: For variables only used in the build of the image, we can use `ARG`

Let's try to do a hello-world in Rust :
 - We will create a basic rust file `app.rs`:
     ```rust
     use std::env;

     fn main() {
         match env::var("LANG") {
             Ok(lang) => {
                 if lang.eq(&String::from("EN")) {
                     println!("Hello World");
                 } else {
                     println!("Bonjour Monde");
                 }
             }
             Err(e) => println!("Couldn't read LANG ({})", e),
         };
     }
     ```
 - We will start from a `rust` image for the `Dockerfile`:
     ```dockerfile
     FROM rust:1.31
     COPY app.rs .
     RUN rustc app.rs -o app
     ENV LANG=EN
     RUN ./app
     ```
 - Then build the image:
    ```bash
    docker build .
     ```

We obtain:
```bash
> docker build .
Sending build context to Docker daemon   16.9kB
Step 1/5 : FROM rust:1.31
 ---> 6f61eb35ad91
Step 2/5 : COPY app.rs .
 ---> d5b1570b2c63
Step 3/5 : RUN rustc app.rs -o app
 ---> Running in e6354d1ff410
Removing intermediate container e6354d1ff410
 ---> 24d81400a54e
Step 4/5 : ENV LANG=EN
 ---> Running in 401cf76f6475
Removing intermediate container 401cf76f6475
 ---> ac92e5f19357
Step 5/5 : RUN ./app
 ---> Running in 5d3c41a3a85e
Hello World
Removing intermediate container 5d3c41a3a85e
 ---> d8349d6d185c
Successfully built d8349d6d185c
```

And if we change the variable's value, we obtain:
```bash
> docker build .
Sending build context to Docker daemon   16.9kB
Step 1/5 : FROM rust:1.31
 ---> 6f61eb35ad91
Step 2/5 : COPY app.rs .
 ---> Using cache
 ---> d5b1570b2c63
Step 3/5 : RUN rustc app.rs -o app
 ---> Using cache
 ---> 24d81400a54e
Step 4/5 : ENV LANG=FR
 ---> Running in 59fc11b76b80
Removing intermediate container 59fc11b76b80
 ---> 5535e8d40a4c
Step 5/5 : RUN ./app
 ---> Running in 16d58e234947
Bonjour Monde
Removing intermediate container 16d58e234947
 ---> 44f72d0290f1
Successfully built 44f72d0290f1
```

##### `CMD` vs `ENTRYPOINT`
Now that we compiled, we can run the binary at the start of the container.

For that we write:
```dockerfile
ENTRYPOINT ["executable", "param1", "param2"]
CMD <command>
```

`ENTRYPOINT` allows to define an entry point for the container.

In the case where an `ENTRYPOINT` is defined, it is possible to define a `CMD` in order to create more options. Actually, `ENTRYPOINT` and `CMD` have a similar role. However, `ENTRYPOINT` is static while `CMD` is dynamic. It is then possible to create an `ENTRYPOINT` merged with a `CMD` in order to finally have a part of static and dynamic arguments. Careful, this feature is not easy to manage and require practicing.

For example, for the Traefik Reverse Proxy, the [`Dockerfile`](https://github.com/traefik/traefik/blob/master/Dockerfile) uses `ENTRYPOINT` to start the binary `CMD` to give him parameters.

Let's try to make a hello-world in rust :
 - We will start a `ubuntu` image for the `Dockerfile`:
     ```dockerfile
     FROM ubuntu:19.04
     ENTRYPOINT ["bash"]
     CMD ["--posix"]
     ```
 - Then build the image:
    ```bash
    docker build .
     ```

We obtain:
```bash
> docker build .
Sending build context to Docker daemon  3.072kB
Step 1/3 : FROM ubuntu:19.04
 ---> 9f3d7c446553
Step 2/3 : ENTRYPOINT ["bash"]
 ---> Running in d7c726e2c46d
Removing intermediate container d7c726e2c46d
 ---> ef0aacedc3fb
Step 3/3 : CMD ["--posix"]
 ---> Running in 41fcaa66d49b
Removing intermediate container 41fcaa66d49b
 ---> 439853250ab9
Successfully built 439853250ab9
```

During the container's run, the `CMD` instruction can be replaced while the `ENTRYPOINT` instruction can not.

##### `WORKDIR`
We managed to launch commands, but we never specified were we came from. We can now use the `WORKDIR` instruction to do so.

```dockerfile
# WORKDIR
WORKDIR /path/to/workdir
```

An example of Dockerfile :

```dockerfile=
FROM ubuntu:19.04
WORKDIR a
WORKDIR b
WORKDIR c
RUN pwd

WORKDIR /d/e/f
RUN pwd
```

And the output :
```bash
> docker build .
docker build .
Sending build context to Docker daemon  3.072kB
Step 1/7 : FROM ubuntu:19.04
 ---> 9f3d7c446553
Step 2/7 : WORKDIR a
 ---> Running in 862c9c25b001
Removing intermediate container 862c9c25b001
 ---> b5182f87efaf
Step 3/7 : WORKDIR b
 ---> Running in a025044916fb
Removing intermediate container a025044916fb
 ---> 00639a0c768a
Step 4/7 : WORKDIR c
 ---> Running in f7636684c808
Removing intermediate container f7636684c808
 ---> c443fe035762
Step 5/7 : RUN pwd
 ---> Running in 666de2796715
/a/b/c
Removing intermediate container 666de2796715
 ---> 933bc24f7abd
Step 6/7 : WORKDIR /d/e/f
 ---> Running in 6a73fa4b4b1a
Removing intermediate container 6a73fa4b4b1a
 ---> 81d89d1f7712
Step 7/7 : RUN pwd
 ---> Running in df4d54248466
/d/e/f
Removing intermediate container df4d54248466
 ---> cd3537b0b0a7
Successfully built cd3537b0b0a7
```

##### `EXPOSE`
The previous steps allowed you to create a Dockerfile and to create an image that can run an application. We now need to let users access your application. For that we use the `EXPOSE` command.

In a Dockerfile we have:
```dockerfile
# EXPOSE
EXPOSE <port> [<port>/<protocol>...]

#En pratique
EXPOSE 80/tcp
```

```dockerfile=
FROM python:3.8
EXPOSE 8000
CMD python -m http.server 8000
```

With this Dockerfile, the container will have an open port which we will need to bind to one of ours.
In the latest versions of docker, the `EXPOSE` instruction is no longer used to open ports but still used as a documentation between the person who builds the image and the person who runs the container. To open ports on a container you should use the `-p` option of docker run.

We obtain:
```bash
> docker build -t python_image .
Sending build context to Docker daemon  3.072kB
Step 1/3 : FROM python:3.8
 ---> 28a4c88cdbbf
Step 2/3 : EXPOSE 8000
 ---> Running in 74ed5b622239
Removing intermediate container 74ed5b622239
 ---> 7eeb65ba6434
Step 3/3 : CMD python -m http.server 8000
 ---> Running in c3d9cee464b8
Removing intermediate container c3d9cee464b8
 ---> c82182474b7f
Successfully built c82182474b7f
```

You can then run the server using the following command:

```
docker run -d -p 8000:8000 --name python_container python_image
```

You should now be able to access the server on [localhost:8000](http://localhost:8000).

<!-->10 min<-->
#### Manage your Images with Registries

A registry is a virtual system of storage space that is used by Docker for your images.
As you saw before, when you want to use an official Docker image, Docker pulls this image from Docker official registry to your local registry.
You can use a public or private registry to publish your own images and use them later.
You can find popular public registries like Docker-hub for example.

To pull an image from a distant registry, you can use the following command:

```
docker pull [options] tag
```

Try pulling the official `PHP` image from Docker's repository. You can now use the state of the art `PHP` language for your beautiful projects.

You can also push a local image to a distant registry:

```
docker push [options] tag
```

You can create your own registry server if you want, using the official registry image.

```
docker run -d -p 5000:5000 --name registry registry:2
```

This command starts a container which contains a registry server.
The server is accessible on the port 5000 (due to the `-p` option).

Create a registry server using the command above and let's play with it.

- First Step: Create an image

Create a new PHP file named `main.php`:

```php
<?php echo 'Long live King Stephan'; ?>
```

Then create a `Dockerfile` containing these instructions:

```
FROM php:7.4-cli
COPY . /usr/src/myapp
WORKDIR /usr/src/myapp
CMD [ "php", "./main.php" ]
```

Build your image by running:
```
docker build -t php_image .
```

Verify that your image is working by running this command:

```
docker run -it --rm --name php_container php_image
```

- Second Step: Push image

You need to tag your image so it can be pushed to your server:

```
docker tag php_image localhost:5000/my_php_image
```

Now push your image to your registry server by running:

```
docker push localhost:5000/my_php_image
```

- Third Step: Delete local image

Now you can delete your local image:

```
docker rmi php_image localhost:5000/my_php_image
```

- Fourth Step: Pull image from registry

Now your image is stored in the distant registry and can be pulled anytime by anyone who has access to it.

You can get your image by running:

```
docker pull localhost:5000/my_php_image
```

And run it by typing:

```
docker run -it --rm --name php_container localhost:5000/my_php_image
```

Don't hesitate to check the docker [documentation](https://docs.docker.com/registry/) for registries :).

<!-->10 min<-->
#### Keep persisting data with Volumes

A container is stateful. But an image is not. If you start an image containing a database, and you update this database, the data will be lost when you will delete the container. If you restart this image, the database will be restored to its initial state. To avoid that and keep our data, we need to use volumes.
A volume is a shared space between your computer and your containers. Volumes are not deleted by default when you delete a container.
There is two kinds of volumes, virtual volumes and physical volumes. Physical volumes are a physical directory of your file system while virtual volumes are virtual space managed by Docker (it is stored in the Docker intern files 'var/lib/local/volumes'). 
You can delete virtual volumes using the options `--volumes` for the rm command (but you already know that since we saw it before ^^).

You can create a virtual volume using the following command:

```
docker volume create [options] [volume]
```

You can list all volumes with:

```
docker volume ls [options]
```

You can also get more detailed informations about a specified volume (or more) with the command:

```
docker volume inspect [options] volume [volume...]
```

You can delete a volume with:

```
docker volume rm [options] volume [volume...]
```

You can bind a volume to a container during the execution of the docker run command with the `-v` option.

Examples:
    Physical volume: `docker run -v /home/local_directory/:./container_directory/`, 
    virtual volume: `docker run -v name_of_the_virtual_volume:./container_directory`.

Let's try to practice with volumes:

- Use the command 
```
docker run -d --name mysql_container -e MYSQL_ROOT_PASSWORD=example -v $PWD/mysql/data:/var/lib/mysql mysql
```

This command will create a container with a basic mysql image with a root password and a shared volume. You can see that a mysql folder was automatically created in your current directory.

- Next, use the command
```
docker run -d -p 8080:8080 --name adminer_container --link mysql_container:db adminer
```

This command will create a configuration panel for your database. Notice the --link option that allows to link the adminer container to the database without needed to open database ports.

- Now, connect to adminer using [localhost:8080](http://localhost:8080) with the root credentials (in the database input enter "mysql" because the root account created is a general mysql account and not only for your database) and create a new database labelled `my_db` and in it you can create a new table `my_table` with an `id` column.

- Then you can do `docker rm -f adminer_container mysql_container`. You can see that the mysql directory created by the run command is still here with all its content.

- Then rerun these containers with previously used commands. Return to [localhost:8080](http://localhost:8080). You can see that your database and table were restored.

This notion is quite complicated so don't be afraid to ask questions ^^ (don't cry pls) or check the documentation.

Don't hesitate to check the docker [documentation](https://docs.docker.com/storage/volumes/) for volumes :).

#### Clean your Docker

- To clean everything you have done during this TP, use:

```
docker system prune [options]
```

It is used to wipe out all unused containers, images, networks and volumes when you add the option `--volumes`.
By default, it will not delete running containers or used images.

You can check the [documentation](https://docs.docker.com/engine/reference/commandline/system_prune/) for `docker system prune` command.

### IV. Final Project

### V. To go further

 - [`HEALTHCHECK`](https://docs.docker.com/engine/reference/builder/#healthcheck)
 - [`USER`](https://docs.docker.com/engine/reference/builder/#user)
 - [`.dockerignore`](https://docs.docker.com/engine/reference/builder/#dockerignore-file)
 - multi layer images: We can create lighter images using temporary images for compiling binary files:
     ```dockerfile
     # image for compiling the file
     FROM golang:1.13.4 AS builder
     # ...
     RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o app .

     # lighter image for running the app
     FROM scratch
     COPY --from=builder app .
     # ....
     ```

 <!-->1 min<-->
#### Import and Export Images

As a container is only a file system with metadatas, you can easily export and reimport your containers as a .tar archive.

```
docker export [options] name_of_the_container
```

It can be used with two syntaxes:
    `docker export red_panda > latest.tar`,  
    `docker export --output="latest.tar" red_panda`

```
docker import [options] file|url
```

These commands are not recommanded to use because there are other better ways to share your containers like registries.

You can check both the [documentation](https://docs.docker.com/engine/reference/commandline/export/) of the `docker export` command and the [documentation](https://docs.docker.com/engine/reference/commandline/import/) of the `docker import` command..

