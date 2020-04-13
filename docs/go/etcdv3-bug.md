## etcdv3编译出错

今天遇到了一个小坑：使用`etcdv3`的时候，报了这么一堆错误

```
# github.com/coreos/etcd/clientv3/balancer/resolver/endpointvendor\github.com\coreos\etcd\clientv3\balancer\resolver\endpoint\endpoint.go:114:78: undefined: resolver.BuildOption

vendor\github.com\coreos\etcd\clientv3\balancer\resolver\endpoint\endpoint.go:182:31: undefined: resolver.ResolveNowOption

# github.com/coreos/etcd/clientv3/balancer/pickervendor\github.com\coreos\etcd\clientv3\balancer\picker\err.go:37:44: undefined: balancer.PickOptions

vendor\github.com\coreos\etcd\clientv3\balancer\picker\roundrobin_balanced.go:55:54: undefined: balancer.PickOptions
```

上网查了一下原因，说是最新的`v1.27.0`版本的`google.golang.org/grpc`包不支持`etcdv3`。同时网上也给出了解决方案：将`grpc1.27.0`替换成`grpc1.26.0`版本（具体操作是手动在`go.mod`的`require`下修改`google.golang.org/grpc v1.26.0`，或者直接用命令`go mod edit -require=google.golang.org/grpc@v1.26.0）`

but~~~~实际操作后发现，替换包版本号的方案是对的，但是操作不正确，修改require不能指定包的版本号！

**正确的做法**是，在go.mod中用replace指定包版本号，比如：

```go
replace google.golang.org/grpc => google.golang.org/grpc v1.26.0
```

然后再`go run`或`go build`，一切完美~~


