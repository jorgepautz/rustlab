use std::fs::File;
use std::io::{Write, Read};
use axum::{routing::get, Json, Router};
use serde::Serialize;

// ==========================================
// 1. CENÁRIO: MANIPULAÇÃO DE VETORES E FILTROS
// ==========================================
#[derive(Debug)]
struct Usuario {
    id: u32,
    nome: String,
    ativo: bool,
}

fn executar_exemplo_vetores() {
    println!("\n--- [1] Executando Exemplo de Vetores e Filtros ---");

    let usuarios = vec![
        Usuario { id: 1, nome: String::from("Ana"), ativo: true },
        Usuario { id: 2, nome: String::from("Carlos"), ativo: false },
        Usuario { id: 3, nome: String::from("Amanda"), ativo: true },
    ];

    // O .filter() gera referências, fazendo o tipo do argumento 'u' virar '&&Usuario'
    // O Rust acessa '.ativo' desreferenciando implicitamente por baixo dos panos (Auto-Deref)
    let ativos: Vec<&Usuario> = usuarios
        .iter()
        .filter(|u| u.ativo) 
        .collect();

    println!("Usuários ativos encontrados: {:?}", ativos);
}

// ==========================================
// 2. CENÁRIO: LEITURA E ESCRITA DE ARQUIVOS
// ==========================================
fn executar_exemplo_arquivos() -> Result<(), std::io::Error> {
    println!("\n--- [2] Executando Exemplo de Arquivos ---");
    let nome_arquivo = "log_laboratorio.txt";

    // O operador '?' intercepta o erro e retorna mais cedo caso falhe (substitui try/catch)
    let mut arquivo_criado = File::create(nome_arquivo)?;
    arquivo_criado.write_all("Dados gravados com sucesso pelo laboratório Rust!".as_bytes())?;
    println!("✔ Arquivo '{}' criado e escrito.", nome_arquivo);

    let mut arquivo_leitura = File::open(nome_arquivo)?;
    let mut conteudo = String::new();
    arquivo_leitura.read_to_string(&mut conteudo)?;
    
    println!("📖 Conteúdo lido de dentro do arquivo: \"{}\"", conteudo);
    Ok(())
}

// ==========================================
// 3. CENÁRIO: API HTTP (AXUM & SERDE)
// ==========================================
#[derive(Serialize)]
struct RespostaApi {
    status: String,
    mensagem: String,
}

async fn manipulador_api() -> Json<RespostaApi> {
    let resposta = RespostaApi {
        status: String::from("sucesso"),
        mensagem: String::from("Olá! API Rust respondendo em JSON puro."),
    };
    Json(resposta)
}

async fn iniciar_servidor_web() {
    println!("\n--- [3] Iniciando Servidor Web HTTP ---");
    
    let app = Router::new().route("/api", get(manipulador_api));
    let endereço = "127.0.0.1:3000";
    
    let listener = tokio::net::TcpListener::bind(endereço).await.unwrap();
    println!("🚀 Servidor local rodando em http://{}", endereço);
    println!("💡 Dica: Abra http://localhost:3000/api no seu navegador ou use o Postman.");
    println!("🛑 Pressione CTRL+C no terminal para encerrar o servidor.");
    
    axum::serve(listener, app).await.unwrap();
}

// ==========================================
// FUNÇÃO PRINCIPAL (MAIN)
// ==========================================
// Usamos o macro #[tokio::main] para permitir que a main seja 'async' por causa da API
#[tokio::main]
async fn main() {
    println!("🧪 BEM-VINDO AO RUSTLAB 🧪");

    // Executa o Cenário 1
    executar_exemplo_vetores();

    // Executa o Cenário 2 (E trata o resultado do Result)
    if let Err(erro) = executar_exemplo_arquivos() {
        println!("❌ Erro ao manipular arquivos: {}", erro);
    }

    // Executa o Cenário 3 (Este bloco irá travar o terminal mantendo o servidor aberto)
    iniciar_servidor_web().await;
}
