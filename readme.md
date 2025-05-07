# ime_switcher_rs
在 macos 上使用 `Command + Shift` 切换输入法。  
不与其他快捷键冲突，仅在按键释放时切换输入法  

## 安装
`brew install noe132/ime-switcher-rs/ime_switcher_rs`

## 使用说明
- 打开系统设置，把切换下一个输入法快捷键修改为 `lcmd + lshift + equal`  
<img width="716" alt="image" src="https://github.com/user-attachments/assets/bef89046-4b3f-4076-a148-10a36c0eef3e" />

- 按下 `lcmd + lshift` 后，模拟一次 `lcmd + lshift + equal` 事件并切换输入法。
- 首次启动后，会弹窗提示，需要在 `Privacy & Security` 里授权 `Accessibility` 权限
<img width="716" alt="image" src="https://github.com/user-attachments/assets/976fbcee-de38-44d7-9ee2-ba0f94d6d50a" />


## 原理
安装后自动注册 brew service，登录后自动启动。  
检测到 `Command + Shift` 被按下并抬起后，模拟一次 `lcmd + lshift + equal` 键盘事件，触发输入法切换.  
为啥不使用 macos 直接切换输入法的 API？因为有 bug，会出现切换失败的场景。   
使用 Rust 编写，内存占用仅 1M  

<img width="741" alt="image" src="https://github.com/user-attachments/assets/45dba1ab-aff6-4673-94a5-7d7d87b0a8b4" />

## Build
- install rust
- run `cargo build`
