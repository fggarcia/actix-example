CREATE KEYSPACE IF NOT EXISTS actix
    WITH REPLICATION = {
      'class': 'SimpleStrategy',
      'replication_factor': 1
    };

CREATE TABLE IF NOT EXISTS actix.actix (
    name text,
    reference text,
    num int,
    num2 int,
    PRIMARY KEY(name, reference)
);