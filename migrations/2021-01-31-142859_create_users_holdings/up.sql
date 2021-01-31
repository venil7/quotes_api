CREATE TABLE users (
  id SERIAL NOT NULL,
  email VARCHAR(50) NOT NULL UNIQUE,
  name VARCHAR(50) NOT NULL,
  password VARCHAR(50) NOT NULL,
  verified BOOLEAN NOT NULL DEFAULT 'f',
  created TIMESTAMP NOT NULL DEFAULT NOW(),
  
  PRIMARY KEY(id)
);

CREATE TABLE holdings (
  id SERIAL NOT NULL,
  user_id SERIAL NOT NULL,
  ticker VARCHAR(50) NOT NULL,
  amount double precision NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT NOW(),

  PRIMARY KEY(id),
  FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE OR REPLACE VIEW users_holdings AS 
(SELECT user_id, ticker, SUM(amount) as amount
FROM holdings h
GROUP BY h.ticker, h.user_id);