# Use official Node.js LTS image for building the app
FROM node:22 AS build

# Set working directory in the container
WORKDIR /app

# Copy package.json and package-lock.json
COPY package.json package-lock.json ./

# Install dependencies
RUN npm install

# Copy the rest of the application files
COPY . .

# Build the application
RUN npm run build

# Use official nginx image for serving the built app
FROM nginx:1.25

# Copy built files from the previous stage
COPY --from=build /app/dist /usr/share/nginx/html

# Copy custom nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 80
EXPOSE 3000

# Start nginx server
CMD ["nginx", "-g", "daemon off;"]
