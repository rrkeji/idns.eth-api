# IDNS.ETH Sqlite

> 实现sqlite的本地数据与IPFS中的数据进行同步
> 进行数据的版本控制
> 重新定义接口

使用版本控制的机制进行同步
## 数据版本的比较

### 获取数据库的当前schema

获取所有的表
```
-- type name tbl_name rootpage sql
SELECT * FROM sqlite_master WHERE type='table' ORDER BY name;
-- 
SELECT * FROM sqlite_master WHERE 1 = 1 ORDER BY name;
```
获取表的所有的列
```
-- cid name type notnull dflt_value pk
PRAGMA  table_info("person")
```


### 表内数据版本
需要备份的表中增加列
> IPFS中的CID
> 一个随机整数字段, 随机整数的意义是当回填CID的时候， 判断下随机整数， 如果不一致，说明在读取和备份的期间数据已经变化。




```
DROP TABLE person;
DROP TABLE trigger__cid_update_person;
DROP TABLE trigger__cid_insert_person;

CREATE TABLE person (
    id    INTEGER PRIMARY KEY,
    name  TEXT NOT NULL,
    data  BLOB,
    _cid  TEXT DEFAULT '',
    _cn INTEGER DEFAULT 0
);
CREATE  TRIGGER trigger__cid_update_person  AFTER UPDATE ON person
BEGIN
    update person set _cid = "", _cn =(RANDOM()+9223372036854775808)/2.0/9223372036854775808 *1000000   where id = new.id;
END;
CREATE  TRIGGER trigger__cid_insert_person  AFTER INSERT ON person
BEGIN
    update person set _cid = "", _cn =(RANDOM()+9223372036854775808)/2.0/9223372036854775808 *1000000   where id = new.id;
END;
```

一个应用程序一个数据库，数据库的路径使用应用的appkey作为目录的一部分