# rust-tcp

**rust-tcp** é um conjunto de demos em Rust para benchmarking de conexões TCP, incluindo:

* Um *echo server* que devolve ao cliente exatamente os bytes recebidos.
* Um *cliente* que envia payloads em potências de 2 (de 2 bytes até 1 GiB), mede o RTT e exibe o tempo de resposta com precisão de microssegundos.

## Recursos

* Implementação 100% em Rust, sem dependências de C.
* Servidor TCP simples e altamente eficiente.
* Cliente de benchmark com medição de RTT em nanosegundos.
* Formatação de resultados usando `bigdecimal` para exibir segundos com 6 casas decimais.

## Pré-requisitos

* Rust (via [rustup](https://rustup.rs/)) versão estável.
* Cargo (vem junto com Rust).

## Instalação

1. Clone este repositório:

   ```bash
   git clone https://github.com/mzet97/rust-tcp.git
   cd rust-tcp
   ```

2. Instale dependências e compile ambos os binários (*server* e *client*):

   ```bash
   cargo build --release
   ```

## Uso

### Servidor de Echo

Execute o servidor TCP em `0.0.0.0:9000` (ou ajuste o endereço conforme necessário):

```bash
./target/release/tcp-server
```

Ele aceitará conexões e ecoará de volta qualquer bloco de bytes enviado pelo cliente.

### Cliente de Benchmark

Em outro terminal, execute o cliente para conectar ao seu servidor de echo:

```bash
./target/release/tcp-client
```

O cliente vai pedir:

```
Digite o IP do servidor: 127.0.0.1
Digite a porta do servidor: 9000
```

Em seguida, ele automaticamente enviará payloads de **2, 4, 8 … até 1073741824 bytes (1 GiB)** e exibirá um relatório como:

```text
Tamanho:      2 bytes → Tempo de resposta: 0.000523 s
Tamanho:      4 bytes → Tempo de resposta: 0.000612 s
…
Tamanho: 1073741824 bytes → Tempo de resposta: 8.593567 s
```

## Dependências principais

No `Cargo.toml` do cliente:

```toml
[dependencies]
bigdecimal = "0.4"
num-bigint = "0.4"
```

* **bigdecimal**: formatação de frações com precisão arbitrária.
* **num-bigint**: suporta inteiros grandes para nanosegundos.

## Contribuição

Contribuições são muito bem-vindas! Para contribuir:

1. Fork deste repositório.
2. Crie uma branch para sua feature (`git checkout -b feature/minha-feature`).
3. Adicione testes e faça commit das suas mudanças (`git commit -m "Adiciona feature X"`).
4. Envie para o repositório remoto (`git push origin feature/minha-feature`).
5. Abra um Pull Request.

## Licença

Este projeto está licenciado sob a **MIT License**. Veja o arquivo [LICENSE](./LICENSE) para detalhes.
