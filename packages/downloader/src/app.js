const WebTorrent = require('webtorrent')
const { Storage } = require('@google-cloud/storage');

const client = new WebTorrent();
const storage = new Storage();

const uploadFile = async (filePath, destFileName) => {
    return storage.bucket(process.env.CLOUD_STORAGE_BUCKET_NAME).upload(filePath, {
        destination: destFileName,
    });
}

const express = require('express');
const app = express();

app.use(express.json());

const _log = (message) => {
    if (process.env.NODE_ENV !== 'production') {
        console.log(message);
    }
}

app.get('/', async (req, res) => {
    const magnetURI = req.query.magnet_uri;
    if (!magnetURI) {
        res.status(200).send('not found magnetURI');
        return;
    }
    client
        .on('error', (err) => {
            _log('catch client error');
            _log(err);
        })
        .add(magnetURI, { path: process.env.OUTPUT_PATH, destroyStoreOnDestroy: true }, (torrent) => {
            torrent.progressRound = 0;
            torrent
                .on('done', async () => {
                    console.log(`${torrent.name} torrent done event`);
                    console.log(`${torrent.name} file uploading...`);
                    await Promise.all(torrent.files.map(async (file) => {
                        return uploadFile(file.path, file.name);
                    }));
                    console.log(`${torrent.name} file uploaded`);
                    torrent.destroy();
                    console.log(`${torrent.name} torrent destroy`);
                })
                .on('warning', (err) => {
                    _log('torrent warning event');
                    _log(err);
                })
                .on('error', (err) => {
                    _log('torrent error event');
                    _log(err);
                    torrent.destroy();
                })
                .on('download', (bytes) => {
                    // 0.9983902939757067 -> 0.99
                    const RoundedTwoDigitProgress = Math.floor(torrent.progress * Math.pow(10, 2)) / Math.pow(10, 2);
                    if (RoundedTwoDigitProgress > torrent.progressRound) {
                        console.log(`【${torrent.name} torrent download event】progress:${torrent.progress},downloaded:${torrent.downloaded},downloadSpeed:${torrent.downloadSpeed}(numPeers:${torrent.numPeers})`);
                        torrent.progressRound = RoundedTwoDigitProgress;
                    }
                })
        });
    res.status(200).send('recept magnetURI');
});

module.exports = app;