CREATE TABLE IF NOT EXISTS files(
    id    INTEGER PRIMARY KEY,
    category   TEXT DEFAULT '',
    parent_id  INTEGER DEFAULT 0,
    file_name  TEXT NOT NULL,
    file_hash  TEXT NOT NULL,
    file_size  INTEGER DEFAULT 0,
    file_type  TEXT NOT NULL,
    is_dir  INTEGER DEFAULT 0,
    status  INTEGER DEFAULT 1,
    _cid  TEXT DEFAULT '',
    _cn INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS devices(
    id    INTEGER PRIMARY KEY,
    owner_id    INTEGER DEFAULT 0,
    node_id     TEXT NOT NULL,
    category    TEXT NOT NULL,
    name        TEXT NOT NULL,
    address     TEXT NOT NULL,
    icon_url     TEXT NOT NULL,
    remote_role    INTEGER DEFAULT 0,
    device_type  INTEGER DEFAULT 0,
    is_dir  INTEGER DEFAULT 0,
    status  INTEGER DEFAULT 1,
    _cid  TEXT DEFAULT '',
    _cn INTEGER DEFAULT 0
);