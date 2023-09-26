DROP TABLE users;

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL
);

INSERT INTO users (user_id, username, password) VALUES
(1, 'user1', 'password1'),
(2, 'user2', 'password2'),
(3, 'user3', 'password3'),
(4, 'user4', 'password4'),
(5, 'user5', 'password5'),
(6, 'user6', 'password6'),
(7, 'user7', 'password7'),
(8, 'user8', 'password8'),
(9, 'user9', 'password9');

SELECT * FROM users;