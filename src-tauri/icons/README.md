# Ícones Necessários

Coloque os seguintes arquivos de ícone nesta pasta antes de compilar:

- `32x32.png` - Ícone 32x32 pixels
- `128x128.png` - Ícone 128x128 pixels
- `128x128@2x.png` - Ícone 256x256 pixels (versão retina)
- `icon.icns` - Ícone para macOS (se aplicável)
- `icon.ico` - Ícone para Windows (OBRIGATÓRIO)

## Como Gerar os Ícones

### Opção 1: Usar o Tauri Icon Generator

```bash
cargo install tauri-cli
cargo tauri icon /caminho/para/seu/icone.png
```

Isso gerará automaticamente todos os tamanhos necessários.

### Opção 2: Manualmente

Use qualquer ferramenta de edição de imagens para criar:
- Um ícone quadrado (recomendado 512x512 ou maior)
- Redimensione para os tamanhos necessários
- Para Windows, use uma ferramenta como o IcoFX ou online: https://converticon.com/

### Opção 3: Usar um Ícone Temporário

Para testes rápidos, você pode usar um ícone genérico ou criar um simples com:
- https://favicon.io/
- https://www.flaticon.com/

## Recomendações

- Use imagens PNG com fundo transparente
- Design simples e reconhecível
- Cores contrastantes para visibilidade no systray
- Teste em fundos claros e escuros do Windows