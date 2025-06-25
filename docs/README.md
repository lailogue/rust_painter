# Rust Painter - ドキュメント

このディレクトリには、Rust Painterプロジェクトに関連するすべての技術文書が含まれています。

## ディレクトリ構造

```
docs/
├── README.md                    # このファイル
├── current/                     # 現在のバージョンの技術文書
│   ├── rust_painter_technical_guide.md
│   ├── rust_painter_technical_guide.pdf
│   └── rust_painter_technical_guide_with_images.pdf
├── diagrams/                    # アーキテクチャ図とフロー図
│   ├── architecture_diagram.mmd
│   ├── architecture_diagram.png
│   ├── dataflow_diagram.mmd
│   ├── dataflow_diagram.png
│   ├── module_dependencies.mmd
│   └── module_dependencies.png
├── legacy/                      # 過去のバージョンや廃止された文書
│   ├── code_explanation.md
│   ├── code_explanation.pdf
│   ├── egui_eframe_guide.md
│   └── egui_eframe_guide_enhanced.pdf
└── analysis/                    # フレームワーク分析と比較文書
    ├── rust_2d_graphics_framework_analysis.md
    └── rust_2d_graphics_framework_analysis.pdf
```

## 文書の説明

### current/ - 現在のバージョン
- **`rust_painter_technical_guide.md`**: 最新の技術仕様書（Markdown版）
- **`rust_painter_technical_guide.pdf`**: 技術仕様書のPDF版（図表なし）
- **`rust_painter_technical_guide_with_images.pdf`**: 技術仕様書のPDF版（図表付き）

### diagrams/ - 図表ファイル
- **アーキテクチャ図**: システム全体の構造を示す図表
- **データフロー図**: ユーザー操作からレンダリングまでの処理フロー
- **モジュール依存関係図**: ソースコードモジュール間の依存関係

各図表は以下の形式で提供：
- `.mmd`: Mermaidソースファイル
- `.png`: 生成された画像ファイル

### legacy/ - 過去のバージョン
- **eGUI/eframeベースの実装**: 初期バージョンの技術文書
- **コード解説文書**: 過去のバージョンのコード解説

### analysis/ - 分析文書
- **フレームワーク比較**: Rust 2Dグラフィックスフレームワークの比較分析

## 利用方法

### 開発者向け
最新の技術情報については `current/` ディレクトリの文書を参照してください。

### プロジェクト保守
過去の実装や設計判断の経緯については `legacy/` と `analysis/` ディレクトリを参照してください。

### 図表の更新
`diagrams/` ディレクトリの `.mmd` ファイルを編集し、以下のコマンドで画像を再生成できます：

```bash
mmdc -i diagrams/filename.mmd -o diagrams/filename.png -w 1200 -H 800
```

## 技術スタック
- **文書作成**: Markdown
- **PDF生成**: Pandoc + LuaLaTeX
- **図表作成**: Mermaid
- **バージョン管理**: Git

---

*最終更新: 2025-06-26*
*プロジェクト: Rust Painter v2.0.0*