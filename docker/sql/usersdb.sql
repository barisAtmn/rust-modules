CREATE DATABASE usersdb;
\c usersdb;

create type gender_t as enum('Woman', 'Man', 'Other');

create type employment_status_t as enum('Employee', 'Employer');

CREATE TABLE users (
   id serial NOT NULL,
   PRIMARY KEY (id),
   name character varying NOT NULL,
   last_name character varying NOT NULL,
   country character varying NOT NULL,
   gender gender_t,
   employment_status employment_status_t,
   age smallint
);
