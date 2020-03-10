## 源码库

- 使用了echo来做路由功能
  + 静态文件echo#Static() 与 echo#File(path, file string)的用法与区别
  + 中间件echo#User()，中间件运行在路由处理完请求之后，可以调用所有的echo.Context API。

- 使用gorilla/sessions保存session处理
- 使用gorilla/schema将值转换为结构体

## go template
最全的教程

[go template讲得最全的 看它就够了](https://github.com/unknwon/building-web-applications-in-go)

## template嵌套
- [嵌套调用](https://colobu.com/2016/10/09/Go-embedded-template-best-practices/)
- [点来传递值](https://blog.csdn.net/zhengzizhi/article/details/73865412)

## 参考

- [gorilla库](https://blog.gmem.cc/gorilla-study-note)
- [gorilla/sessions示例](https://gowebexamples.com/sessions/)
- [go template讲得最全的 看它就够了](https://github.com/unknwon/building-web-applications-in-go)
