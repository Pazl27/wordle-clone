# Use Node.js image to build the Vue.js app
FROM node:18 AS build

# Set working directory
WORKDIR /app

# Install dependencies
COPY package*.json ./
RUN npm install

COPY . .
RUN npm run dev

# Serve the app with Nginx
FROM nginx:alpine

# Copy the built app from the previous stage
COPY --from=build /app/dist /usr/share/nginx/html

# Expose port 80 to serve the frontend
EXPOSE 80

# Start Nginx to serve the app
CMD ["nginx", "-g", "daemon off;"]

