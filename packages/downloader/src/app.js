const WebTorrent = require('webtorrent')
const fs = require('fs');
const { drive, auth } = require('@googleapis/drive');
const { JWT } = auth;

const client = new WebTorrent();

const jwtClient = new JWT(
    process.env.SERVICE_ACCOUNT_CLIENT_EMAIL,
    null,
    process.env.SERVICE_ACCOUNT_PRIVATE_KEY,
    ['https://www.googleapis.com/auth/drive.file'],
);

const express = require('express');
const app = express();

app.use(express.json());

const _log = (message) => {
    if (process.env.NODE_ENV !== 'production') {
        console.log(message);
    }
}

const _err_log = (error) => {
    console.error(error);
}

app.get('/', async (req, res) => {
    const magnetURI = req.query.magnet_uri;
    if (!magnetURI) {
        res.status(200).send('not found magnetURI');
        return;
    }
    await jwtClient.authorize();
    client
        .on('error', (err) => {
            _log('catch client error');
            _err_log(err);
        })
        .add(magnetURI, { path: process.env.OUTPUT_PATH, destroyStoreOnDestroy: true }, (torrent) => {
            torrent
                .on('done', async () => {
                    console.log('torrent done event');
                    const driveService = drive({
                        version: 'v3',
                        auth: jwtClient
                    });
                    console.log('file uploading...');
                    await Promise.all(torrent.files.map( async (file) => {
                        return driveService.files.create({
                            resource: {
                                name: file.name,
                                parents: [process.env.GOOGLE_DRIVE_ID]
                            },
                            media: {
                                body: fs.createReadStream(file.path)
                            },
                        });
                    }));
                    console.log('file uploaded');
                    torrent.destroy();
                    console.log('torrent destroy');
                })
                .on('warning', (err) => {
                    _log('torrent warning event');
                    _err_log(err);
                })
                .on('error', (err) => {
                    _log('torrent error event');
                    _err_log(err);
                    torrent.destroy();
                })
                .on('download', (bytes) => {
                    _log(`【torrent download event】progress:${torrent.progress},downloaded:${torrent.downloaded},downloadSpeed:${torrent.downloadSpeed}(numPeers:${torrent.numPeers})`);
                })
        });
    res.status(200).send('recept magnetURI');
});

module.exports = app;