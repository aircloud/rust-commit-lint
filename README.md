
# 简易 commit-lint 可执行程序

> 由于在不引入 node 和 npm 的情况下暂时没有找到特别合适的 commit-msg hook 程序，因此写了此简易程序。

使用方式：

```
commit lint git hook

USAGE:
    rust-commit-lint --config_path <config_path> --message <MESSAGE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config_path <config_path>    the lint config path
    -m, --message <MESSAGE>            commit message
```

## 配置 git-hook

参考 `demo/commit-msg`