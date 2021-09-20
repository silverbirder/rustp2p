# rustp2p

## Original Docker

```
docker pull rust:1.48.0-alpine
docker run --rm -it -e USER=$USER -v $(pwd):/app/ rust:1.48.0-alpine /bin/ash
apk update
apk add vim alpine-sdk libressl-dev
```

## My Docker

```
docker build -t rustp2p .
docker run --rm -it -e USER=$USER -v $(pwd):/app/ rustp2p /bin/ash
```