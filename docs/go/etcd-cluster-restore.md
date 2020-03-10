## etcd集群数据迁移

首先在目标集群上停止使用集群的相关服务。再进行备份：

**备份**:
```bash
# 备份
export ETCDCTL_API=3
echo $ETCDCTL_API
etcdctl --endpoints 127.0.0.1:2379 snapshot save snapshot.db
# ETCDCTL_API=3 etcdctl --endpoints $ENDPOINT snapshot save snapshot.db
```

再找三台服务器，ip地址分别是`182.140.213.131`, `132`, `133` 分别执行如下操作:
```bash
# 恢复
# 在每一个节点都分别执行如下操作：
export ETCDCTL_API=3
echo $ETCDCTL_API
# 182.140.213.131
etcdctl snapshot restore snapshot.db --name n0 --initial-cluster n0=http://182.140.213.131:2380,n1=http://182.140.213.132:2380,n2=http://182.140.213.133:2380 --initial-cluster-token xjgw-miner-token --initial-advertise-peer-urls http://182.140.213.131:2380 --data-dir=/home/xjgw/docker/db

# 182.140.213.132
etcdctl snapshot restore snapshot.db --name n1 --initial-cluster n0=http://182.140.213.131:2380,n1=http://182.140.213.132:2380,n2=http://182.140.213.133:2380 --initial-cluster-token xjgw-miner-token --initial-advertise-peer-urls http://182.140.213.132:2380 --data-dir=/home/xjgw/docker/db

# 182.140.213.133
etcdctl snapshot restore snapshot.db --name n2 --initial-cluster n0=http://182.140.213.131:2380,n1=http://182.140.213.132:2380,n2=http://182.140.213.133:2380 --initial-cluster-token xjgw-miner-token --initial-advertise-peer-urls http://182.140.213.133:2380 --data-dir=/home/xjgw/docker/db
```

上面是初始化集群信息，会生成db数据库相关信息。然后再依次启动三个docker容器运行三个节点：
```bash
# 启动
docker rm -f n0

# 182.140.213.131
docker run -d --name n0 --user $(id -u):$(id -g) --restart=always --net=host -v /home/xjgw/docker/db:/etcd-data:rw quay.io/coreos/etcd:latest /usr/local/bin/etcd -name n0 --data-dir /etcd-data --initial-advertise-peer-urls http://182.140.213.131:2380 --listen-peer-urls http://0.0.0.0:2380 --advertise-client-urls http://182.140.213.131:2379 --listen-client-urls http://0.0.0.0:2379 --initial-cluster n0=http://182.140.213.131:2380,n1=http://182.140.213.132:2380,n2=http://182.140.213.133:2380 --initial-cluster-state existing --initial-cluster-token xjgw-miner-token

# 182.140.213.132
docker run -d --name n1 --user $(id -u):$(id -g) --restart=always --net=host -v /home/xjgw/docker/db:/etcd-data:rw quay.io/coreos/etcd:latest /usr/local/bin/etcd -name n1 --data-dir /etcd-data --initial-advertise-peer-urls http://182.140.213.132:2380 --listen-peer-urls http://0.0.0.0:2380 --advertise-client-urls http://182.140.213.132:2379 --listen-client-urls http://0.0.0.0:2379 --initial-cluster n0=http://182.140.213.131:2380,n1=http://182.140.213.132:2380,n2=http://182.140.213.133:2380 --initial-cluster-state existing --initial-cluster-token xjgw-miner-token

# 182.140.213.133
docker run -d --name n2 --user $(id -u):$(id -g) --restart=always --net=host -v /home/xjgw/docker/db:/etcd-data:rw quay.io/coreos/etcd:latest /usr/local/bin/etcd -name n2 --data-dir /etcd-data --initial-advertise-peer-urls http://182.140.213.133:2380 --listen-peer-urls http://0.0.0.0:2380 --advertise-client-urls http://182.140.213.133:2379 --listen-client-urls http://0.0.0.0:2379 --initial-cluster n0=http://182.140.213.131:2380,n1=http://182.140.213.132:2380,n2=http://182.140.213.133:2380 --initial-cluster-state existing --initial-cluster-token xjgw-miner-token
```

## 参考

- [v2 数据迁移](https://github.com/penglongli/blog/issues/38)
- [灾难恢复](https://skyao.gitbooks.io/learning-etcd3/content/documentation/op-guide/recovery.html)
- [recovery](https://github.com/etcd-io/etcd/blob/master/Documentation/op-guide/recovery.md)
