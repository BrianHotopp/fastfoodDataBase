FROM node:15-alpine
WORKDIR /frontend
# copy both 'package.json' and 'package-lock.json' (if available)
COPY ./fastfood_frontend/package*.json ./

# install project dependencies
RUN npm install -f
RUN npm install -g --silent vue-cli
EXPOSE 8080
