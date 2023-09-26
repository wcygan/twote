FROM postgres:latest

# local folder --> docker folder /scripts/
COPY scripts/init-accounts-db.sql /scripts/init-accounts-db.sql

# Grant execute permissions to the script
RUN chmod +x /scripts/init-accounts-db.sql