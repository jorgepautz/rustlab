use std::fs::File;
use std::io::{Write, Read};
use axum::{
    routing::{get, post}, 
    Json, 
    Router, 
    extract::{Path, Query},
    http::StatusCode
};
use serde::{Serialize, Deserialize};
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

// MANTIDA PARA ESTUDOS ANTERIORES
async fn obter_usuario_por_id_old(Path(id_recebido): Path<u32>) -> Json<PerfilUsuario> {
    println!("🔍 API acionada! Buscando dados para o ID: {}", id_recebido);

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
    parametros_recebidos: HashMap<String, String>,
}

// NOVA IMPLEMENTAÇÃO: Retorna um Result para permitir alterar o Status Code HTTP para 404
async fn obter_usuario_por_id(
    Path(id): Path<u32>,
    Query(params): Query<HashMap<String, String>>
) -> Result<Json<PerfilUsuario>, StatusCode> {
    
    println!("🔍 Buscando ID na API com controle de Status Code: {}", id);

    // Simulando uma validação de banco de dados (Se o ID for maior que 100, simulamos um 404)
    if id > 100 {
        println!("❌ ID {} não encontrado no sistema. Retornando HTTP 404.", id);
        // Retornamos o status HTTP de erro mapeado
        return Err(StatusCode::NOT_FOUND);
    }

    let usuario = PerfilUsuario {
        id,
        nome: String::from("Eric"),
        endereco: Endereco {
            cidade: String::from("Canela"),
            estado: String::from("RS"),
        },
        habilidades: vec![
            String::from("Rust"), String::from("C#"), String::from("PHP")
        ],
    };

    // Em caso de sucesso, o Axum responde implicitamente com HTTP 200 OK
    Ok(Json(usuario))
}

// --- NOVAS ESTRUTURAS PARA REQUISIÇÕES POST ---

// Adicionamos 'Deserialize' para permitir que o Serde converta o JSON vindo do cliente em Struct
#[derive(Deserialize, Debug)]
struct NovoUsuarioRequest {
    nome: String,
    cidade: String,
}

#[derive(Serialize)]
struct NovoUsuarioResponse {
    id: u32,
    nome: String,
    status: String,
}

// Nova estrutura para expor mensagens de erro detalhadas em JSON
#[derive(Serialize)]
struct ErroResponse {
    erro: String,
}

// 2. Manipulador POST com Validação de Campos Vazios
// Retorna 'Ok' com o sucesso ou 'Err' contendo o Status HTTP correspondente e o JSON explicativo
async fn criar_usuario(
    Json(payload): Json<NovoUsuarioRequest>
) -> Result<(StatusCode, Json<NovoUsuarioResponse>), (StatusCode, Json<ErroResponse>)> {
    
    println!("📥 Requisição POST recebida. Validando dados...");

    // Validação do campo 'nome' (Equivalente ao empty() do PHP ou IsNullOrEmpty do C#)
    if payload.nome.trim().is_empty() {
        println!("❌ Falha na validação: O campo 'nome' está vazio.");
        let resposta_erro = ErroResponse {
            erro: String::from("O campo 'nome' é obrigatório e não pode estar vazio."),
        };
        return Err((StatusCode::BAD_REQUEST, Json(resposta_erro)));
    }

    // Validação do campo 'cidade'
    if payload.cidade.trim().is_empty() {
        println!("❌ Falha na validação: O campo 'cidade' está vazio.");
        let resposta_erro = ErroResponse {
            erro: String::from("O campo 'cidade' é obrigatório e não pode estar vazio."),
        };
        return Err((StatusCode::BAD_REQUEST, Json(resposta_erro)));
    }

    // Fluxo de Sucesso se passar pelas validações acima
    println!("📋 Dados validados com sucesso! Nome: {}, Cidade: {}", payload.nome, payload.cidade);
    
    let resposta_sucesso = NovoUsuarioResponse {
        id: 123,
        nome: payload.nome,
        status: String::from("Criado com sucesso"),
    };

    Ok((StatusCode::CREATED, Json(resposta_sucesso)))
}

// 3. Inicialização do Servidor
async fn iniciar_servidor_web() {
    println!("\n--- [3] Iniciando Servidor Web HTTP ---");
    
    let app = Router::new()
        // Rota GET com tratamento de erro (Ex: tente /api/usuario/42 e /api/usuario/150)
        .route("/api/usuario/:id", get(obter_usuario_por_id))
        // Nova rota POST para criação de dados
        .route("/api/usuarios", post(criar_usuario));
    
    let endereço = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(endereço).await.unwrap();
    
    println!("🚀 Servidor rodando em http://{}", endereço);
    println!("💡 GET válido: http://localhost:3000/api/usuario/42");
    println!("💡 GET 404: http://localhost:3000/api/usuario/150");
    println!("🛑 Pressione CTRL+C no terminal para encerrar.");
    
    axum::serve(listener, app).await.unwrap();
}

// ==========================================
// 4. TESTE DE HASHMAP
// ==========================================
fn hashtest() {
    println!("\n--- [4] Executando Exemplo de HashMaps ---");
    let mut dados = HashMap::new();

    dados.insert(String::from("nome"), String::from("Eric"));
    dados.insert(String::from("cidade"), String::from("Canela"));
    dados.insert(String::from("cidade"), String::from("Gramado"));

    if let Some(nome) = dados.get("nome") {
        println!("O nome guardado é: {}", nome);
    }

    println!("Dicionário completo: {:?}", dados);
}

// ==========================================
// FUNÇÃO PRINCIPAL (MAIN)
// ==========================================
#[tokio::main]
async fn main() {
    println!("🧪 BEM-VINDO AO RUSTLAB 🧪");

    executar_exemplo_vetores();

    if let Err(erro) = executar_exemplo_arquivos() {
        println!("❌ Erro ao manipular arquivos: {}", erro);
    }

    hashtest();

    iniciar_servidor_web().await;
}
