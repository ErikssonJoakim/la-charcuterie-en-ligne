# Containerized MySQL Database

## Requirements

You'll need to install docker in order to launch the docker container: https://docs.docker.com/engine/install/

## 🚚 Building the image

```sh
docker build -t <IMAGE_NAME> .
```

## 🏃‍♂️ Running the container

```sh
docker run --restart=always -d --env-file=<ENV_FILE> -v <DATA_DIRECTORY>:/var/lib/mysql --name <CONTAINER_NAME> <IMAGE_NAME>
```

Where `DATA_DIRECTORY` is the absolute path to the directory where you wish to store the server data.

MySQL requires environment variables to successfully run a database. You can find the different variables on their Docker Hub page: https://hub.docker.com/_/mysql/

### Verify your entries

Easiest way to verify your entries is to run mysql in the container terminal:

```sh
docker exec -it <CONTAINER_NAME> mysql -u root -p
```

You will be asked to enter the database password and after that you're free to start making mysql queries in the terminal!
