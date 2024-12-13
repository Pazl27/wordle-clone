FROM postgres:13

ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=password
ENV POSTGRES_DB=mydb

COPY ./init.sql /docker-entrypoint-initdb.d/

EXPOSE 5432

