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


