FROM twote-javascript-runtime

WORKDIR /app

COPY scripts/ /app/scripts/
COPY schemas/ /app/schemas/
COPY twote-web/ /app/twote-web/

# Provide execute permissions for the script
RUN chmod +x ./scripts/proto.sh

# Run the proto script
RUN ./scripts/proto.sh

WORKDIR twote-web

# Install npm
RUN npm install

EXPOSE 3000

CMD ["npm", "start"]

