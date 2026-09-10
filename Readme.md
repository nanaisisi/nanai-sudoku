# nanai-sudoku

数独ソフト
・数独
・キラー数独（複数の小グループ）（未実装）

・下書きなど

Windowsのあべののじいさんさんのナンプレ独学とスマホのeasybrainのパクリ

## 🛠️ 開発・ビルドコマンド

Admin

winapp cert install devcert.pfx

```bash
# 実行
cargo run

# Clippyチェック & 自動修正
cargo clippy --fix --all --allow-dirty

# 依存関係の脆弱性チェック
cargo audit fix

# 構造チェック
cargo coupling --no-git --ai
```
