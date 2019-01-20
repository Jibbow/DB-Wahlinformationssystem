FROM node:8 as react-build

ARG TRAVIS=false
ARG TRAVIS_COMMIT=
ARG TRAVIS_BRANCH=

WORKDIR /app
COPY . ./
RUN yarn
RUN REACT_APP_VERSION_INFO=${TRAVIS} REACT_APP_COMMIT_ID=${TRAVIS_COMMIT} REACT_APP_BRANCH=${TRAVIS_BRANCH} yarn build

FROM nginx:alpine
COPY --from=react-build /app/build /etc/nginx/html
EXPOSE 80
COPY nginx.conf /etc/nginx/conf.d/default.conf
CMD ["nginx", "-g", "daemon off;"]
