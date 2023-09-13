# bb_status

> :warning: **DEPRECATED**: A inclusão de um CAPTCHA na página de consulta
> impossibilitou a automação do crawler

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

## Automação com systemd

A execução do programa pode ser automatizada com a criação de um serviço.
Primeiro, crie o arquivo `.service` em `~/.config/systemd/user` com o conteúdo
abaixo.

```ini
# ~/.config/systemd/user/bb_status.service
[Unit]
Description=bb_status mailer service
After=network.target
Wants=bb_status.timer

[Service]
Type=oneshot
WorkingDirectory=%h/Imagens/bb_status
ExecStart=/bin/sh -c 'bb_status <CPF> -o "$(date -I).png" -e <EMAIL>'

[Install]
WantedBy=default.target
```

Para configurar o ambiente de serviços de um usuário, crie um arquivo `.conf` em `~/.config/environment.d` seguindo o formato abaixo.

```ini
# ~/.config/environment.d/bb_status.conf
PATH="$PATH:$HOME/.cargo/bin"
BB_SMTP_USER="your_smtp_username@email.com"
BB_SMTP_PWD="your_smtp_password"
BB_SMTP_SERVER="smtp.server.com"
```

Para testar a execução do serviço, recarregue o daemon do systemd

```bash
$ systemctl --user daemon-reload
```

e rode o serviço com

```bash
$ systemctl --user start bb_status.service
```

A saída do programa ou eventuais erros são acessíveis com

```bash
$ systemctl --user status bb_status.service
```

Para executar o serviço periodicamente, crie no mesmo diretório um arquivo
`.timer` com o mesmo prefixo e seguinte conteúdo.

```ini
# ~/.config/systemd/user/bb_status.timer
[Unit]
Description=bb_status mailer service
Requires=bb_status.service

[Timer]
Unit=bb_status.service
OnCalendar=*-*-* 10:00:00

[Install]
WantedBy=timers.target
```

Aqui o campo `OnCalendar` controla quando o serviço `bb_status.service` será
executado — no caso acima, diariamente às 10h. Recarregue o daemon do systemd
novamente e ative o timer com

```bash
$ systemctl --user enable --now bb_status.timer
```

Em caso de sucesso, o timer, serviço correspondente, horário das última e
próxima execuções serão listados na saída de

```bash
$ systemctl --user list-timers
```
