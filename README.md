## 

1 api通过get获取客户端请求，参数包含图层、x坐标、y坐标。
2 代理使用rust通过ffi调用libmapserver功能，实现mapserver。
3 代理创建mapObj，转换wfs格式到栅格png返回到客户端。
