```
$ docker build -t imgitor .
$ docker run -it --rm --entrypoint "bash" imgitor
$ docker run -it --rm imgitor
```

```
$ gcloud tasks create-http-task \
--queue=await \
--url=https://rocket.rs/v0.5-rc/guide/overview/ \
--method='GET'
```