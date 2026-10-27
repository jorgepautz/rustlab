use std::fs::File;
use std::io::{Write, Read};
use axum::{routing::get, Json, Router, extract::{Path, Query}};
use serde::Serialize;
use std::collections::HashMap;

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
// 1. Definição das Estruturas Aninhadas
#[derive(Serialize)]
struct Endereco {
    cidade: String,
    estado: String,
}

#[derive(Serialize)]
struct PerfilUsuario {
    id: u32,
    nome: String,
    endereco: Endereco,
    habilidades: Vec<String>,
}

// 2. O Novo Manipulador de Rota que Aceita Parâmetro
// O Axum converte automaticamente o ':id' da URL para o tipo 'u32' do argumento!
async fn obter_usuario_por_id_old(Path(id_recebido): Path<u32>) -> Json<PerfilUsuario> {
    println!("🔍 API acionada! Buscando dados para o ID: {}", id_recebido);

    // Simulando a busca de um registro com dados aninhados
    let usuario_mockado = PerfilUsuario {
        id: id_recebido,
        nome: String::from("Eric"),
        endereco: Endereco {
            cidade: String::from("Canela"),
            estado: String::from("RS"),
        },
        habilidades: vec![
            String::from("Rust"),
            String::from("C#"),
            String::from("PHP"),
            String::from("JavaScript"),
        ],
    };

    Json(usuario_mockado)
}

#[derive(Serialize)]
struct RespostaDinamica {
    id: u32,
    parametros_recebidos: HashMap<String, String>, // Nosso array associativo dinâmico no JSON
}

// O extrator 'Query' captura qualquer coisa após a '?' na URL e joga no HashMap
async fn obter_usuario_por_id(
    Path(id): Path<u32>,
    Query(params): Query<HashMap<String, String>>
) -> Json<RespostaDinamica> {
    
    println!("🔍 Buscando ID: {}", id);
    println!("📋 Filtros dinâmicos enviados na Query String: {:?}", params);

    let resposta = RespostaDinamica {
        id,
        parametros_recebidos: params, // Passamos o mapa dinâmico direto para o Serde gerar o JSON
    };

    Json(resposta)
}

// 3. Atualizando a Inicialização do Servidor
async fn iniciar_servidor_web() {
    println!("\n--- Iniciando Servidor Web HTTP ---");
    
    // Configurando a rota dinâmica usando ':id'
    let app = Router::new().route("/api/usuario/:id", get(obter_usuario_por_id));
    
    let endereço = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(endereço).await.unwrap();
    
    println!("🚀 Servidor dinâmico rodando em http://{}", endereço);
    println!("💡 Teste acessando: http://localhost:3000/api/usuario/42");
    println!("🛑 Pressione CTRL+C no terminal para encerrar.");
    
    axum::serve(listener, app).await.unwrap();
}

// 4. Teste de hashmap
fn hashtest() {
    // No PHP: $dados = [];
    // No Rust: Criamos um mapa mutável. O compilador infere os tipos com base no uso.
    let mut dados = HashMap::new();

    // No PHP: $dados["nome"] = "Eric";
    // No Rust: Usamos o método .insert()
    dados.insert(String::from("nome"), String::from("Eric"));
    dados.insert(String::from("cidade"), String::from("Canela"));

    // Modificando um valor existente (Sobrescreve a chave anterior)
    // No PHP: $dados["cidade"] = "Gramado";
    dados.insert(String::from("cidade"), String::from("Gramado"));

    // Lendo um valor com segurança
    // No Rust, acessar uma chave pode retornar 'None' se ela não existir.
    if let Some(nome) = dados.get("nome") {
        println!("O nome guardado é: {}", nome);
    }

    // Exibindo toda a estrutura dinâmica
    println!("Dicionário completo: {:?}", dados);
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

    hashtest();

    // Executa o Cenário 3 (Este bloco irá travar o terminal mantendo o servidor aberto)
    iniciar_servidor_web().await;
}
