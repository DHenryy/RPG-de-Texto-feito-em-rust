# RPG-de-Texto-feito-em-rust
Trabalho da faculdade entregue em 17/10/2025

Para rodar o código em seu computador:
1. será necessário a instalação do pacote do C++ (que pode ser baixado como pacote durante a instalação do Visual Studio) https://visualstudio.microsoft.com/pt-br/visual-cpp-build-tools/
2. instalar rustup do site: https://rust-lang.org/tools/install/  para poder executar e realizar os codigos
3. abra o cmd e digite rustup -V para verificar se foi instalado (irá mostrar a versão que está atualmente instalada)
4. crie um nova pasta com o comando "mkdir (nome da pasta pai)", enter
5. digite "cd (nome da pasta)", enter
6. digite "cargo new (nome da nova pasta para o codigo)", enter
7. depois digite "cd (nome da pasta que criou)", enter
8. após entrar na pasta que criou, digite "cargo build", enter
9. apos ter buildado no cmd, digite "code .", este comando faz com que o vscode seja aberto automaticamente naquela pasta.
10. quando o vscode abrir, poderá ver que no explorer haverá as files "src, target, .gitignore, cargo.lock, cargo.toml", para este codigo, teremos que importar duas crates, que funcionam como uma biblioteca do rust
11. clique no "cargo.toml" e abaixo de dependencies digite "rand = 0.8" e "colored = 2.1" e dê um CTRL + S
12. depois disso, clique no "src" e vá em "main.rs"
13. copie o código que estará no "main.rs" desse repositório para o seu main.rs
14. depois de ter colado e salvo, no mesmo cmd que já está aberto, digite "cargo run" e ele começará a rodar o código.


é preferível o download das seguintes extensões no vscode:
rust-analyzer
c/c++ extension pack (caso já venha instalado depois de baixar os pacotes do C++ mencionados acima, não precisa se preocupar)
C/c++

