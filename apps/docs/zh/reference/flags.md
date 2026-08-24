# 我们添加的参数

应用直接启动 `python.exe`，用的是从配置里解析出来的参数，外加最多三个它
自己的。启动任何东西之前，都能在启动参数编辑器里看到最终的命令行。

## `--port <n>`

端口由应用分配：整合包的首选端口空闲就用它，否则用下一个空闲的。

配置里已有的任何 `--port` 都会**连同它的值一起**被移除，两种写法都算
（`--port 8188` 和 `--port=8188`）。两个整合包用同一个写死的端口，意味着
第二个会在启动时失败，而失败的理由看上去和你做过的任何事都对不上。

## `--disable-auto-launch`

好让启动整合包时不弹出浏览器标签页。

这不需要修改任何东西就能生效，靠的是 ComfyUI 自己处理参数时的顺序
（`comfy/cli_args.py`）：

```python
if args.windows_standalone_build:
    args.auto_launch = True
if args.disable_auto_launch:
    args.auto_launch = False
```

关闭的那个参数在后面才被应用，所以它总是胜出——把它追加到命令行上就够了。

## `--extra-model-paths-config <路径>`

只有当整合包以**参数模式**连接到共享模型文件夹时才添加。这个路径指向应用
自己数据目录里的那份配置，设置一有改动它就会重新生成。

配置里已有的 `--extra-model-paths-config` **不会**被动。这个参数是累加的
（`nargs='+'`、`action='append'`），而 ComfyUI 会先加载
`ComfyUI/extra_model_paths.yaml`，再加载参数里给的文件，所以我们的是加在
你的之上，而不是替换掉它。

在“文件位于安装目录内”模式下，则完全不添加参数：文件就在整合包目录里，
ComfyUI 自己会读到。

## 我们从不添加的东西

- `--enable-cors-header`。它能让应用把 ComfyUI 嵌进 `<iframe>` ——同时也
  彻底关掉了 ComfyUI 的跨站保护，也就是说你浏览器里打开的任何网站都能跟
  你本地的服务器对话。应用改用了一个原生子窗口，ComfyUI 自己的中间件会把
  它当作普通的顶层导航接受。
- `--listen`。服务器就待在 `127.0.0.1` 上。
- 任何会改动模型路径、而不止于上面那份配置的东西。
