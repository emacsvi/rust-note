## ubuntu18.04 安装mysql
```bash
sudo apt update
sudo apt install mysql-server
```
** 配置 **

```bash
sudo mysql_secure_installation
```

```text
xjgw@cd-xx-009:~/sealwork/log$ sudo mysql_secure_installation

Securing the MySQL server deployment.

Connecting to MySQL using a blank password.

VALIDATE PASSWORD PLUGIN can be used to test passwords
and improve security. It checks the strength of password
and allows the users to set only those passwords which are
secure enough. Would you like to setup VALIDATE PASSWORD plugin?

Press y|Y for Yes, any other key for No: y

There are three levels of password validation policy:

LOW    Length >= 8
MEDIUM Length >= 8, numeric, mixed case, and special characters
STRONG Length >= 8, numeric, mixed case, special characters and dictionary                  file

Please enter 0 = LOW, 1 = MEDIUM and 2 = STRONG: 1
Please set the password for root here.

New password:

Re-enter new password:

Estimated strength of the password: 100
Do you wish to continue with the password provided?(Press y|Y for Yes, any other key for No) : y
By default, a MySQL installation has an anonymous user,
allowing anyone to log into MySQL without having to have
a user account created for them. This is intended only for
testing, and to make the installation go a bit smoother.
You should remove them before moving into a production
environment.

Remove anonymous users? (Press y|Y for Yes, any other key for No) : y
Success.


Normally, root should only be allowed to connect from
'localhost'. This ensures that someone cannot guess at
the root password from the network.

Disallow root login remotely? (Press y|Y for Yes, any other key for No) : n

 ... skipping.
By default, MySQL comes with a database named 'test' that
anyone can access. This is also intended only for testing,
and should be removed before moving into a production
environment.


Remove test database and access to it? (Press y|Y for Yes, any other key for No) : y
 - Dropping test database...
Success.

 - Removing privileges on test database...
Success.

Reloading the privilege tables will ensure that all changes
made so far will take effect immediately.

Reload privilege tables now? (Press y|Y for Yes, any other key for No) : y
Success.

All done!
```
**查看运行状态**

```bash
systemctl status mysql.service
```

## 登录到mysql中
**一定要加sudo**
```bash
sudo mysql -uroot -p
```
** 设置权限**:
```bash
GRANT ALL PRIVILEGES ON *.* TO 'root'@'%' IDENTIFIED BY '您的数据库密码' WITH GRANT OPTION;
flush privileges;
```

** 设置远程登录端口**
```bash
sudo vim /etc/mysql/mysql.conf.d/mysqld.cnf
# 修改绑定地址为 0.0.0.0。原来默认绑定 127.0.0.1 注释掉。
bind-address = 0.0.0.0
# bind-address = 127.0.0.1
```

** 重启生效**
```bash
sudo systemctl restart mysql.service
```

## 参考

- [遇到的坑](https://wangxin1248.github.io/linux/2018/07/ubuntu18.04-install-mysqlserver.html)
