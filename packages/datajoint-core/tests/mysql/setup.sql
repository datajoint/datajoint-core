-- https://github.com/prisma/database-schema-examples/tree/master/postgres/basic-twitter#basic-twitter
CREATE TABLE tweet
(
    id         BIGINT PRIMARY KEY AUTO_INCREMENT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    text       varchar(100)      NOT NULL,
    owner_id   BIGINT
);