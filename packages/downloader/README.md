## Dev

```
$ docker build -t downloader .
$ docker run -p 8080:8080 --rm downloader
$ docker run -p 8080:8080 -it --rm --entrypoint "ash" downloader
```