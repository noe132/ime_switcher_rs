# ime_switcher_rs
在 macos 上使用 `Command + Shift` 切换输入法。
不与其他快捷键冲突，仅在按键释放时切换输入法

## 安装
`brew install noe132/ime-switcher-rs/ime_switcher_rs`

## 使用说明
- 打开系统设置，把切换下一个输入法快捷键修改为 `lcmd + lshift + equal`
<img width="716" alt="image" src="https://github.com/user-attachments/assets/bef89046-4b3f-4076-a148-10a36c0eef3e" />
- 按下 `lcmd + lshift` 后，模拟一次 `lcmd + lshift + equal` 事件并切换输入法。

## 原理
检测到 `Command + Shift` 被按下并抬起后，模拟一次 `lcmd + lshift + equal` 键盘事件，触发输入法切换.
为啥不使用 macos 直接切换输入法的 API？因为有 bug，会出现切换失败的场景。

## Build
- install rust
- run `cargo build`
