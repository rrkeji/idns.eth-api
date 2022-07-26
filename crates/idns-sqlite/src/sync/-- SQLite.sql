-- SQLite
-- SQLite

        DROP TABLE person;
        DROP TRIGGER trigger__cid_update_person;
        DROP TRIGGER trigger__cid_insert_person;

        CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB,
            _cid  TEXT DEFAULT '',
            _cn INTEGER DEFAULT 0
        );
        CREATE  TRIGGER trigger__cid_update_person  AFTER UPDATE ON person for each row
        BEGIN
            update person set _cid = "", _cn =ABS(RANDOM() % 100000000)   where id = new.id and new._cid = old._cid and new._cid != '';
        END;
        CREATE  TRIGGER trigger__cid_insert_person  AFTER INSERT ON person
        BEGIN
            update person set _cid = "xxxxx", _cn =ABS(RANDOM() % 100000000)   where id = new.id;
        END;

SELECT * FROM sqlite_master WHERE 1 = 1 ORDER BY name;

INSERT INTO person (name, data) VALUES ("ss1111", "sssss111");

update person set name = "ssccccc333" where id = 2;
update person set _cid = "222222220002" where id = 1 and _cn = 74042263;
SELECT * FROM person ORDER BY id ;
SELECT * FROM idns_table_version ORDER BY id ;

SELECT COUNT(*) FROM sqlite_master where type ='table' and name ='idns_table_version';

CREATE TABLE idns_table_version (
    id INTEGER PRIMARY KEY,
    table_id INTEGER,
    _cid  TEXT,
    _cn INTEGER
);


PRAGMA  table_info("person") 

ALTER TABLE person ADD column tt TEXT;


SELECT ABS(RANDOM() % 100000000);