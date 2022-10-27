### Basic Docker commands

Now that you begin to understand how Docker works let's try to play with some containers.

#### Run your first Containers [~5 min]

First, type the following command:

```
docker run --name name_of_your_container -d nginx
```

This command should start a new Nginx container.
Docker starts by pulling a Nginx image from Docker's official registries and then run this image to create a container.
To start another Nginx container, Docker will use the local image previously downloaded.
The --name option allows to name your container.

`run`:

Allows to launch containers from images, This command has several interesting options:
|Option| Do |
|------|------|
|`-p`|  Bind your container's ports to your PC |
|`-d`|  Launch your container in detached mode (similar to a background mode) |
| `-it`|  Launch the container in interactive mode (stdin, stdout, stderr linked to your terminal) |
|`-e`|  Define environment variables |
|`-v`|  mount a volume in the container (we will develop this notion later) |
|`--name`|  give a name to the container |
|`--rm`|  when a containers exits this option clean the container and its associated data and virtual volumes |


If you want more informations about the `run` command and its associated options in the [documentation](https://docs.docker.com/engine/reference/run/).
The documentation is quite complete so don't hesitate to have a look on it.

#### Monitor your Images and Containers [~10 min]

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


#### Control your containers [~15 min]

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


#### Execute commands in Containers [~10 min]

Before executing commands inside a container, we need a new container running on our system, for that run a new Nginx container.

- To execute commands in your containers use the following command:

```
docker exec [options] name_of_your_container command [command_options]
```

This command has several interesting options:
|Option| Do |
|------|------|
| `-ti`| Launch the container in interactive mode (stdin, stdout, stderr linked to your terminal) |
| `-e` | Define environment variables |
| `-d` | Launch the command in detached mode |

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

#### Debug your containers [~2 min]

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
|Option| Do |
|------|------|
|`--details` | Add several informations | 
|`-n` | Restrict to a precise number of line from the end of the logs |
|`--since` | Restrict the logs display from a specific date. |

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


