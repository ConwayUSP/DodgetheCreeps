# Tutorial Dodge The Creeps em Rust
Implementação do jogo Dodge the Creeps do Godot Engine Tutorial em Rust. Essa implementação faz parte do curso [Lucky Crab](https://github.com/ConwayUSP/Lucky_Crab).

## Sobre
O jogo _Dodge The Creeps_ é um jogo 2D criado pelo Tutorial Oficial do Godot Engine. Apresenta-se aqui uma implementação utilizando a linguagem Rust como linguagem de scripting. Mova-se pela tela, desvie dos inimigos e sobreviva mais tempo.

## Como jogar?
Aperte Espaço ou clique no botão _Start_, mova-se com as setas e desvie dos inimigos, sobreviva o maior tempo possível para ganhar mais pontos.

## Como rodar?

### Requisitos
- Rust `>=1.89`
- Godot Engine `4.4.x`

Acesse a pasta `rust` e compile o projeto com `cargo build`. Depois, abra a pasta `godot` com seu editor do Godot, execute a cena `Main`. 

## Referências

Nesse tutorial foram utilizados os _assets_ do tutorial original, que pode ser encontrado [aqui](https://docs.godotengine.org/en/stable/getting_started/first_2d_game/index.html), tal conteúdo também foi usado como principal referência para esse projeto. Reconhecemos também a existência do mesmo [tutorial em Rust](https://github.com/godot-rust/demo-projects) feito pela equipe responsável pelo **godot-rust**, tomamos de inspiração o trabalho deles, principalmente para resolver conflitos de adaptação. A principal diferença entre nosso projeto é que utilizamos a versão da _crate_ godot `3.5` e as alterações necessária para manter compatibilidade.


