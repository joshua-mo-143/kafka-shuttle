-- Add migration script here
create table if not exists messages (
     message_id int primary key,
     name varchar not null,
     message varchar not null
);
