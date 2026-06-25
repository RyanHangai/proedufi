use std::io::{self, Write};

fn mensagem_boas_vindas() {
    println!("Boas vindas ao Proedufi!");
    println!("O seu app de gestao financeira");
}

fn limpa_espaco_no_texto(texto: &str) -> String {
    let trimmed = texto.trim();

    let mut resultado = String::new();
    let mut espaco_entre_letras = false;

    for ch in trimmed.chars() {
        if ch.is_whitespace() {
            if !espaco_entre_letras {
                resultado.push(' ');
                espaco_entre_letras = true;
            }
        } else {
            resultado.push(ch);
            espaco_entre_letras = false;
        }
    }

    resultado.trim_end().to_string()
}

fn valida_nome(nome: &str) -> bool {
    for ch in nome.chars() {
        if !ch.is_alphabetic() {
            println!("O nome so pode ter letras");
            return false;
        }
    }
    true
}

fn valida_email(email: &str) -> bool {
    let mut tem_arroba = false;
    let mut tem_ponto = false;

    for ch in email.chars() {
        if ch == ' ' {
            println!("Nao pode ter espacos no email");
            return false;
        }
        if ch == '@' {
            if tem_arroba {
                println!("O email nao pode ter dois \"@\" (arroba)");
                return false;
            }
            tem_arroba = true;
        }
        if ch == '.' {
            tem_ponto = true;
        }
    }

    if !tem_arroba || !tem_ponto {
        return false;
    }

    if email.starts_with('@') || email.starts_with('.') || email.ends_with('@') || email.ends_with('.') {
        return false;
    }

    true
}

fn valida_senha(senha: &str) -> bool {
    let tamanho = senha.len();

    if tamanho < 8 || tamanho > 128 {
        println!("A senha so pode ter entre 8 a 128 caracteres");
        return false;
    }

    let mut tem_maiuscula = false;
    let mut tem_minuscula = false;
    let mut tem_numero = false;
    let mut tem_especial = false;

    for ch in senha.chars() {
        if ch == ' ' {
            println!("Nao pode ter espacos na senha");
            return false;
        }
        if ch.is_uppercase() {
            tem_maiuscula = true;
        } else if ch.is_lowercase() {
            tem_minuscula = true;
        } else if ch.is_ascii_digit() {
            tem_numero = true;
        } else if ch.is_ascii_punctuation() {
            tem_especial = true;
        }
    }

    if tem_maiuscula && tem_minuscula && tem_numero && tem_especial {
        true
    } else {
        println!("A senha tem que ter maiusculas, minusculas, numeros e caracter especial (!@#%$&*)");
        false
    }
}

fn criar_conta() {
    println!("Voce escolheu criar uma conta");
    println!("[0] Para voltar");

    let mut nome = String::new();
    let mut email = String::new();
    let mut senha = String::new();

    print!("Digite seu nome: ");
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    let entrada = limpa_espaco_no_texto(&entrada);

    if entrada == "0" {
        println!("Voce Voltou!\n");
        return;
    } else if entrada.is_empty() {
        println!("Digite o seu nome primeiro!");
    } else if entrada.len() > 60 {
        println!("Nome muito extenso, digite seu nome real!");
    } else if !valida_nome(&entrada) {
        println!("O nome so pode ter letras");
    } else {
        nome = entrada;
    }

    print!("Digite seu email: ");
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    let entrada = limpa_espaco_no_texto(&entrada);

    if entrada == "0" {
        println!("Voce Voltou!\n");
        return;
    } else if entrada.is_empty() {
        println!("Digite o seu email primeiro!");
    } else if !valida_email(&entrada) {
        println!("Digite um email valido");
    } else {
        email = entrada;
    }

    print!("Digite uma senha: ");
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    let entrada = limpa_espaco_no_texto(&entrada);

    if entrada == "0" {
        println!("Voce Voltou!\n");
        return;
    } else if entrada.is_empty() {
        println!("Digite o seu email primeiro!");
    } else if !valida_senha(&entrada) {
        println!();
    } else {
        senha = entrada;
    }
}

fn main() {
    mensagem_boas_vindas();

    loop {
        println!("Escolha uma opcao:");
        println!("[1] Criar conta");
        println!("[2] Fazer login");
        println!("[3] Continuar sem conta");
        println!("[0] Sair!");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).unwrap();
        let escolha: i32 = match escolha.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Escolha uma opcao valida");
                continue;
            }
        };

        match escolha {
            1 => criar_conta(),
            2 => println!("Faca login na sua conta"),
            3 => println!("Ok!"),
            0 => {
                println!("Ate a proxima!");
                break;
            }
            _ => println!("Escolha uma opcao valida"),
        }
    }
}
