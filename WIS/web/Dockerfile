FROM node:8 as react-build

ARG BUILD_VERSION

WORKDIR /app
COPY . ./
RUN yarn
RUN REACT_APP_BUILD_VERSION=${BUILD_VERSION} yarn build

FROM nginx:alpine
COPY --from=react-build /app/build /etc/nginx/html
EXPOSE 80
COPY nginx.conf /etc/nginx/conf.d/default.conf
CMD ["nginx", "-g", "daemon off;"]
