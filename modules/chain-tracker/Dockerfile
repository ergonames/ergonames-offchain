FROM node:16-alpine

RUN mkdir -p /app

WORKDIR /app

COPY . .

RUN npm install
RUN npx prisma generate


CMD ["npm", "run", "start"]