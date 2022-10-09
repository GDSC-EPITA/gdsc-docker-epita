
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


