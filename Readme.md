### GDSC Docker Workshop

In this workshop, you will learn to use docker to put services and applications into containers. You will even learn how to write your Dockerfiles, to put your own applications in containers in the future.


- Part 1: Live presentation of what is docker and why you should use it (for those not present a quick summary is avaible bellow)
- Part 2: Practical composed of 5 different steps to better understand and use docker

#### Prerequisite:
- Basic knowledge of the how CLI works and how to execute commands

#### Summary:
- [Step 1 Installation](step1-Install/Readme.md)
- [Step 2 Basic command (creating, deleting, running, executing, logging)](step2-Basic/Readme.md)
- [Step 3 Dockerfiles](step3-Dockerfiles/Readme.md)
- [Step 4 Advanced (registries, volumes)](step4-Advanced/Readme.md)
- [Step 5 Cleanup](step5-Cleanup/Readme.md)

![](https://www.docker.com/sites/default/files/d8/2019-07/vertical-logo-monochromatic.png)

#### What is Docker ? [~5 min]
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

#### Credit

This workshop was made by the GDSC of EPITA and inspired by the 2021 / 2022 promotion of epita students in the TCOM specialization and the assistant team of thoses promotions, special thanks to:
- TCOM 2021 Tom Moulard, Martin Huvelle, Adrien Nebon-Carle
- TCOM 2022 Corentin Lefevre-Pontalis, Adrien Devouassoux
- EPITA GDSC 2023 Alexandre Dias, Mickael Chau
