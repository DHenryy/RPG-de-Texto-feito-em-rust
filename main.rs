use std::{io, thread, time::Duration, io::Write};  // importa módulos padrão do Rust
use rand::{Rng, distributions::WeightedIndex, distributions::Distribution, seq::SliceRandom, thread_rng}; // importa a crate rand para geração de números aleatórios
use colored::*;    // importa a crate colored para colorir o texto no terminal

fn digita_baixo(text: &str, delay_ms: u64) {
    for c in text.chars() {
        print!("{}", c);
        io::Write::flush(&mut io::stdout()).unwrap(); // força a saída imediata
        thread::sleep(Duration::from_millis(delay_ms)); // pausa por um tempo
    }
    println!(); // nova linha após o texto
} // função para simular digitação lenta no terminal

#[derive(Debug, Clone, Copy)] 
enum Classes {
    Guerreiro,
    Mago,
    Arqueiro,
    Padre,
    Ladrão,
} // define a enumeração para as classes de personagens

#[derive(Debug, Clone, Copy)] // define a enumeração para os inimigos
enum Inimigos {
    Goblin,
    Zumbi,
    Esqueleto,
    CavaleiroAssombrado,
    Wyrm,
}

#[derive(Debug)] // define a estrutura da sala
struct Sala {
    descricao: &'static str,
    inimigo : Option<Inimigos>,
    tesouro: Option<&'static str>,
    visitada: bool
}


#[derive(Debug)] 
#[allow(dead_code)] 
// define a estrutura do personagem, com atributos básicos
struct Personagem {
    nome: String,
    classe: Classes,
    força: i32,
    res_fisica: i32,
    res_magica: i32,
    vida_atual: i32,
    vida_max: i32,
    mana_atual: i32,
    mana_max: i32,
    xp_atual: i32,
    xp_para_nivel: i32,
    nivel: i32,
}

#[derive(Debug)] // define a estrutura do inimigo
struct InimigoAtivo {
    tipo: Inimigos,
    vida_atual: i32,
    vida_max: i32,
    força: i32,
    res_fisica: i32,
    res_magica: i32,
    xp_dropado: i32,
}

impl Inimigos {
 // implementação de métodos para a enum Inimigos, no caso, ele cria as funções para os inimigos, como os pesos que seriam em % para cada um deles 
// aparecerem nas salas possíveis de respawn. o valor para cada esses atributos no struct referente aos inimigos ativos são definidos na função "novo" abaixo
// e abaixo da função do "Novo", está sendo realizada a ação de ataque por parte do inimigo ao personagem

      
    fn aleatorio() -> Self {
        let mut rng = thread_rng();
        let inimigos = [
            Inimigos::Goblin,
            Inimigos::Zumbi,
            Inimigos::Esqueleto,
            Inimigos::CavaleiroAssombrado,
            Inimigos::Wyrm,
        ];
        let pesos = [25, 25, 20, 21, 9];
        let dist = WeightedIndex::new(&pesos).unwrap();
        inimigos[dist.sample(&mut rng)]
    }
}

impl InimigoAtivo { // implementação de métodos para a estrutura InimigoAtivo
    fn novo(tipo: Inimigos) -> Self {
        match tipo {
            Inimigos::Goblin => Self { tipo, vida_atual: 30, vida_max: 30, força: 10, res_fisica: 5, res_magica: 3, xp_dropado: 10 },
            Inimigos::Zumbi => Self { tipo, vida_atual: 40, vida_max: 40, força: 5, res_fisica: 5, res_magica: 4, xp_dropado: 20 },
            Inimigos::Esqueleto => Self { tipo, vida_atual: 35, vida_max: 35, força: 10, res_fisica: 2, res_magica: 2, xp_dropado: 20 },
            Inimigos::CavaleiroAssombrado => Self { tipo, vida_atual: 50, vida_max: 50, força: 16, res_fisica: 5, res_magica: 2, xp_dropado: 40 },
            Inimigos::Wyrm => Self { tipo, vida_atual: 80, vida_max: 80, força: 20, res_fisica: 15, res_magica: 15, xp_dropado: 100 },
        }
    }
fn atacar(&self, personagem: &mut Personagem, magico: bool) {
    let mut rng = rand::thread_rng();
    // Gera um dano aleatório uma única vez
    let dano_bruto = rng.gen_range((self.força - 3).max(0)..=self.força + 6);

    let dano_final = if magico {
        calcular_dano(dano_bruto, personagem.res_magica)
    } else {
        calcular_dano(dano_bruto, personagem.res_fisica)
    };

    personagem.vida_atual -= dano_final;
    let tipo_ataque = if magico { "mágico" } else { "físico" };
    digita_baixo(
        &format!("O {:?} usa ataque {} e causa {} de dano!", self.tipo, tipo_ataque, dano_final),
        30
    );personagem.mostrar_status();
}
    fn mostrar_status(&self) {
        println!("{}", "===== STATUS INIMIGO =====".bold().red());
        println!("Tipo: {:?}", self.tipo);
        println!("Vida: {}/{}", self.vida_atual, self.vida_max);
        println!("Força: {}", self.força);
        println!("Resistência Física: {}", self.res_fisica);
        println!("Resistência Mágica: {}", self.res_magica);
        println!("{}", "==========================".bold().red());
        // aqui mostra os status do inimigo após você/ele atacar
    }

}

impl Sala {
    fn nova(descricao: &'static str) -> Self {
        let mut rng = thread_rng();

        // chance de aparecer inimigo
        let inimigo = if rng.gen_bool(0.6) {
            Some(Inimigos::aleatorio())
        } else {
            None
        };

        // chance de aparecer tesouro
        let tesouro = if rng.gen_bool(0.25) {
            Some("Você ganhou 50 de XP!")
        } else {
            None
        };

        Sala {
            descricao,
            inimigo,
            tesouro,
            visitada: false,
        }
    }
    fn mostrar(&mut self) {
        println!("{}", "------------------------------------".bright_black());
        println!("{}", self.descricao.bold().white());

        if let Some(inimigo) = &self.inimigo {
            println!("Você encontrou um inimigo: {:?}", inimigo);
        }

        if self.inimigo.is_none() && self.tesouro.is_none() {
            println!("A sala está vazia...");
        }

        println!("{}", "------------------------------------".bright_black());
        self.visitada = true;
    }
    fn tentar_regenerar_inimigo(&mut self) {
        let mut rng = thread_rng();
        if self.visitada && self.inimigo.is_none() && rng.gen_bool(0.2) {
            self.inimigo = Some(Inimigos::aleatorio());
            println!("{}", "Algo está a espreita novamente...".red());
        }
    }
}
impl Personagem {
    
    fn new(nome: String, classe: Classes) -> Self {
        let (força, res_fisica, res_magica, vida_max, mana_max) = match classe {
            Classes::Guerreiro => (13, 10, 1, 100, 30),
            Classes::Mago => (5, 5, 13, 70, 100),
            Classes::Arqueiro => (10, 7, 8, 80, 30),
            Classes::Padre => (7, 10, 15, 80, 60),
            Classes::Ladrão => (8, 8, 8, 75, 40),
        };
// inicializa o personagem com atributos baseados na classe escolhida
        Self {
            nome,
            classe,
            força,
            res_fisica,
            res_magica,
            vida_atual: vida_max,
            vida_max,
            mana_atual: mana_max,
            mana_max,
            xp_atual: 0,
            xp_para_nivel: 100,
            nivel: 1,
        }// retorna a instância do personagem
        }
    fn mostrar_status(&self) { //aqui seria para mostrar o status do personagem com as cores diferentes e tal
        println!("{}", "================ STATUS ================".bold().cyan());
        digita_baixo(&format!("{} {}", "Nome:".bold().white(), self.nome.bright_yellow()), 1);
        digita_baixo(&format!("{} {:?}", "Classe:".bold().white(), self.classe), 1);
        digita_baixo(&format!("{} {}", "Nível:".bold().white(), self.nivel.to_string().bright_green()), 1);
        digita_baixo(&format!("{} {}", "Força:".bold().white(), self.força.to_string().red()), 1);
        digita_baixo(&format!("{} {}", "Resistência Física:".bold().white(), self.res_fisica.to_string().blue()), 1);
        digita_baixo(&format!("{} {}", "Resistência Mágica:".bold().white(), self.res_magica.to_string().magenta()), 1);
        digita_baixo(&format!("{} {}/{}", "Vida:".bold().white(), self.vida_atual.to_string().green(), self.vida_max), 1);
        digita_baixo(&format!("{} {}/{}", "Mana:".bold().white(), self.mana_atual.to_string().cyan(), self.mana_max), 1);
        digita_baixo(&format!("{} {}/{}", "XP:".bold().white(), self.xp_atual.to_string().yellow(), self.xp_para_nivel), 1);
        println!("{}", "========================================".bold().cyan());   // metodo para mostrar o status do personagem com cores
    }
    fn atacar(&self, inimigo: &mut InimigoAtivo, magico: bool) { // ataque do personagem ao inimigo
         let mut rng = rand::thread_rng();
         let dano_bruto = rng.gen_range((self.força - 1).max(0)..=self.força + 7);
        let dano_final = if magico {
            calcular_dano(dano_bruto, inimigo.res_magica)
        } else {
            calcular_dano(dano_bruto, inimigo.res_fisica)
        };
        inimigo.vida_atual -= dano_final;
        let tipo_ataque = if magico { "mágico" } else { "físico" };
        digita_baixo(&format!("Você usa ataque {} e causa {} de dano!", tipo_ataque, dano_final), 30);
        inimigo.mostrar_status();
    }

    // novo método para aplicar ganhos ao subir de nível
    fn level_up(&mut self) {
        self.nivel += 1;
        // ajusta atributos ao subir de nível (valores exemplo, ajuste conforme desejar)
        self.força += 2;
        self.res_fisica += 1;
        self.res_magica += 1;
        self.vida_max += 10;
        self.mana_max += 5;
        // restaura vida e mana ao máximo ao subir de nível
        self.vida_atual = self.vida_max;
        self.mana_atual = self.mana_max;
        digita_baixo(&format!("Parabéns! Você subiu para o nível {}!", self.nivel), 50);
        println!("{}", "Seus atributos aumentaram e você recuperou vida e mana!".green());
    }
}

impl Classes {
    fn aleatoria() -> Self {
        let mut rng = thread_rng();
        let classes = [
            Classes::Guerreiro,
            Classes::Mago,
            Classes::Arqueiro,
            Classes::Padre,
            Classes::Ladrão,
        ];// array com todas as classes possíveis
        *classes.choose(&mut rng).unwrap()// escolhe uma classe aleatória
    }
}

fn calcular_dano(dano_bruto: i32, resistencia: i32) -> i32 {
    let dano_final = dano_bruto - resistencia;
    if dano_final < 0 { 0 } else { dano_final }
}

fn explorar_castelo(personagem: &mut Personagem) {
    // cria salas fixas
    let mut sala1 = Sala {
    descricao: "Você está na entrada de um antigo castelo. Há um corredor à frente.",
    inimigo: None,
    tesouro: None,
    visitada: false,
    }
    ;
    let mut sala2 = Sala::nova("Um corredor frio e úmido, o som de gotas ecoa nas paredes.");
    let mut sala3 = Sala::nova("Você chega a uma bifurcação. Caminhos seguem para cima e para o lado.");
    let mut sala4 = Sala::nova("Uma antiga biblioteca coberta de poeira e livros em decomposição.");
    let mut sala5 = Sala::nova("O salão principal, iluminado por tochas espectrais.");
   
    let mut posicao = 0usize; // posição inicial
    let limite = 4usize; // número de salas - 1
    let mut passos_no_corredor = 0; // contador de passos no corredor
    let mut rng = thread_rng();

    let mut primeiro_loop = true; // evita limpar a tela na primeira vez

   loop {
        if !primeiro_loop {
            print!("{esc}[2J{esc}[H", esc = 27 as char); // limpa a tela apenas depois da primeira iteração
        } else {
            primeiro_loop = false;
        }

        println!("{}", "=== EXPLORAÇÃO DO CASTELO ===".bold().cyan());

        // Mostra a sala atual
        match posicao {
            0 => sala1.mostrar(),
            1 => sala2.mostrar(),
            2 => sala3.mostrar(),
            3 => sala4.mostrar(),
            4 => sala5.mostrar(),
            _ => unreachable!(),
        }

        // Verifica se o personagem está vivo
        if personagem.vida_atual <= 0 {
            println!("{}", "Você morreu! Fim de jogo.".red());
            break;
        }

        // verifica se há inimigo na sala atual (consome campo .inimigo)
        let inimigo_presente = match posicao {
            0 => sala1.inimigo.take(),
            1 => sala2.inimigo.take(),
            2 => sala3.inimigo.take(),
            3 => sala4.inimigo.take(),
            4 => sala5.inimigo.take(),
            _ => None,
        };

        // Se houver inimigo, inicia a batalha 
        if let Some(mut inimigo_tipo) = inimigo_presente {
            let sucesso = batalha(personagem, &mut inimigo_tipo);
            if !sucesso {
                digita_baixo(&format!("Você acorda na última sala segura (posição {})...", posicao), 50);
                personagem.vida_atual = personagem.vida_max; // regen vida
                continue; // volta ao loop
            } 
        }

        println!("\nMovimentos possíveis:");
        if posicao > 0 { print!("(S) Voltar "); }
        if posicao < limite { print!("(A) Avançar "); }
        if posicao == 2 { print!("(W) Subir (biblioteca) "); } // Caminho alternativo
        if posicao == 3 { print!("(D) Descer (de volta ao corredor) "); }
        println!("(C) Checar status ");
        if let Some(_) = match posicao {
            0 => &sala1.tesouro,
            1 => &sala2.tesouro,
            2 => &sala3.tesouro,
            3 => &sala4.tesouro,
            4 => &sala5.tesouro,
            _ => &None,
        }
            {
    println!("(B) Abrir baú ");
    }

        println!("(Q) Sair");

        print!("\n> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let comando = input.trim().to_uppercase();

        match comando.as_str() {
            "S" if posicao > 0 => posicao -= 1,
            // Caso especial: se estiver na bifurcação (2), "A" deve seguir pelo corredor principal (pular biblioteca)
            "A" if posicao < limite => {
                if posicao == 2 {
                    posicao = 4; // segue pelo corredor principal (ajuste conforme seu mapa)
                } else {
                    posicao += 1;
                }
            }
            "W" if posicao == 2 => posicao = 3, // sobe para a biblioteca
            "D" if posicao == 3 => posicao = 2, // desce da biblioteca para o corredor
            "Q" => {
                println!("{}", "Você decide deixar o castelo... Obrigado por jogar!".yellow());
                break;
            }
            "B" => {
                match posicao {
                    0 => loot_tesouro(personagem, &mut sala1),
                    1 => loot_tesouro(personagem, &mut sala2),
                    2 => loot_tesouro(personagem, &mut sala3),
                    3 => loot_tesouro(personagem, &mut sala4),
                    4 => loot_tesouro(personagem, &mut sala5),
                    _ => {}
                }
                continue;
            }

            "C" => {
                personagem.mostrar_status();
                println!("Pressione Enter para sair da checagem de status...");
                let mut tmp = String::new();
                io::stdin().read_line(&mut tmp).ok();
                continue; // Volta para a escolha de movimento
            }
            _ => {
                println!("{}", "Movimento inválido!".red());
                continue;
            }
        }

        passos_no_corredor += 1;

        // Chance de regenerar inimigos após alguns movimentos
        if passos_no_corredor >= 3 {
            passos_no_corredor = 0;
            let chance_reaparecer = rng.gen_bool(0.3);
            if chance_reaparecer {
                println!("{}", "Você sente novas presenças hostis ao redor...".red());
                sala3.tentar_regenerar_inimigo();
                sala4.tentar_regenerar_inimigo();
                sala2.tentar_regenerar_inimigo();
            }
        }
    }
} // exploração do castelo, movimentação entre salas, batalhas e coleta de tesouros

fn batalha(personagem: &mut Personagem, inimigo_tipo: &mut Inimigos) -> bool {
    let mut inimigo = InimigoAtivo::novo(*inimigo_tipo);

    digita_baixo(&format!("Um {:?} apareceu!", inimigo.tipo), 30);

    loop {
        println!("\n=== Sua vez ===");
        println!("(1) Ataque físico");
        println!("(2) Ataque mágico");
        println!("(3) Fugir");

        print!("Escolha uma ação: ");
        io::stdout().flush().unwrap();
        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).unwrap();
        let escolha = escolha.trim();

        match escolha {
            "1" => {
                personagem.atacar(&mut inimigo, false);
            }
            "2" => {
                if personagem.mana_atual >= 10 {
                    personagem.atacar(&mut inimigo, true);
                    personagem.mana_atual -= 10;
                } else {
                    println!("{}", "Você não tem mana suficiente!".red());
                    continue;
                }
            }
            "3" => {
                let mut rng = thread_rng();
                if rng.gen_bool(0.5) {
                    println!("{}", "Você conseguiu fugir!".yellow());
                    return false;
                } else {
                    println!("{}", "Falha ao tentar fugir!".red());
                }
            }
            _ => {
                println!("{}", "Escolha inválida!".red());
                continue;
            }
        }

        // Se o inimigo morreu pelo seu ataque, recompensa e termina a batalha
        if inimigo.vida_atual <= 0 {
            println!("{}", format!("Você derrotou o {:?}!", inimigo.tipo).green());

            let xp_ganho = inimigo.xp_dropado;
            personagem.xp_atual += xp_ganho;
            digita_baixo(&format!("Inimigo derrotado, Você ganhou {} XP!", xp_ganho), 50);

            // Subir nível enquanto tiver XP suficiente (pode subir múltiplos níveis)
            while personagem.xp_atual >= personagem.xp_para_nivel {
                personagem.xp_atual -= personagem.xp_para_nivel;
                personagem.xp_para_nivel += 50;
                personagem.level_up();
            }

            return true; // venceu
        }

        // Turno do inimigo (executado apenas se ele ainda estiver vivo)
        println!("\n=== Turno do inimigo ===");
        inimigo.atacar(personagem, false);

        if personagem.vida_atual <= 0 {
            println!("{}", "Você foi derrotado...".red());
            return false;
        }
    }
}// função principal de batalha do jogo


fn main() {
    let mut personagem = criar_personagem();
    personagem.mostrar_status();

    println!("Pressione Enter para começar a explorar...");
    let mut tmp = String::new();
    io::stdin().read_line(&mut tmp).ok();

    explorar_castelo(&mut personagem);
}
fn loot_tesouro(personagem: &mut Personagem, sala: &mut Sala) {
    if let Some(tesouro) = &sala.tesouro {
        
        digita_baixo(&format!("Um baú misterioso reluz no canto da sala: {}", tesouro), 50);
        // Ganho de XP pelo tesouro
        personagem.xp_atual += 50;


        // tratar possibilidade de subir múltiplos níveis
        while personagem.xp_atual >= personagem.xp_para_nivel {
            personagem.xp_atual -= personagem.xp_para_nivel;
            personagem.xp_para_nivel += 50; // aumenta para o próximo nível
            personagem.level_up(); // aplica aumentos de status
        }

        sala.tesouro = None; // remove o tesouro após coletar
    } else {
        digita_baixo("Não há baú nesta sala.", 30);
    }
} // esta função trata do loot do tesouro, dando xp ao personagem e removendo o tesouro da sala após coletá-lo

fn criar_personagem() -> Personagem {
    digita_baixo("Bem-vindo ao RPG em Rust!", 30);
    digita_baixo("Escolha uma classe para seu personagem:", 30);
    digita_baixo("1. Guerreiro", 50);
    digita_baixo("2. Mago", 50);
    digita_baixo("3. Arqueiro", 50);
    digita_baixo("4. Padre", 50);
    digita_baixo("5. Ladrão", 50);  
    digita_baixo("Ou deixe que o sistema escolha uma classe aleatória para você!", 20);

    println!("Digite o número da classe desejada ou 0 para aleatório:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
    let escolha: i32 = input.trim().parse().unwrap_or(0);

    let classe = match escolha {
        1 => Classes::Guerreiro,
        2 => Classes::Mago,
        3 => Classes::Arqueiro,
        4 => Classes::Padre,
        5 => Classes::Ladrão,
        _ => Classes::aleatoria(),
    };

    println!("Digite o nome do seu personagem:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
    let nome_digitado = input.trim().to_string();

    let nome = if nome_digitado.is_empty() {
        "Aventureiro Sem Nome".to_string()
    } else {
        nome_digitado
    };
    // aqui ^ basicamente seria a parte da criação do personagem, como está descrito no nome da função
    Personagem::new(nome, classe)
}