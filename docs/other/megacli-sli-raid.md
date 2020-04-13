## megacli 安装
`ubuntu18.04`下面做的测试
```bash
# 末尾添加源
sudo vim /etc/apt/sources.list

deb http://hwraid.le-vert.net/ubuntu precise main

# 更新源
sudo apt-get update  
# 注意：会提示一些警告可忽略，但如果提示 GPG 错误，需要执行如下命令添加证书：
sudo wget -O - http://hwraid.le-vert.net/debian/hwraid.le-vert.net.gpg.key | sudo apt-key add -
sudo apt-get update
# 安装MegaCLI
sudo apt-get install -y megacli megactl megaraid-status

# 显示Raid卡型号，Raid设置，Disk相关信息
sudo megacli -cfgdsply -aALL
```

## megacli raid队列设置
一些常用命令
```bash
# 显示Raid卡型号，Raid设置，Disk相关信息
sudo megacli -cfgdsply -aALL

# 查看RAID状态
sudo megacli -cfgdsply -aALL
# 注意：State为Optimal，表示正常。更换损坏的硬盘后，机器会自动同步raid数据，此时状态为Degraded降级状态，不需要别的操作，只需等待几个小时待完全同步数据，恢复raid状态。
sudo megacli -cfgdsply -aALL |grep "State"
# 查看物理磁盘信息
sudo megacli -PDList -aALL

# 检测磁盘 ID 注意, 该ID 值用于标注磁盘 Enclosure Device ID: 252
sudo megacli -PDlist -aALL | grep "ID"  | uniq
# 查看当前raid数量
sudo megacli -cfgdsply -aALL |grep "Number of DISK GROUPS:"
# 查看Raid卡信息
sudo megacli -cfgdsply –aALL  | more
# 其他物理信息
sudo megacli -PDList -aALL | more
# 当前raid信息
sudo megacli -LDInfo -LALL –aAll
# raid 控制器个数
sudo megacli  -adpCount
# raid 控制器时间
sudo megacli -AdpGetTime –aALL

# https://blog.51cto.com/hmtk520/2140657
# 创建raid5 创建一个raid5阵列，由物理盘1,2,3,4,5构成，该阵列的热备盘是物理盘6
xj@xjgw3970:~$ sudo megacli -CfgLdAdd -r5 [252:0,252:1,252:2,252:3,252:4] WB Direct -Hsp[252:5] –a0

Adapter 0: Created VD 0
Adapter: 0: Set Physical Drive at EnclId-252 SlotId-5 as Hot Spare Success.

Adapter 0: Configured the Adapter!!

Exit Code: 0x00

# 如果不指定热备
sudo megacli -CfgLdAdd -r5 [252:0,252:1,252:2,252:3,252:4] WB Direct –a0

# 创建一个raid10阵列，由物理盘2,3和4,5分别做raid1，在将两组raid1做raid0
sudo megacli –CfgSpanAdd –r10 –Array0[1:2,1:3] –Array1[1:4,1:5] WB Direct -a0


# 创建分区，格式化，mount
sudo fdisk /dev/sda
g n t 31 w p
sudo mkfs.ext4 /dev/sda1
sudo mount /dev/sda1 /r5
# 写fstab
/dev/sda1 /r5 ext4 rw,noatime 0 0

# 删除radi5
sudo megacli -CfgLdDel -L0 -a0
```

## 实例

### 线上新设备做raid5
在线上的设备先用`8`块盘做`raid5`
```bash
# 1. 安装
sudo echo "deb http://hwraid.le-vert.net/ubuntu precise main" | sudo tee -a /etc/apt/sources.list
sudo apt-get update
sudo wget -O - http://hwraid.le-vert.net/debian/hwraid.le-vert.net.gpg.key | sudo apt-key add -
sudo apt-get update
sudo apt-get install -y megacli megactl megaraid-status

# 2. 做raid5
sudo megacli -PDlist -aALL | grep "ID"  | uniq
# 上面这命令查看到的结果是26
sudo megacli -CfgLdAdd -r5 [26:1,26:2,26:3,26:4,26:5,26:6,26:7,26:8] WB Direct –a0
# 3. 格式化该磁盘
sudo fdisk /dev/sda
sudo mkfs.ext4 /dev/sda1

# 4. mount 分区
sudo mkdir -p /data/filecoin
sudo mount /dev/sda1 /data/filecoin
# 5. 开机自动挂载
sudo echo "/dev/sda1 /data/filecoin ext4 rw,noatime 0 0" | sudo tee -a /etc/fstab
# 6. 修改权限
sudo chown xjgw:root /data -Rf
```

### 后期增加两个盘，该两块盘做raid0做分布式文件系统

```bash
sudo megacli -PDList -aALL
sudo megacli -PDlist -aALL | grep "ID"  | uniq
sudo megacli -CfgLdAdd -r0 [26:15,26:16] WB Direct –a0

# 做的时候发现报错了，这可能是因为我这几块磁盘是从其他机器上弄下来，里面有Raid的旧数据
xjgw@cd-xx-012:~$ sudo megacli -CfgLdAdd -r0 [26:15,26:16] WB Direct –a0

The specified physical disk does not have the appropriate attributes to complete
the requested command.

Exit Code: 0x26

# 为了修复这个问题，重新扫描一下
xjgw@cd-xx-012:~$ sudo megacli -cfgforeign -scan -a0

There are 1 foreign configuration(s) on controller 0.

Exit Code: 0x00
# 清除扫描的记录
xjgw@cd-xx-012:~$ sudo megacli -cfgforeign -clear -a0

Foreign configuration 0 is cleared on controller 0.

Exit Code: 0x00
# 再次扫描
xjgw@cd-xx-012:~$ sudo megacli -cfgforeign -scan -a0

There is no foreign configuration on controller 0.

Exit Code: 0x00
# 再做一次成功了
xjgw@cd-xx-012:~$ sudo megacli -CfgLdAdd -r0 [26:15,26:16] WB Direct –a0

Adapter 0: Created VD 1

Adapter 0: Configured the Adapter!!

Exit Code: 0x00
# fdisk 看一下结果成功了
xjgw@cd-xx-012:~$ sudo fdisk -l

Disk /dev/sdb: 21.4 TiB, 23511724720128 bytes, 45921337344 sectors
Units: sectors of 1 * 512 = 512 bytes
Sector size (logical/physical): 512 bytes / 4096 bytes
I/O size (minimum/optimal): 4096 bytes / 4096 bytes
```

## 参考链接

- [创建raid](https://blog.51cto.com/hmtk520/2140657)
- [megacli](https://blog.csdn.net/oaa608868/article/details/53523960)
- [megacli](https://www.aikaiyuan.com/11557.html)
- [换盘raid报错](https://forum.huawei.com/enterprise/zh/thread-430333.html)
- [raid web ui](https://www.yeboyzq.com/yingjianweihu/942.html)
