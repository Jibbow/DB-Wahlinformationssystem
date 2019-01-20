FROM node:8 as react-build

ARG BUILD_VERSION

WORKDIR /app
COPY . ./
RUN yarn
RUN REACT_APP_BUILD_VERSION=${BUILD_VERSION} yarn build

FROM nginx:alpine
COPY --from=react-build /app/build /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
