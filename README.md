# 🔖 Descrição

Projeto desktop criado para podermos desencriptar dados do projeto Polaris como:  
- **Documento consultado**
- **Documento consultante**
- **Query graphql**

# Tecnologias utilizadas  
## Rust versão 1.85.0 - utilizado para Back-end
-<img src="config/img/rust-icon-512x512-16xbdsxq.png" width="32" height="32"> [Rust Site oficial](https://www.rust-lang.org/tools/install)

## Tauri versão 1.8.2 - utilizado para Front-end

-<img src="config/img/icon.png" width="32" height="32"> [Tauri Site oficial](https://v1.tauri.app/)

## <img src="config/img/java-icon-1511x2048-6ikx8301.png" width="32" height="32"> Java - utilizado para realizar desencriptação dos dados

## 🧑‍💻 Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) 
- [RustRover](https://www.jetbrains.com/rust/)

## 🔧 Comandos uteis
### Gerar icones
Com o comando `cargo tauri icon [OPTIONAL] [INPUT]` o cargo vai gerar todos icones necessário para os SO Linux, MAC e Windows.  
```
Generates various icons for all major platforms

Usage: cargo tauri icon [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Path to the source icon (png, 1024x1024px with transparency) [default: ./app-icon.png]

Options:
  -o, --output <OUTPUT>  Output directory. Default: 'icons' directory next to the tauri.conf.json file
  -v, --verbose...       Enables verbose logging
  -p, --png <PNG>        Custom PNG icon sizes to generate. When set, the default icons are not generated
  -h, --help             Print help
  -V, --version          Print version
```

**Exemplo:**  
```
cargo tauri icon /Users/Desktop/polaris_decrypt/src-tauri/icons/icon_128.png
```

### Buildar projeto para gerar executavel
Na raiz do projeto executar o camando `cargo tauri build`  

