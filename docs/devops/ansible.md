## Ansible环境

`Ansible`配置文件是以`.ini`格式存储配置数据的，在`Ansible`中，几乎所有的配置项都可以通过`Ansible`的`playbook`或环境变量来重新赋值。在运行`Ansible`命令式，命令将会按照预先设定的顺序查找配置文件，如下所示：

1. `ANSIBLE_CONFIG`: 首先，Ansible命令会检查环境变量，及这个环境变量指向的配置文件。
2. `./ansible.cfg`: 其次，将会检查当前目录下的ansible.cfg配置文件
3. `~/.ansible.cfg`: 再次，将会检查当前用户home目录下的`.ansible.cfg`配置文件
4. `/etc/ansible/ansible.cfg`: 最后，将会检查再用软件包管理工具安装Ansible时自动产生的配置文件。


## ansible.cfg常用配置参数

```ini
inventory = ~/ansible_hosts #这个参数表示主机清单inventory文件的位置
forks = 5 #并发连接数，默认为5
sudo_user = root #设置默认执行命令的用户
remote_port = 22 #指定连接被管节点的管理端口，默认为22端口，建议修改，能够更加安全
host_key_checking = False #设置是否检查SSH主机的密钥，值为True/False。关闭后第一次连接不会提示配置实例
timeout = 60 #设置SSH连接的超时时间，单位为秒
log_path = /var/log/ansible.log #指定一个存储ansible日志的文件（默认不记录日志）
```

```ini
[defaults]                                    #常规连接类配置
inventory      = /etc/ansible/hosts           #定义主机信息配置文件,默认路径
forks          = 5                            #定义默认开启的并发数
poll_interval  = 15                           #定义轮询时间间隔
sudo_user      = root                         #定义默认sudo用户
#ask_sudo_pass  = True                        #是否需要sudo密码
#ask_pass      = True                         #是否需要密码
remote_port    = 51899                        #定义ssh端口
roles_path    = /etc/ansible/roles            #默认的role规则路径
host_key_checking = False                     #首次连接检查key认证(record_host_keys协作使用)
timeout = 10                                  #默认超时时间
log_path = /var/log/ansible.log               #默认日志路径
deprecation_warnings = Flase                  #该参数在使用playbooks操作的时候sudo权限不会进行提示
nocows = 1                                    #关闭cowsay功能，如果需要启用的话必须首先安装了cowsay才可以
nocolor = 0                                   #开启颜色，下面的colors设置相关的颜色
[privilege_escalation]                        #权限管理
#become    = True                             #是否sudo
#become_method  = True                        #sudo方式
#become_user    = root                        #sudo后变成root用户
#become_ask_pass= False                       #sudo后是否验证密码  

[paramiko_connection]                         #连接配置
record_host_keys=Flase                        #记录新主机的key.在host_key_checking 被禁用时候,设置为False时,性能将会提升

[ssh_connection]                              #ssh协议连接配置
ssh_args = -C -o ControlMaster=auto -o ControlPersist=1800s    #传递一组选项给Ansible,而不是使用以前的默认值.ControlPersist的值提高会提高性能，30分钟会比较合适

[accelerate]                                  #加速配置
#accelerate_port = 5099                       #加速连接端口
#accelerate_timeout = 30                      #命令执行超时时间，单位秒
#accelerate_connect_timeout = 5.0             #连接超时时间
#accelerate_daemon_timeout = 30               #上一个活动连接的时间，单位分钟

[selinux]                                     #该参数一般不会使用
[colors]                                      #配置常用的颜色
highlight = white
verbose = blue
warn = bright purple
error = red
ok = green
changed = yellow
diff_add = green
diff_remove = red
diff_lines = cyan
pipelining=true
```

## 主机与组

```ini
[webservers]
foo.example.com
bar.example.com

[dbservers]
one.example.com
two.example.com
three.example.com
```

设置别名：
```ini
jumper ansible_ssh_port=5555 ansible_ssh_host=192.168.1.50
```

一组相似的hostname可简写如下：
```ini
[webservers]
www[01:50].example.com
```
数字的简写模式中,01:50 也可写为 1:50,意义相同.你还可以定义字母范围的简写模式:
```ini
[databases]
db-[a:f].example.com
```
对于每一个host,你还可以选择连接类型和连接用户名：
```ini
[targets]
localhost ansible_connection=local
other1.example.com ansible_connection=ssh ansible_ssh_user=mpdehaan
other2.example.com ansible_connection=ssh ansible_ssh_user=mdehaan    
```

## 主机变量
分配变量给主机的方式:
```ini
[atlanta]
host1 http_port=80 maxRequestsPerChild=808
host2 http_port=303 maxRequestsPerChild=909
```

## 组的变量
也可以定义属于整个组的变量:
```ini
[atlanta]
host1
host2

[atlanta:vars]
ntp_server=ntp.atlanta.example.com
proxy=proxy.atlanta.example.com
```
