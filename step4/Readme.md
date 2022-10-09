
<!-->8 min<-->
#### Execute commands in Containers

Before executing commands inside a container, we need a new container running on our system, for that run a new Nginx container.

- To execute commands in your containers use the following command:

```
docker exec [options] name_of_your_container command [command_options]
```

This command has several interesting options:
| `-ti`| allows to launch the container in interactive mode (stdin, stdout, stderr linked to your terminal) |
| `-e` | allows to define environment variables |
| `-d` | allows to launch the command in detached mode |

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


