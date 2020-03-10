## mysql权限管理

### MySQL权限经验原则：

权限控制主要是出于安全因素，因此需要遵循一下几个经验原则：

- 只授予能满足需要的最小权限，防止用户干坏事。比如用户只是需要查询，那就只给select权限就可以了，不要给用户赋予update、insert或者delete权限。
- 创建用户的时候限制用户的登录主机，一般是限制成指定IP或者内网IP段。
- 初始化数据库的时候删除没有密码的用户。安装完数据库的时候会自动创建一些用户，这些用户默认没有密码。
- 为每个用户设置满足密码复杂度的密码。
- 定期清理不需要的用户。回收权限或者删除用户。



### MySQL权限实战：

#### GRANT命令使用说明：

先来看一个例子，创建一个只允许从本地登录的超级用户`jack`，并允许将权限赋予别的用户，密码为：`jack`.

```mysql
mysql> grant all privileges on *.* to jack@'localhost' identified by "jack" with grant option;
Query OK, 0 rows affected (0.01 sec)
```
GRANT命令说明：
- `ALL PRIVILEGES`是表示所有权限，你也可以使用`select`、`update`等权限。
- `ON` 用来指定权限针对哪些库和表。
- `*.*` 中前面的`*`号用来指定数据库名，后面的`*号`用来指定表名。
- `TO` 表示将权限赋予某个用户。
- `jack@'localhost'` 表示`jack`用户，`@`后面接限制的主机，可以是`IP`、`IP段`、`域名`以及`%`，`%`表示**任何地方**。注意：这里`%`有的版本不包括本地，以前碰到过给某个用户设置了`%`允许任何地方登录，但是在本地登录不了，这个和版本有关系，遇到这个问题再加一个`localhost`的用户就可以了。
- `IDENTIFIED BY`指定用户的登录密码。
- `WITH GRANT OPTION`这个选项表示该用户可以将自己拥有的权限授权给别人。注意：经常有人在创建操作用户的时候不指定`WITH GRANT OPTION`选项导致后来该用户不能使用`GRANT`命令创建用户或者给其它用户授权。

备注：可以使用`GRANT`重复给用户添加权限，权限叠加，比如你先给用户添加一个`select`权限，然后又给用户添加一个`insert`权限，那么该用户就同时拥有了`select`和`insert`权限。

#### 刷新权限

使用这个命令使权限生效，尤其是你对那些权限表user、db、host等做了update或者delete更新的时候。以前遇到过使用grant后权限没有更新的情况，只要对权限做了更改就使用FLUSH PRIVILEGES命令来刷新权限。
```mysql
mysql> flush privileges;
Query OK, 0 rows affected (0.01 sec)
```
#### 查看权限

查看当前用户的权限：
```mysql
mysql> show grants;
+---------------------------------------------------------------------+
| Grants for root@localhost                                           |
+---------------------------------------------------------------------+
| GRANT ALL PRIVILEGES ON *.* TO 'root'@'localhost' WITH GRANT OPTION |
| GRANT PROXY ON ''@'' TO 'root'@'localhost' WITH GRANT OPTION        |
+---------------------------------------------------------------------+
2 rows in set (0.00 sec)
```

查看某个用户的权限：
```mysql
mysql> show grants for 'jack'@'%';
+-----------------------------------------------------------------------------------------------------+
| Grants for jack@%                                                                                   |
+-----------------------------------------------------------------------------------------------------+
| GRANT USAGE ON *.* TO 'jack'@'%' IDENTIFIED BY PASSWORD '*9BCDC990E611B8D852EFAF1E3919AB6AC8C8A9F0' |
+-----------------------------------------------------------------------------------------------------+
1 row in set (0.00 sec)
```

#### 回收权限
```mysql
mysql> revoke delete on *.* from 'jack'@'localhost';
Query OK, 0 rows affected (0.01 sec)
```

#### 删除用户
```mysql
mysql> select host,user,password from user;
+-----------+------+-------------------------------------------+
| host      | user | password                                  |
+-----------+------+-------------------------------------------+
| localhost | root |                                           |
| rhel5.4   | root |                                           |
| 127.0.0.1 | root |                                           |
| ::1       | root |                                           |
| localhost |      |                                           |
| rhel5.4   |      |                                           |
| localhost | jack | *9BCDC990E611B8D852EFAF1E3919AB6AC8C8A9F0 |
+-----------+------+-------------------------------------------+
7 rows in set (0.00 sec)

mysql> drop user 'jack'@'localhost';
Query OK, 0 rows affected (0.01 sec)
```
#### 对账户重命名
```mysql
mysql> rename user 'jack'@'%' to 'jim'@'%';
Query OK, 0 rows affected (0.00 sec)
```
#### 修改密码
用set password命令
```mysql
mysql> SET PASSWORD FOR 'root'@'localhost' = PASSWORD('123456');
Query OK, 0 rows affected (0.00 sec)
```
用mysqladmin
```bash
[root@rhel5 ~]# mysqladmin -uroot -p123456 password 1234abcd
```
备注：
格式：`mysqladmin -u用户名 -p旧密码 password 新密码`

用update直接编辑user表
```mysql
mysql> use mysql
Reading table information for completion of table and column names
You can turn off this feature to get a quicker startup with -A

Database changed
mysql> update user set PASSWORD = PASSWORD('1234abcd') where user = 'root';
Query OK, 1 row affected (0.01 sec)
Rows matched: 1  Changed: 1  Warnings: 0

mysql> flush privileges;
Query OK, 0 rows affected (0.00 sec)
```
在丢失root密码的时候：
```mysql
[root@rhel5 ~]# mysqld_safe --skip-grant-tables &
[1] 15953
[root@rhel5 ~]# 130911 09:35:33 mysqld_safe Logging to '/mysql/mysql5.5/data/rhel5.4.err'.
130911 09:35:33 mysqld_safe Starting mysqld daemon with databases from /mysql/mysql5.5/data

[root@rhel5 ~]# mysql -u root
Welcome to the MySQL monitor.  Commands end with ; or \g.
Your MySQL connection id is 2
Server version: 5.5.22 Source distribution

Copyright (c) 2000, 2011, Oracle and/or its affiliates. All rights reserved.

Oracle is a registered trademark of Oracle Corporation and/or its
affiliates. Other names may be trademarks of their respective
owners.

Type 'help;' or '\h' for help. Type '\c' to clear the current input statement.

mysql> \s
--------------
mysql  Ver 14.14 Distrib 5.5.22, for Linux (i686) using  EditLine wrapper

Connection id:        2
Current database:    
Current user:        root@
SSL:            Not in use
Current pager:        stdout
Using outfile:        ''
Using delimiter:    ;
Server version:        5.5.22 Source distribution
Protocol version:    10
Connection:        Localhost via UNIX socket
Server characterset:    utf8
Db     characterset:    utf8
Client characterset:    utf8
Conn.  characterset:    utf8
UNIX socket:        /tmp/mysql.sock
Uptime:            36 sec

Threads: 1  Questions: 5  Slow queries: 0  Opens: 23  Flush tables: 1  Open tables: 18  Queries per second avg: 0.138
--------------

mysql> use mysql
Reading table information for completion of table and column names
You can turn off this feature to get a quicker startup with -A

Database changed
mysql> update user set password = PASSWORD('123456') where user = 'root';
Query OK, 1 row affected (0.00 sec)
Rows matched: 1  Changed: 1  Warnings: 0

mysql> flush privileges;
Query OK, 0 rows affected (0.00 sec)
```

## 参考

- [权限管理详解](https://www.cnblogs.com/richardzhu/p/3318595.html)
