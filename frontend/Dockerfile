# Use the NodeJS image as builder
FROM node:lts AS builder

# Create the workspace
WORKDIR /usr/src/app

# Copy over the package, and install the dependencies
COPY package.json .
RUN yarn

# Copy over the other files.
COPY src ./src
COPY tsconfig.json .
COPY .babelrc .
COPY .postcssrc .

# Build the application
RUN yarn build

# The actual server, this builds the final image
FROM nginx:alpine

# Copy the nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Copy the output of the builder
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html
RUN chown nginx:nginx /usr/share/nginx/html/ -R
EXPOSE 8000
