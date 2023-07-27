# bb_status

Crawler para acompanhamento das convocações do concurso público do Banco do
Brasil de 2022. A aplicação levanta um processo filho executando uma instância
do navegador Chromium na
[página de consulta](https://www37.bb.com.br/portalbb/resultadoConcursos/resultadoconcursos/arh0.bbx),
digita o CPF informado e captura a área da viewport correspondente à tabela com
o progresso das convocações. O usuário pode salvar a captura em um arquivo e
enviá-la como um anexo para seu endereço de email.

## Instalação e uso

Para instalar, use o comando

```bash
$ cargo install --git https://github.com/davifeliciano/bb_status
```

A instalação deve levar alguns minutos. Uma vez instalado, rode o programa com

```bash
$ bb_status <CPF> -o today_bb_status.png
```

Para ver todas as opções disponíveis, consulte a mensagem de ajuda passando a
flag `--help`. Caso opte por usar a feature de emails, será necessário
configurar algumas variáveis de ambiente.

```bash
# final de ~/.bashrc, ~/.zshrc, etc
export BB_SMTP_USER="your_smtp_username@email.com"
export BB_SMTP_PWD="your_smtp_password"
export BB_SMTP_SERVER="smtp.server.com"
```

Caso vá usar uma conta do Gmail para o envio de emails, por exemplo, será
necessário habilitar a verificação de duas etapas e gerar uma
[senha de aplicativo](https://support.google.com/accounts/answer/185833)
para usar em `BB_SMTP_PWD`. Já o servidor será `BB_SMTP_SERVER="smtp.gmail.com"`.

## Automação com crond

A execução do programa pode ser automatizada com o uso de do serviço crond.
Primeiro, tenha certeza de que o serviço está em execução com

```bash
$ sudo systemctl status crond
```

Caso não esteja, inicie com

```bash
$ sudo systemctl start crond
```

Em seguida, crie um arquivo crontab com

```bash
$ crontab -e
```

Esse comando abrirá tal arquivo para edição em um editor de linha de comando.
Para realizar uma consulta periodicamente de segunda a sexta, às 10h, digite

```bash
BB_SMTP_USER="your_smtp_username@email.com"
BB_SMTP_PWD="your_smtp_password"
BB_SMTP_SERVER="smtp.server.com"
OUTDIR="Imagens/bb_status"
0 10 * * 1-5 .cargo/bin/bb_status <CPF> -o "$OUTDIR/bb_status_$(date +%Y%m%d).png" -e <EMAIL>
```

e salve o arquivo. Note que as variáveis de ambiente com as configurações do
servidor SMTP devem ser incluídas no arquivo, já que o crond roda o comando em
um ambiente isolado. Por esse mesmo motivo rodamos o `bb_status` com
`.cargo/bin/bb_status`, uma vez que, no ambiente usado, a variável `PATH` é mais
restritiva.

Em caso de sucesso, essa mesma linha deve aparecer na saída de `crontab -l` e
uma mensagem indicando o carregamento do arquivo crontab será exibida nos logs
do serviço, acessíveis via `sudo systemctl status crond`.
