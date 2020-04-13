## testnet3合并代码修改的地方


### 下载分支代码：
```bash
git clone https://github.com/filecoin-project/lotus.git
git branch -r | grep testnet
# 创建一个dev分支于远程分支对应
git checkout -b dev origin/testnet/3
```

查看依赖库，将go-sectorbuilder, go-fil-markets库复制到本地进行修改
```bash
go list -m -json all
chmod -R 755 *

```

在go.mod文件末尾添加，并且进行`make build`编译
```go
replace github.com/filecoin-project/go-fil-markets => ../go-fil-markets

replace github.com/filecoin-project/go-sectorbuilder => ../go-sectorbuilder
```

### 修改源码的地方

#### 获取id号
`storedcounter.go`文件里面的Next()函数进行修改，获取**sector id**信息
由于信息本身保存到本地数据库里面，这里需要用集群来替换保存方式。

以前在builder.go里面的modules.SectorBuilder来构建的，现在在builder.go里面的modules.SectorIDCounter里面进行依赖注入。

```go
func (sc *StoredCounter) Next() (uint64, error) {
	var (
		id uint64
		err error
	)
	
	_, err = sc.ds.Has(sc.name)
	if err != nil {
		return 0, err
	}

	if cluster.MinerFlag != nil {
		id, err = cluster.MinerFlag.AcquireSectorId()
		if err != nil || id <= 0 {
			return 0, errors.Wrapf(err, "acquired sector id error.")
		}

	} else {
		return 0, errors.New("cluster is not present.")
	}
	
	buf := make([]byte, binary.MaxVarintLen64)
	size := binary.PutUvarint(buf, id)

	return id, sc.ds.Put(sc.name, buf[:size])
}
```
#### 查找文件路径

客户端也就是在lotus-storage-miner里面会申请id的时候，会向服务器上报自己的路径并且由服务器分配一个id号。

**客户端**
- Repo是Base去掉`.`之后的目录, 比如/data/filecoin则是filecoin; /home/xjgw/.lotusstorage则是lotusstorage;
- ip是本地的ip地址。
- hostname是系统的hostname信息

```go
func (m *IsMiner) AcquireSectorId() (uint64, error) {
	m.RLock()
	defer m.RUnlock()
	var err error
	req := AgentReportRequest{
		Hostname:     m.Hostname,
		Repo:         m.RepoStorage,
		ActorId:      m.ActorId,
		IP:           m.localIp,
		AgentVersion: "2.0.2",
	}
	var resp AgentSectorIdResponse
	err = m.cli.Call("Agent.GetSectorId", req, &resp)
	if err != nil || resp.SectorId == 0 {
		err = errors.New("Agent.GetSectorId failed.")
		return 0, err
	}
	err = m.createSymLink(resp.SectorId)
	if err != nil {
		log.Println("createSymLink error:", err)
	}
	return resp.SectorId, nil
}
```


[/xjgw/sectors/t01021/1576]=xjgw3960-r3960


#### miner

修改`node/modules/storageminer.go`文件里面的SetupBlockProducer()开关其是否能挖矿

```go
func SetupBlockProducer(lc fx.Lifecycle, ds dtypes.MetadataDS, api lapi.FullNode, epp gen.ElectionPoStProver) (*miner.Miner, error) {
	minerAddr, err := minerAddrFromDS(ds)
	if err != nil {
		return nil, err
	}

	m := miner.NewMiner(api, epp)

	// added by liwei
	if cluster.MinerFlag != nil && cluster.MinerFlag.IsSeal {
		log.Info("not miner and return")
		return m, nil
	} else {
		log.Info("do miner")
	}

	lc.Append(fx.Hook{
		OnStart: func(ctx context.Context) error {
			if err := m.Register(minerAddr); err != nil {
				return err
			}
			return nil
		},
		OnStop: func(ctx context.Context) error {
			return m.Unregister(ctx, minerAddr)
		},
	})

	return m, nil
}
```

#### fpost

修改文件`storage/fpost_sched.go`里面的Run()函数：
```go
func (s *FPoStScheduler) Run(ctx context.Context) {
	// added by liwei
	check := false
	if cluster.MinerFlag == nil || (cluster.MinerFlag != nil && !cluster.MinerFlag.IsSeal) {
		check = true
	}
	if !check {
		log.Info("not do fpost action.")
		return
	}
	log.Info("do fpost")

	notifs, err := s.api.ChainNotify(ctx)
	if err != nil {
		return
	}

	current := <-notifs
	if len(current) != 1 {
		panic("expected first notif to have len = 1")
	}
	if current[0].Type != store.HCCurrent {
		panic("expected first notif to tell current ts")
	}

	if err := s.update(ctx, current[0].Val); err != nil {
		panic(err)
	}

	defer s.abortActivePoSt()
  // ....
```

#### go-sectorbuilder
在go-sectorbuilder里面进行修改路径：
`go-sectorbuilder/fs/basic.go`的AcquireSector()函数：

```go
	// added by liwei
	var (
		unseadPath string
		sealedPath string
		cachePath string
		root string
	)
	root = b.Root
	if cluster.G_setctorInfo != nil {
		if p, ok := cluster.G_setctorInfo.GetDirPath(uint64(id.Number)); ok {
			root = p
			log.Info("success find root:", root)
		} else {
			log.Info("error failed to find path in cluster. so we will use traditional style to find.")
		}
	}

	unseadPath =  filepath.Join(root, sectorbuilder.FTUnsealed.String(), fmt.Sprintf("s-t0%d-%d", id.Miner, id.Number))
	sealedPath = filepath.Join(root, sectorbuilder.FTSealed.String(), fmt.Sprintf("s-t0%d-%d", id.Miner, id.Number))
	cachePath = filepath.Join(root, sectorbuilder.FTCache.String(), fmt.Sprintf("s-t0%d-%d", id.Miner, id.Number))

```
还有个地方：
`index.go`里面需要大改：

## meta数据的修改

元数据在生成SectorId时生成的，我们需要用集群对元数据进行同步，同步的方式：
- 在生成sectorid之后，再生成元数据之后，将数据发布到集群中去，
- 在集群中接收元数据到内存中。以id为key
-


## 最终修改的文件

- go-sectorbuilder/fs/basic.go
- storedcounter.go
- node/modules/storageminer.go
- storage/fpost_sched.go
- storage/sectorstorage/manager.go # 当客户端AddPiece的时候，保存元数据到etcd里面
- storage/sectorstorage/sched.go # 解除cpu的限制
- storage/sectorstorage/stores/index.go #服务器查找文件路径修改 
- go.mod



