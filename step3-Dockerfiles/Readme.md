
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


