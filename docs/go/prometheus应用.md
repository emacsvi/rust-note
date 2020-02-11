prometheus相关的一些使用。
<!--more-->

## Prometheus


## Grafana使用

### 每一个单词意思

其实我觉得除了 Variables 比较难理解，其他可能都是英文障碍吧，每个选项慢慢抠，都能理解。所以最后列出学习过程出现的单词及其理解。

简单说明：

- **Thresholds**:  根据`Singlestat`值，在面板中动态更改背景和值颜色。 阈值字段接受**2个逗号分隔值**，这些值表示直接对应于三种颜色的3个范围。 例如：如果阈值是`70,90`，那么第一种颜色代表**<70**，第二种颜色代表**70和90之间**，第三种颜色代表`> 90`。


| 名词           | 释义                                                                                                                         | 出现位置           |
| :--------      | :----------                                                                                                                  | :----------------- |
| spark line     | 走势图                                                                                                                       | Singlestat         |
| gauge          | 测量仪器，也就是设置显示为仪表盘那种图的意思                                                                                 | Singlestat         |
| value mapping  | 值映射，将数据的值映射为“一段文本”显示                                                                                       | Singlestat         |
| legend         | 图例                                                                                                                         | Graph              |
| playlist       | 播放列表，用来轮换显示播放列表里的                                                                                           | Dashboard          |
| gradient       | 梯度                                                                                                                         | Graph              |
| staircase      | 楼梯，阶梯，也就是阶梯线                                                                                                     | Graph              |
| hover tooltip  | 悬停提示                                                                                                                     | Graph              |
| series         | 每条线就可以理解为一个                                                                                                       | series             |
| stacking       | 堆叠，可以去搜索一下“堆积折线图”                                                                                             | Graph              |
| decimals       | 小数位数                                                                                                                     | Graph              |
| axes           | axis 的复数                                                                                                                  | Graph              |
| axis           | 轴，坐标轴                                                                                                                   | Graph              |
| thresholds     | 阈值                                                                                                                         | Alert              |
| time regions   | 时间区域，对特定时间区域标注，注意是 utc 时间                                                                                | Graph              |
| individual     | 独立的，理解“堆积折线图”，就知道啥意思了                                                                                     | Graph              |
| cumulative     | 累加的，理解“堆积折线图”，就知道啥意思了                                                                                     | Graph              |
| instant        | 实时的，instant query 就是只查询最新时间的数据，而不是一个时间段的数据                                                       | Table              |
| heatmap	热度图 |
| histograms     | 直方图                                                                                                                       | Dashboard          |
| evaluate       | 评估，出现在告警设置中，表示多长时间检查一下是否超过规则设定的阈值                                                           | Alert              |
| dedupe         | dedupe是单词De-duplication简单形式，可以用作动词或名词，意思是“重复数据删除“。Grafana 早期版本多机部署时，存在告警重复的情况 |Grafana |

另外，不想自己安装的，可以先到练兵场，摆弄一番。之后再自己安装一下，加深理解

**替换标签**：

在Grafana之中，由于node_export都是9100，而我的其他一些应用是9101端口的，在配置变量的时候。想将所有的instance="10.0.20.x:9100"的替换为instance="10.0.20.x:9101"即可以同步node的dashboard界面一起输出。

并且尽量不要修改prometheus本身的数据。

新建一个hip的变量，

```go
Query=label_value(node_uname_info{job=~"$job",instance=~"$node"}, instance)
// 只获取ip地址，不要端口号的正则
Regex=/([\d.]*):.*/


Query=query_result(label_replace(node_uname_info{job=~"$job",instance=~"$node"}, "instance", "$1:9101", "instance", "(.*):.*"))
Regex=/.*instance="(.*)".*/
```

另外一个替换外网标签的例子：

例如，有时候我们想要动态修改变量查询结果。比如某一个节点绑定了多个ip，一个用于内网访问，一个用于外网访问，此时prometheus采集到的指标是内网的ip，但我们需要的是外网ip。这里我们想要能在Grafana中动态改变标签值，进行ip段的替换，而避免从prometheus或exporter中修改采集指标。
这时需要使用grafana的`query_result`函数
```go
// 将10.10.15.xxx段的ip地址替换为10.20.15.xxx段 注：替换端口同理
query_result(label_replace(kube_pod_info{pod=~"$pod"}, "node", "10.20.15.$1", "node", "10.10.15.(.*)"))
// 通过正则从返回结果中匹配出所需要的ip地址
regex：/.*node="(.*?)".*/
```

### Table
制定表格举例，比如例出所有磁盘分区的使用空间情况 。

- 增加一个`Table Panel`表格面板。
- **Metrics**在写的时候，一定要选中`Instant`表示实时值，并且**Format**要为`Table`。
- 如果添加多条**Metrics**的时候，**Format**也要一致地为`Table`。
- 隐藏无用的数据: `Apply to columns named=/.*/ Type=Hidden`

```go
// 添加A,B,C三个metrics, 并且每一个都选中Instant和Format=Table值
node_filesystem_size_bytes{instance=~"10.0.20.3:9100", fstype=~"ext4"}-0
node_filesystem_avail_bytes{instance=~"10.0.20.3:9100", fstype=~"ext4"}-0
1 - (node_filesystem_avail_bytes{instance=~"10.0.20.3:9100", fstype=~"ext4"} / node_filesystem_size_bytes{instance=~"10.0.20.3:9100", fstype=~"ext4"})

// 增加/.*/ Type=Hidden 隐藏不要的字段

// Value #C 是百分比，Percent(0.0-0.1)  Thresholds=0.3,0.6 Cell
```

## 参考文献

- [模板化Dashboard](https://yunlzheng.gitbook.io/prometheus-book/part-ii-prometheus-jin-jie/grafana/templating)
- [prometheus的所有函数](https://prometheus.io/docs/prometheus/latest/querying/functions/#label_replace)
- [详细讲解每一个单词](https://tlog.cc/posts/grafana/grafana-get-start/)
- [简要说明参数](http://www.51niux.com/?id=239)


