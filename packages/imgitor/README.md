## Docker
### Local

```
$ docker build -t imgitor .
$ docker run -it --rm --entrypoint "bash" imgitor
$ docker run \
  --env ROCKET_PORT=8080 \
  --env ROCKET_ADDRESS=0.0.0.0 \
  --env GCP_CLOUD_STORAGE_READ_BUCKET_NAME=$(cat .env | grep GCP_CLOUD_STORAGE_READ_BUCKET_NAME | sed s/GCP_CLOUD_STORAGE_READ_BUCKET_NAME=//) \
  --env GCP_CLOUD_STORAGE_WRITE_BUCKET_NAME=$(cat .env | grep GCP_CLOUD_STORAGE_WRITE_BUCKET_NAME | sed s/GCP_CLOUD_STORAGE_WRITE_BUCKET_NAME=//) \
  -p 8080:8080 \
  -it --rm --entrypoint "bash" imgitor
```

### GCP

```
$ docker build -t gcr.io/$(gcloud config get-value project)/imgitor:latest .
$ docker push gcr.io/$(gcloud config get-value project)/imgitor:latest
```

#### Other

```
$ gcloud tasks create-http-task \
--queue=await \
--url=https://rocket.rs/v0.5-rc/guide/overview/ \
--method='GET'
```