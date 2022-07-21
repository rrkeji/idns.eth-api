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
> 一个随机整数字段