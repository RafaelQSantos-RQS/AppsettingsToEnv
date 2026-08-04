# AppsettingsToEnv

Converte `appsettings.json` (configuração do .NET) em variáveis de ambiente no formato `KEY=value`, pronto para usar em containers Docker.

## Uso

Cole o JSON no painel esquerdo — a conversão acontece em tempo real no painel direito. Depois é só copiar o resultado.

- **Upload** — carrega um arquivo `.json` para substituir o conteúdo do painel esquerdo
- **Limpar** — esvazia os dois painéis
- **Copiar** — copia o resultado para a área de transferência
- **Tema** — alterna entre os temas claro e escuro

## Regras de conversão

Segue a convenção da Microsoft para configuração por variáveis de ambiente:

| JSON | Resultado |
|---|---|
| `"ConnectionStrings": { "Default": "Server=db" }` | `ConnectionStrings__Default=Server=db` |
| `"AllowedHosts": ["a", "b"]` | `AllowedHosts__0=a`<br>`AllowedHosts__1=b` |
| `"Feature": null` | *(omitido)* |

- Chaves aninhadas usam `__` como separador
- Arrays são indexados (`Key__0`, `Key__1`, ...)
- Valores `null` são ignorados
- JSON inválido mostra o erro do parser no painel direito

## Como rodar

```bash
cargo run
```

## Testes

```bash
cargo test
```

## Stack

- **Rust** com [Slint](https://slint.dev) (UI desktop)
- **serde_json** (parse) e **arboard** (área de transferência), **rfd** (file picker)
