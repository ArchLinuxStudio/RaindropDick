## RaindropDic

### v2ray tui 界面，静态编译

tui界面的v2ray

按键

* h
	* 按h出设置界面，设置界面里面有两个设置
		* v2raycore 
		* 订阅，当在第二个设置里面添加了订阅后按下键可以选中订阅，d键删除
		* s 回到popup的只读界面后按下s进行设置更新和订阅同步，同步失败后不会回退，应该需要做个提示
* e
	* TODO, 计划作为设置
* s
	* 进入选择节点界面
		* 此界面按F5可以运行core,ese+q退出后设置代理可以访问互联网
		* 此界面左右键可以切换订阅
* ese + q
	* 此两键各种退出

使用

```sh
export http_proxy=http://127.0.0.1:8889
export https_proxy=http://127.0.0.1:8889
```

然后可以联网

下载方式

cargo :

```sh
cargo install raindropdic
```

wget

```sh
https://endpoint.fastgit.org/ArchLinuxStudio/RaindropDick/releases/download/v0.3.5/raindropdick
```
