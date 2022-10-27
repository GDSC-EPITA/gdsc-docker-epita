#### Clean your Docker

- To clean everything you have done during this TP, use:

```
docker system prune [options]
```

It is used to wipe out all unused containers, images, networks and volumes when you add the option `--volumes`.
By default, it will not delete running containers or used images.

You can check the [documentation](https://docs.docker.com/engine/reference/commandline/system_prune/) for `docker system prune` command.


### IV. To go further

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

#### Import and Export Images [~1 min]

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

