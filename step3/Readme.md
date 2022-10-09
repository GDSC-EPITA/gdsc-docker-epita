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


