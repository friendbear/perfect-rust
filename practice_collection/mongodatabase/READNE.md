# MongoDB


起動

```sh
docker run \
  --name mdb \
  --publish 27017:27017 \
  --detach \
  mongo
```

接続

```sh
docker exec -it mdb \
  mongosh
```
