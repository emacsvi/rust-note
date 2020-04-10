## nfs配置

1. 在被挂载机器上面安装`nfs server`端应用
```bash
sudo apt-get update
sudo apt-get install -y nfs-kernel-server
```

2. 配置`nfs`目录和读写权限相关配置
```bash
sudo mkdir /lotus
sudo chown xjgw:xjgw /lotus
sudo vi /etc/exports
# 将下列内容添加进最后一行：
/lotus *(rw,sync,no_root_squash,no_subtree_check)
```


3. 重启服务
```bash
sudo /etc/init.d/rpcbind restart
sudo /etc/init.d/nfs-kernel-server restart
```

4. 在其他服务器进行挂载验证
```bash
sudo apt-get install -y nfs-common
showmount -e 10.0.20.12
# 创建一个挂载过来的目录 cdxx12fs
sudo mkdir /mnt/cdxx12fs
sudo chown xjgw:xjgw /mnt/cdxx12fs
sudo mount -o soft,nolock,timeo=30,retry=2  10.0.20.12:/lotus /mnt/cdxx12fs

# umount
sudo umount /mnt/cdxx12fs
```

## 参数说明

在这里，除了`no_root_squash`之外，我们对两个目录使用相同的配置选项。 让我们来看看每个选项的含义：

- rw ：此选项为客户端计算机提供对卷的读写访问权限。
- sync ：此选项强制NFS在回复之前将更改写入磁盘。 这导致更稳定和一致的环境，因为回复反映了远程卷的实际状态。 但是，它也会降低文件操作的速度。
- no_subtree_check ：此选项可防止子树检查，这是一个主机必须检查文件是否在每个请求的导出树中实际可用的过程。 在客户端打开文件时重命名文件时，这可能会导致许多问题。 几乎在所有情况下，最好禁用子树检查。
- no_root_squash ：默认情况下，NFS将来自root用户的请求远程转换为服务器上的非特权用户。 这旨在作为安全功能，以防止客户端上的root帐户以root身份使用主机的文件系统。 no_root_squash禁用某些共享的此行为。

## 参考

- [ubuntu安装nfs server](https://www.jianshu.com/p/5314f90330a6)
