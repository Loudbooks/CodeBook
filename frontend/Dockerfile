FROM node:23-alpine AS build

WORKDIR /codebook-frontend

ARG COMMIT_HASH
ENV VITE_COMMIT_HASH=$COMMIT_HASH

RUN echo "VITE_COMMIT_HASH=$VITE_COMMIT_HASH" >> .env

COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

FROM node:23-alpine

WORKDIR /codebook-frontend

COPY --from=build /codebook-frontend /codebook-frontend

RUN npm install --production

EXPOSE 3000

CMD ["node", "build"]