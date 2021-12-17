FROM node:16.13.0-slim

WORKDIR /usr/src/app

COPY package*.json ./

RUN npm install
COPY src src

ENTRYPOINT ["npm"]
CMD ["start"]