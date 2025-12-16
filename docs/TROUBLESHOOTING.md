# Solução de Problemas - YT-DLP Monitor

## Problemas de Compilação

### Erro: "cargo não é reconhecido como comando"

**Causa:** Rust não está instalado ou não está no PATH.

**Solução:**
1. Instale o Rust de: https://www.rust-lang.org/tools/install
2. Feche e reabra o terminal
3. Teste com: `cargo --version`
4. Se ainda não funcionar, reinicie o computador

### Erro: "error: linker 'link.exe' not found"

**Causa:** Visual Studio Build Tools não instalado.

**Solução:**
1. Baixe: https://visualstudio.microsoft.com/downloads/
2. Instale "Build Tools for Visual Studio"
3. Selecione "C++ build tools" durante instalação
4. Reinicie e tente compilar novamente

### Erro: "failed to fetch https://github.com/..."

**Causa:** Problemas de rede ou firewall bloqueando.

**Solução:**
1. Verifique sua conexão com internet
2. Desative temporariamente antivírus/firewall
3. Use VPN se estiver em rede corporativa
4. Tente: `cargo clean` e compile novamente

### Erro: "ícone não encontrado"

**Causa:** Arquivos de ícone ausentes.

**Solução:**
1. Crie ícones temporários:
   ```bash
   # No PowerShell ou terminal
   cargo tauri icon caminho/para/sua/imagem.png
   ```
2. Ou baixe ícones prontos e coloque em `src-tauri/icons/`
3. Necessário: icon.ico, 32x32.png, 128x128.png, 128x128@2x.png

### Compilação muito lenta

**Causa:** Primeira compilação baixa muitas dependências.

**Soluções:**
1. Seja paciente - primeira vez pode levar 10-20 minutos
2. Use SSD se possível
3. Aumente a RAM virtual se tiver pouca memória
4. Compilações subsequentes serão mais rápidas (1-3 min)

## Problemas de Execução

### Aplicativo não aparece no systray

**Soluções:**
1. Verifique se o systray não está oculto:
   - Clique na seta ^ na barra de tarefas
   - Procure o ícone do YT-DLP Monitor
2. Configure para sempre mostrar:
   - Configurações > Personalização > Barra de tarefas
   - Selecione quais ícones aparecer na barra

### Erro: "yt-dlp não encontrado"

**Causa:** yt-dlp não está no PATH ou não instalado corretamente.

**Verificação:**
```bash
# Teste no terminal
yt-dlp --version
```

**Solução:**
1. Se o comando acima falhar, reinstale o yt-dlp
2. Adicione ao PATH manualmente:
   - Painel de Controle > Sistema > Configurações avançadas
   - Variáveis de ambiente
   - Edite PATH e adicione pasta do yt-dlp
3. Reinicie o terminal e o aplicativo

### URL não é detectada

**Causas possíveis:**
1. URL não é de um site suportado
2. Área de transferência está travada por outro programa
3. Permissões insuficientes

**Soluções:**
1. Verifique se a URL é de:
   - YouTube, Vimeo, TikTok, Instagram, etc.
2. Copie novamente a URL
3. Execute o aplicativo como administrador
4. Feche outros programas que acessam área de transferência

### Download falha sempre

**Diagnóstico:**
```bash
# Teste manual do yt-dlp
yt-dlp --verbose "URL_DO_VIDEO"
```

**Soluções comuns:**
1. Atualize o yt-dlp:
   ```bash
   yt-dlp -U
   ```
2. Vídeo pode estar bloqueado geograficamente (use VPN)
3. Vídeo privado ou removido
4. Site mudou a API (aguarde atualização do yt-dlp)

### Pasta de destino não existe

**Erro:** Downloads não aparecem

**Solução:**
1. Verifique se a pasta existe: `C:\Users\Lucas\Desktop`
2. Se não existir, crie manualmente
3. Ou edite o caminho em `src-tauri/src/main.rs`:
   ```rust
   let download_path = "C:\\Users\\Lucas\\Desktop";
   // Altere para sua pasta preferida
   ```

### Aplicativo trava ou não responde

**Soluções:**
1. Feche pelo Gerenciador de Tarefas
2. Execute novamente
3. Verifique logs do Windows:
   - Visualizador de Eventos
   - Logs de Aplicativos
4. Se persistir, compile em modo debug:
   ```bash
   cargo tauri dev
   ```
   E veja os logs no terminal

## Problemas de Permissões

### Windows Defender bloqueia

**Solução:**
1. Adicione exceção no Windows Defender:
   - Segurança do Windows > Proteção contra vírus e ameaças
   - Gerenciar configurações
   - Exclusões > Adicionar exclusão
   - Selecione o executável ytdl-monitor.exe

### "Acesso negado" ao baixar

**Causas:**
1. Pasta de destino sem permissão de escrita
2. Antivírus bloqueando

**Solução:**
1. Execute como administrador
2. Mude a pasta de destino para uma com permissões
3. Desative temporariamente o antivírus

## Problemas de Performance

### Muita CPU/RAM

**Causa:** Monitoramento muito frequente da área de transferência.

**Solução:**
Edite `src-tauri/src/main.rs` e aumente o intervalo:
```rust
thread::sleep(Duration::from_millis(1000)); // Era 500, agora 1000
```

### Downloads lentos

**Causas:**
1. Conexão lenta
2. Servidor do vídeo limitando velocidade

**Soluções:**
1. Verifique sua internet
2. Use opções do yt-dlp para otimizar:
   ```rust
   .arg("--concurrent-fragments")
   .arg("4")
   ```

## Problemas com Sites Específicos

### YouTube: "Sign in to confirm you're not a bot"

**Solução:**
1. Atualize yt-dlp
2. Use cookies do seu navegador:
   ```bash
   yt-dlp --cookies-from-browser chrome URL
   ```
3. Configure no código (avançado)

### TikTok: Vídeos sem som

**Solução:**
Use formato específico:
```rust
.arg("-f")
.arg("best[ext=mp4]")
```

### Instagram: Erro 404

**Causas:**
1. Vídeo privado
2. Precisa estar logado

**Solução:**
Use cookies do navegador (ver YouTube acima)

## Debugging Avançado

### Ativar logs verbosos

Edite `main.rs` e adicione:
```rust
println!("Debug: Área de transferência mudou: {}", clipboard_content);
println!("Debug: É URL de vídeo: {}", is_video_url(&clipboard_content));
```

### Ver saída do yt-dlp

Modifique a chamada do comando:
```rust
let output = std::process::Command::new("yt-dlp")
    .arg("--verbose")
    .arg("--print-traffic")
    // ... outros args
    .output();

// Imprimir stdout e stderr
println!("STDOUT: {}", String::from_utf8_lossy(&result.stdout));
println!("STDERR: {}", String::from_utf8_lossy(&result.stderr));
```

### Testar em modo desenvolvimento

```bash
cargo tauri dev
```

Isso mostra todos os logs em tempo real no terminal.

## Problemas Conhecidos

### Windows 11 22H2 ou posterior

**Problema:** Pode haver atrasos no systray.

**Solução:** Aguarde alguns segundos após iniciar o app.

### Múltiplos monitores

**Problema:** Notificações podem aparecer no monitor errado.

**Solução:** Limitação do Windows, não há fix disponível.

### Área de transferência com imagens

**Problema:** App pode não detectar URLs se há imagem junto.

**Solução:** Copie apenas o texto da URL.

## Obtendo Ajuda

Se nenhuma solução acima resolver:

1. **Logs:** Procure por arquivos de log em:
   - `C:\Users\Lucas\AppData\Local\ytdl-monitor\`

2. **Teste manual:** Execute yt-dlp manualmente para isolar o problema:
   ```bash
   yt-dlp -v "URL_AQUI"
   ```

3. **Recompile em debug:**
   ```bash
   cargo tauri build --debug
   ```

4. **Verifique versões:**
   ```bash
   yt-dlp --version
   cargo --version
   ```

5. **GitHub Issues do yt-dlp:**
   - https://github.com/yt-dlp/yt-dlp/issues

## Checklist de Diagnóstico Rápido

- [ ] yt-dlp instalado e no PATH
- [ ] Rust/Cargo instalado
- [ ] Visual Studio Build Tools instalado (Windows)
- [ ] Ícones presentes em src-tauri/icons/
- [ ] Pasta de destino existe e tem permissões
- [ ] Firewall/antivírus não está bloqueando
- [ ] URL é de um site suportado
- [ ] App está visível no systray
- [ ] Teste manual do yt-dlp funciona

---

**Última atualização:** Versão 0.1.0  
**Plataforma:** Windows 11