# Prompt para Mejorar el Codigo Base

Copia y pega el contenido del bloque de abajo en un asistente de IA (Claude, ChatGPT)
para obtener un ZIP con el proyecto completo y arrancable.

Si preferis trabajar en tu editor con un agente local (Claude Code, Cursor, Copilot), usa `AGENTS.md` en vez de este archivo: dice lo mismo pero para que escriba los archivos en disco.

## Las dos reglas que no se negocian

1. **Completa el boilerplate.** Todo lo que el proyecto necesita para compilar y arrancar: manifiesto de dependencias, punto de entrada, configuracion, capa de interfaz, y las capas del patron arquitectonico declarado. Eso es andamiaje y es tu trabajo.
2. **NO resuelvas el reto.** Los entregables de las fases son el trabajo de la persona. El hueco pedagogico se deja como esta: el proyecto arranca, pero lo que el reto pide implementar NO esta implementado.

Dicho de otra forma: si algo impide compilar, arreglalo. Si algo es logica de negocio incompleta, validaciones ausentes, un secreto hardcodeado o un patron mejorable, dejalo exactamente como esta — es lo que la persona tiene que encontrar.

## Lo que le falta a este proyecto

Esto NO lo tenes que adivinar: salio de comparar el proyecto contra la arquitectura declarada del reto y de un analisis estatico del codigo. Completalo TODO.

### Boilerplate del stack que falta

Sin esto no compila ni arranca. Es andamiaje, no toca nada de lo pedagogico:

- **Punto de entrada del stack elegido** — Sin un punto de entrada reconocible, el runtime no tiene por donde arrancar la aplicacion.
- **Capa de interfaz (controller/handler)** — Sin una capa de interfaz explicita, no hay forma de invocar la logica de negocio desde afuera del proceso.

## Como saber que terminaste

```bash
el comando de build o arranque canonico del stack elegido
```

Ese comando corriendo sin errores es la definicion de "listo".

---

```
## Briefing del reto (autoridad)
Este bloque manda sobre los archivos adjuntos. El stack y el rol salen de AQUÍ, no de un topic genérico ni de markdown placeholder.

### Contexto técnico original
Build a REST API with Rust, Actix Web and Diesel ORM

### Reto
- Tema: rust-actix-web
- Seniority: junior-l1
- Tipo: practical
- Título: Desarrollo de una API REST en Rust con Actix Web y Diesel ORM
- Tiempo estimado: 8 horas

### Fases (trabajo del HUMANO — PROHIBIDO completarlas)
No implementes estos entregables. Dejalos como hueco pedagógico. El asistente solo materializa el proyecto arrancable para que el participante pueda trabajar.
- Fase 1: Registro de solicitudes — objetivo: Implementar la funcionalidad para registrar solicitudes de préstamo con validación de campos y manejo de duplicados. — entregable (NO resolver): Servicio que registra solicitudes de préstamo con validación y manejo de duplicados.
- Fase 2: Persistencia de datos — objetivo: Implementar la persistencia de las solicitudes de préstamo en una base de datos PostgreSQL. — entregable (NO resolver): Servicio que registra y persiste solicitudes de préstamo en una base de datos PostgreSQL.
- Fase 3: Manejo de errores y recuperación — objetivo: Implementar el manejo de errores y la recuperación ante posibles fallos en el sistema. — entregable (NO resolver): Servicio que registra, persiste y maneja errores en las solicitudes de préstamo de forma idempotente y consistente.

Eres un asistente experto en análisis, corrección y generación de archivos de cualquier tipo:
código fuente, documentación, hojas de cálculo, documentos Word, configuraciones, entre otros.
Voy a enviarte una cadena de texto que contiene uno o más archivos. Cada archivo está delimitado por un marcador con el siguiente formato:
// === ARCHIVO: ruta/del/archivo.extension ===
o también puede aparecer como:
## === ARCHIVO: ruta/del/archivo.extension ===
Lo que sigue al marcador puede ser:

El contenido real del archivo (código, texto, YAML, etc.)
Una descripción en lenguaje natural de lo que debe contener el archivo


TU TAREA
PASO 0 — ¿Esto es un proyecto o una carcasa?
Antes de extraer archivos, leé el Briefing (si está) y diagnosticá el adjunto.

Es CARCASA si ocurre CUALQUIERA de estas:
- No hay manifiesto de dependencias del stack del briefing (manifest.json de VTEX IO / package.json / pom.xml / build.gradle / requirements.txt / go.mod / *.tf / *.csproj, según corresponda)
- Hay un "binario" que en realidad es un comentario ("no puede ser mostrado como texto plano", placeholder .fig/.docx vacío)
- Los markdowns ya completan entregables de fases posteriores ("se implementó fade-in", lista de áreas ya resuelta)

Si es CARCASA:
- MATERIALIZÁ un proyecto que arranca en el stack del briefing (VTEX IO Store Framework, Angular, Terraform, pytest, Nest, etc.). Incluí manifiesto, punto de entrada y capa de interfaz reales.
- NO copies los markdowns de "solución" como si fueran el producto. Son ruido de generación.
- NO resuelvas las fases del briefing (están marcadas PROHIBIDO). Dejá el hueco pedagógico: el flujo existe, las microinteracciones/calidad/infra que el reto pide NO están hechas.
- Después seguí al PASO 5 (ZIP).

Si es un proyecto REAL (manifiesto + código que compila o arranca):
- Seguí PASO 1 en adelante. 🔴 compilación sí. 🟡 pedagógico no.

PASO 1 — Detección y extracción
Identifica todos los archivos presentes en la cadena. Para cada archivo extrae:

Su ruta completa (ej: src/main/java/com/pragma/Service.java)
Su contenido o descripción

PASO 2 — Clasificación por tipo
Clasifica cada archivo en una de estas categorías:
A) Código fuente (Java, Python, TypeScript, JavaScript, Kotlin, etc.)
B) Configuración / documentación (YAML, properties, Markdown, JSON, txt, etc.)
C) Excel (.xlsx, .xls, .csv)
D) Word (.docx, .doc)
E) Otro tipo de archivo binario o especial
PASO 3 — Clasificación de errores en código fuente

Objetivo prioritario: que el proyecto compile. No corrijas flujo de negocio ni lógica funcional.

Antes de modificar cualquier archivo de código fuente, clasifica cada problema encontrado en una de estas dos categorías:
🔴 ERROR DE COMPILACIÓN — corregir siempre
Son errores que impiden que el proyecto arranque, sin valor pedagógico:

Import faltante o incorrecto
Clase, método o variable referenciada que no existe en ningún archivo del proyecto
Error de sintaxis
Anotación con atributos inválidos
Dependencia ausente en pom.xml, package.json, etc.
Archivo referenciado que no existe y debe ser creado con implementación mínima

→ CORREGIR estos errores.
🟡 PROBLEMA FUNCIONAL O DE CALIDAD — preservar siempre
Son problemas que no impiden compilar. Pueden ser intencionales para el aprendizaje:

Clave secreta hardcodeada ("secret", "password123")
API deprecada que funciona pero tiene reemplazo moderno
Lógica de negocio incorrecta o incompleta
Código redundante o de baja legibilidad
Falta de validaciones en flujo de negocio
Patrones de diseño incorrectos pero funcionales
Concurrencia no segura
Configuración funcional pero no óptima

→ PRESERVAR tal cual. No corregir, no mejorar, no comentar.
PASO 4 — Procesamiento según tipo de archivo
Tipo A — Código fuente
Aplica únicamente las correcciones clasificadas como 🔴 ERROR DE COMPILACIÓN.
No alteres ningún elemento clasificado como 🟡 PROBLEMA FUNCIONAL O DE CALIDAD.
Si falta un archivo referenciado, créalo con la implementación mínima necesaria para compilar.
Tipo B — Configuración / documentación
Extrae el contenido tal cual, sin modificaciones salvo errores evidentes de sintaxis
(ej: YAML mal indentado).
Tipo C — Excel (.xlsx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un archivo Excel funcional con:

Fila de encabezados en negrita con color de fondo distintivo
Columnas con ancho ajustado al contenido
Tipos de dato correctos por columna
Validaciones si la descripción lo indica
Hojas nombradas descriptivamente si hay más de una
Filas de ejemplo si no hay datos reales

Tipo D — Word (.docx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un documento Word funcional con:

Estilos de título (Título 1, Título 2) para jerarquía de secciones
Fuente legible (Calibri o equivalente), tamaño 11-12pt para cuerpo
Márgenes estándar
Tabla de contenido si tiene múltiples secciones
Tablas con encabezados en negrita si aplica

Tipo E — Otro
Genera el archivo con el contenido o estructura más apropiada según la descripción.
PASO 5 — Exportación en ZIP
Empaqueta todos los archivos en un único archivo ZIP descargable respetando exactamente
la estructura de rutas indicada por los marcadores.
El ZIP debe incluir:

Archivos de código con únicamente los errores de compilación corregidos
Archivos de configuración y documentación sin cambios
Archivos nuevos creados para resolver dependencias de compilación faltantes
Archivos Excel y Word generados desde descripción

IMPORTANTE: El ZIP debe estar listo para descargar al finalizar. No preguntes si el usuario
quiere generarlo. Simplemente genera el archivo y proporciona el enlace de descarga; No debes desplegar en el chat el resumen de lo que arreglaste al Zip, solo entregalo.

REGLAS IMPORTANTES

No omitas ningún archivo aunque no tenga errores ni modificaciones
Respeta los nombres y rutas exactas indicadas por los marcadores
Si un archivo no tiene marcador claro, infiere el nombre desde su contenido
Si la cadena contiene solo documentación, placeholders o binarios fake, NO la reproduzcas:
aplicá PASO 0 (materializar el proyecto del briefing). Reproducir la carcasa es un fallo.
No agregues texto después del enlace de descarga del ZIP
No preguntes si el usuario quiere el ZIP: simplemente generalo siempre
Si detectas que falta un archivo de configuración necesario para compilar
(pom.xml, package.json, requirements.txt, build.gradle, etc.), créalo e inclúyelo
inferiendo su contenido desde los imports y frameworks detectados en el código
Nunca corrijas problemas 🟡 aunque parezcan obvios o fáciles de mejorar.
El participante que recibirá este proyecto los debe encontrar y resolver él mismo.


INPUT
Aquí está la cadena con los archivos:

// === ARCHIVO: package.json ===
{
  "name": "loan-management-api",
  "version": "1.0.0",
  "description": "API REST para gestión de solicitudes de préstamo en plataforma de préstamos en línea",
  "main": "src/main.rs",
  "scripts": {
    "build": "cargo build --release",
    "dev": "cargo run",
    "test": "cargo test"
  },
  "keywords": ["rust", "actix-web", "loan", "api", "rest"],
  "author": "",
  "license": "MIT",
  "dependencies": {
    "actix-web": "4.5.1",
    "diesel": "2.1.0",
    "diesel_migrations": "2.1.0",
    "dotenv": "0.15.0",
    "serde": "1.0.196",
    "validator": "2.1.0",
    "thiserror": "1.0.56",
    "tokio": "1.36.0",
    "postgres": "0.19.7"
  },
  "devDependencies": {},
  "engines": {
    "node": ">=18.0.0"
  }
}

// === ARCHIVO: src/domain/models/loan_request.rs ===
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LoanStatus {
    Pending,
    Approved,
    Rejected,
}

impl Default for LoanStatus {
    fn default() -> Self {
        LoanStatus::Pending
    }
}

impl std::fmt::Display for LoanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoanStatus::Pending => write!(f, "pending"),
            LoanStatus::Approved => write!(f, "approved"),
            LoanStatus::Rejected => write!(f, "rejected"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoanRequest {
    #[validate(length(min = 2, max = 100, message = "El nombre debe tener entre 2 y 100 caracteres"))]
    pub applicant_name: String,
    
    #[validate(range(min = 100, max = 10000, message = "El monto debe estar entre 100 y 10000"))]
    pub amount: f64,
    
    #[validate(range(min = 6, max = 60, message = "El plazo debe estar entre 6 y 60 meses"))]
    pub term_months: i32,
    
    #[validate(uuid(message = "El identificador debe ser un UUID válido"), required(message = "El identificador es requerido"))]
    pub request_id: Option<String>,
    
    pub status: LoanStatus,
    
    pub created_at: Option<DateTime<Utc>>,
    
    pub updated_at: Option<DateTime<Utc>>,
}

impl LoanRequest {
    pub fn new(
        applicant_name: String,
        amount: f64,
        term_months: i32,
        request_id: String,
    ) -> Result<Self, ValidationError> {
        let mut request = LoanRequest {
            applicant_name,
            amount,
            term_months,
            request_id: Some(request_id),
            status: LoanStatus::default(),
            created_at: None,
            updated_at: None,
        };
        
        request.validate()?;
        
        if request.amount < 100.0 || request.amount > 10000.0 {
            return Err(ValidationError::AmountOutOfRange);
        }
        
        if request.term_months < 6 || request.term_months > 60 {
            return Err(ValidationError::TermOutOfRange);
        }
        
        Ok(request)
    }
    
    pub fn update_status(&mut self, status: LoanStatus) {
        self.status = status;
        self.updated_at = Some(Utc::now());
    }
    
    pub fn is_valid_amount(&self) -> bool {
        self.amount >= 100.0 && self.amount <= 10000.0
    }
    
    pub fn is_valid_term(&self) -> bool {
        self.term_months >= 6 && self.term_months <= 60
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanRequestResponse {
    pub id: String,
    pub applicant_name: String,
    pub amount: f64,
    pub term_months: i32,
    pub request_id: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<LoanRequest> for LoanRequestResponse {
    fn from(req: LoanRequest) -> Self {
        LoanRequestResponse {
            id: req.request_id.clone().unwrap_or_default(),
            applicant_name: req.applicant_name,
            amount: req.amount,
            term_months: req.term_months,
            request_id: req.request_id.unwrap_or_default(),
            status: req.status.to_string(),
            created_at: req.created_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| Utc::now().to_rfc3339()),
            updated_at: req.updated_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    AmountOutOfRange,
    TermOutOfRange,
    InvalidName,
    MissingRequestId,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::AmountOutOfRange => {
                write!(f, "El monto debe estar entre 100 y 10000 unidades")
            }
            ValidationError::TermOutOfRange => {
                write!(f, "El plazo debe estar entre 6 y 60 meses")
            }
            ValidationError::InvalidName => {
                write!(f, "El nombre del solicitante es inválido")
            }
            ValidationError::MissingRequestId => {
                write!(f, "El identificador de solicitud es requerido")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loan_request_creation_with_valid_data() {
        let request = LoanRequest::new(
            "Juan Pérez".to_string(),
            5000.0,
            24,
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        );
        
        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.applicant_name, "Juan Pérez");
        assert_eq!(req.amount, 5000.0);
        assert_eq!(req.term_months, 24);
    }
    
    #[test]
    fn test_loan_request_rejects_invalid_amount_below_minimum() {
        let request = LoanRequest::new(
            "Juan Pérez".to_string(),
            50.0,
            24,
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        );
        
        assert!(request.is_err());
    }
    
    #[test]
    fn test_loan_request_rejects_invalid_amount_above_maximum() {
        let request = LoanRequest::new(
            "Juan Pérez".to_string(),
            15000.0,
            24,
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        );
        
        assert!(request.is_err());
    }
    
    #[test]
    fn test_loan_request_rejects_invalid_term_below_minimum() {
        let request = LoanRequest::new(
            "Juan Pérez".to_string(),
            5000.0,
            3,
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        );
        
        assert!(request.is_err());
    }
    
    #[test]
    fn test_loan_request_rejects_invalid_term_above_maximum() {
        let request = LoanRequest::new(
            "Juan Pérez".to_string(),
            5000.0,
            72,
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        );
        
        assert!(request.is_err());
    }
    
    #[test]
    fn test_loan_request_status_update() {
        let mut request = LoanRequest::new(
            "Juan Pérez".to_string(),
            5000.0,
            24,
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        ).unwrap();
        
        request.update_status(LoanStatus::Approved);
        
        assert_eq!(request.status, LoanStatus::Approved);
        assert!(request.updated_at.is_some());
    }
}

// === ARCHIVO: src/domain/repositories/loan_repository.rs ===
use crate::domain::models::loan_request::LoanRequest;
use crate::interfaces::controllers::errors::RepositoryError;
use async_trait::async_trait;

#[async_trait]
pub trait LoanRepository: Send + Sync {
    async fn create(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError>;
    
    async fn find_by_id(&self, id: &str) -> Result<Option<LoanRequest>, RepositoryError>;
    
    async fn find_by_request_id(&self, request_id: &str) -> Result<Option<LoanRequest>, RepositoryError>;
    
    async fn find_all(&self) -> Result<Vec<LoanRequest>, RepositoryError>;
    
    async fn update(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError>;
    
    async fn delete(&self, id: &str) -> Result<bool, RepositoryError>;
    
    async fn exists_by_request_id(&self, request_id: &str) -> Result<bool, RepositoryError>;
}

pub mod async_trait {
    pub use async_trait::async_trait;
}

#[derive(Debug, Clone)]
pub struct LoanFilter {
    pub status: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub applicant_name_contains: Option<String>,
}

impl Default for LoanFilter {
    fn default() -> Self {
        LoanFilter {
            status: None,
            min_amount: None,
            max_amount: None,
            applicant_name_contains: None,
        }
    }
}

impl LoanFilter {
    pub fn with_status(status: String) -> Self {
        LoanFilter {
            status: Some(status),
            ..Default::default()
        }
    }
    
    pub fn with_amount_range(min: f64, max: f64) -> Self {
        LoanFilter {
            min_amount: Some(min),
            max_amount: Some(max),
            ..Default::default()
        }
    }
    
    pub fn with_applicant_name(name: String) -> Self {
        LoanFilter {
            applicant_name_contains: Some(name),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loan_filter_default() {
        let filter = LoanFilter::default();
        
        assert!(filter.status.is_none());
        assert!(filter.min_amount.is_none());
        assert!(filter.max_amount.is_none());
        assert!(filter.applicant_name_contains.is_none());
    }
    
    #[test]
    fn test_loan_filter_with_status() {
        let filter = LoanFilter::with_status("approved".to_string());
        
        assert!(filter.status.is_some());
        assert_eq!(filter.status.unwrap(), "approved");
    }
    
    #[test]
    fn test_loan_filter_with_amount_range() {
        let filter = LoanFilter::with_amount_range(100.0, 5000.0);
        
        assert!(filter.min_amount.is_some());
        assert!(filter.max_amount.is_some());
        assert_eq!(filter.min_amount.unwrap(), 100.0);
        assert_eq!(filter.max_amount.unwrap(), 5000.0);
    }
    
    #[test]
    fn test_loan_filter_with_applicant_name() {
        let filter = LoanFilter::with_applicant_name("Juan".to_string());
        
        assert!(filter.applicant_name_contains.is_some());
        assert_eq!(filter.applicant_name_contains.unwrap(), "Juan");
    }
}

// === ARCHIVO: src/interfaces/controllers/loan_controller.rs ===
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::loan_request::LoanRequest;
use crate::domain::repositories::loan_repository::LoanRepository;
use crate::interfaces::dto::loan_dto::{CreateLoanRequestDto, LoanResponseDto, UpdateLoanStatusDto, LoanListResponseDto};
use crate::interfaces::controllers::errors::{ApiError, ApiResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct PathParams {
    pub id: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::<String> {
        success: true,
        data: Some("API de gestión de préstamos operativa".to_string()),
        message: None,
    })
}

pub async fn create_loan(
    pool: web::Data<crate::config::database::DbPool>,
    dto: web::Json<CreateLoanRequestDto>,
) -> Result<impl Responder, ApiError> {
    dto.validate().map_err(|e| ApiError::ValidationError(e.to_string()))?;
    
    let loan_request = LoanRequest::new(
        dto.applicant_name.clone(),
        dto.amount,
        dto.term_months,
        dto.request_id.clone(),
    ).map_err(|e| ApiError::ValidationError(e.to_string()))?;
    
    let repository = pool.get_repository();
    
    let exists = repository.exists_by_request_id(&dto.request_id).await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;
    
    if exists {
        return Err(ApiError::DuplicateResource(
            "Ya existe una solicitud con este identificador".to_string()
        ));
    }
    
    let created = repository.create(loan_request).await
        .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
    
    let response = LoanResponseDto::from(created);
    
    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Solicitud de préstamo creada exitosamente".to_string()),
    }))
}

pub async fn get_loan(
    pool: web::Data<crate::config::database::DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    
    let repository = pool.get_repository();
    
    let loan = repository.find_by_id(&id).await
        .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
    
    match loan {
        Some(req) => {
            let response = LoanResponseDto::from(req);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(response),
                message: None,
            }))
        }
        None => Err(ApiError::NotFound(
            "Solicitud de préstamo no encontrada".to_string()
        )),
    }
}

pub async fn get_all_loans(
    pool: web::Data<crate::config::database::DbPool>,
) -> Result<impl Responder, ApiError> {
    let repository = pool.get_repository();
    
    let loans = repository.find_all().await
        .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
    
    let responses: Vec<LoanResponseDto> = loans.into_iter()
        .map(LoanResponseDto::from)
        .collect();
    
    let total = responses.len();
    
    Ok(HttpResponse::Ok().json(LoanListResponseDto {
        success: true,
        data: responses,
        total,
        message: Some("Lista de solicitudes de préstamo".to_string()),
    }))
}

pub async fn update_loan_status(
    pool: web::Data<crate::config::database::DbPool>,
    path: web::Path<String>,
    dto: web::Json<UpdateLoanStatusDto>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    dto.validate().map_err(|e| ApiError::ValidationError(e.to_string()))?;
    
    let repository = pool.get_repository();
    
    let mut loan = repository.find_by_id(&id).await
        .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
    
    match loan {
        Some(mut req) => {
            let new_status = crate::domain::models::loan_request::LoanStatus::from(dto.status.clone());
            req.update_status(new_status);
            
            let updated = repository.update(req).await
                .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
            
            let response = LoanResponseDto::from(updated);
            
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(response),
                message: Some("Estado de solicitud actualizado".to_string()),
            }))
        }
        None => Err(ApiError::NotFound(
            "Solicitud de préstamo no encontrada".to_string()
        )),
    }
}

pub async fn delete_loan(
    pool: web::Data<crate::config::database::DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    
    let repository = pool.get_repository();
    
    let deleted = repository.delete(&id).await
        .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
    
    if deleted {
        Ok(HttpResponse::Ok().json(ApiResponse::<String> {
            success: true,
            data: None,
            message: Some("Solicitud de préstamo eliminada".to_string()),
        }))
    } else {
        Err(ApiError::NotFound(
            "Solicitud de préstamo no encontrada".to_string()
        ))
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/loans")
            .route("", web::post().to(create_loan))
            .route("", web::get().to(get_all_loans))
            .route("/{id}", web::get().to(get_loan))
            .route("/{id}/status", web::put().to(update_loan_status))
            .route("/{id}", web::delete().to(delete_loan))
    );
}

// === ARCHIVO: src/interfaces/dto/loan_dto.rs ===
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::domain::models::loan_request::LoanRequest;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateLoanRequestDto {
    #[validate(length(min = 2, max = 100, message = "El nombre debe tener entre 2 y 100 caracteres"))]
    pub applicant_name: String,
    
    #[validate(range(min = 100, max = 10000, message = "El monto debe estar entre 100 y 10000"))]
    pub amount: f64,
    
    #[validate(range(min = 6, max = 60, message = "El plazo debe estar entre 6 y 60 meses"))]
    pub term_months: i32,
    
    #[validate(uuid(message = "Debe ser un UUID válido"))]
    pub request_id: String,
}

impl CreateLoanRequestDto {
    pub fn to_entity(self) -> Result<LoanRequest, String> {
        LoanRequest::new(
            self.applicant_name,
            self.amount,
            self.term_months,
            self.request_id,
        ).map_err(|e| e.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateLoanStatusDto {
    #[validate(length(min = 1, message = "El estado es requerido"))]
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanResponseDto {
    pub id: String,
    pub applicant_name: String,
    pub amount: f64,
    pub term_months: i32,
    pub request_id: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<LoanRequest> for LoanResponseDto {
    fn from(req: LoanRequest) -> Self {
        LoanResponseDto {
            id: req.request_id.clone().unwrap_or_default(),
            applicant_name: req.applicant_name,
            amount: req.amount,
            term_months: req.term_months,
            request_id: req.request_id.unwrap_or_default(),
            status: req.status.to_string(),
            created_at: req.created_at
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            updated_at: req.updated_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T, message: Option<String>) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            message,
        }
    }
    
    pub fn error(message: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanListResponseDto {
    pub success: bool,
    pub data: Vec<LoanResponseDto>,
    pub total: usize,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoanQueryParams {
    #[validate(range(min = 0, message = "El monto mínimo debe ser positivo"))]
    pub min_amount: Option<f64>,
    
    #[validate(range(min = 0, message = "El monto máximo debe ser positivo"))]
    pub max_amount: Option<f64>,
    
    pub status: Option<String>,
    
    #[validate(length(min = 2, max = 100, message = "El nombre debe tener entre 2 y 100 caracteres"))]
    pub applicant_name: Option<String>,
}

impl Default for LoanQueryParams {
    fn default() -> Self {
        LoanQueryParams {
            min_amount: None,
            max_amount: None,
            status: None,
            applicant_name: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_loan_request_dto_to_entity_valid() {
        let dto = CreateLoanRequestDto {
            applicant_name: "Juan Pérez".to_string(),
            amount: 5000.0,
            term_months: 24,
            request_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        };
        
        let result = dto.to_entity();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_create_loan_request_dto_to_entity_invalid_amount() {
        let dto = CreateLoanRequestDto {
            applicant_name: "Juan Pérez".to_string(),
            amount: 50.0,
            term_months: 24,
            request_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        };
        
        let result = dto.to_entity();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_api_response_success() {
        let response = ApiResponse::success("test data", Some("Success message".to_string()));
        
        assert!(response.success);
        assert!(response.data.is_some());
        assert!(response.message.is_some());
    }
    
    #[test]
    fn test_api_response_error() {
        let response = ApiResponse::<String>::error("Error message".to_string());
        
        assert!(!response.success);
        assert!(response.data.is_none());
        assert!(response.message.is_some());
    }
}

// === ARCHIVO: src/interfaces/controllers/errors.rs ===
use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    ValidationError(String),
    NotFound(String),
    DuplicateResource(String),
    Unauthorized(String),
    InternalError(String),
    DatabaseError(String),
    RepositoryError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::ValidationError(msg) => write!(f, "Error de validación: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Recurso no encontrado: {}", msg),
            ApiError::DuplicateResource(msg) => write!(f, "Recurso duplicado: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "No autorizado: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Error interno: {}", msg),
            ApiError::DatabaseError(msg) => write!(f, "Error de base de datos: {}", msg),
            ApiError::RepositoryError(msg) => write!(f, "Error de repositorio: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::ValidationError(_) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "VALIDATION_ERROR".to_string(),
                    message: self.to_string(),
                    status: 400,
                })
            }
            ApiError::NotFound(_) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: "NOT_FOUND".to_string(),
                    message: self.to_string(),
                    status: 404,
                })
            }
            ApiError::DuplicateResource(_) => {
                HttpResponse::Conflict().json(ErrorResponse {
                    error: "DUPLICATE_RESOURCE".to_string(),
                    message: self.to_string(),
                    status: 409,
                })
            }
            ApiError::Unauthorized(_) => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "UNAUTHORIZED".to_string(),
                    message: self.to_string(),
                    status: 401,
                })
            }
            ApiError::InternalError(_) | ApiError::DatabaseError(_) | ApiError::RepositoryError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "INTERNAL_ERROR".to_string(),
                    message: self.to_string(),
                    status: 500,
                })
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status: u16,
}

impl ErrorResponse {
    pub fn new(error: String, message: String, status: u16) -> Self {
        ErrorResponse {
            error,
            message,
            status,
        }
    }
    
    pub fn validation(message: String) -> Self {
        ErrorResponse::new("VALIDATION_ERROR".to_string(), message, 400)
    }
    
    pub fn not_found(message: String) -> Self {
        ErrorResponse::new("NOT_FOUND".to_string(), message, 404)
    }
    
    pub fn duplicate(message: String) -> Self {
        ErrorResponse::new("DUPLICATE_RESOURCE".to_string(), message, 409)
    }
    
    pub fn internal(message: String) -> Self {
        ErrorResponse::new("INTERNAL_ERROR".to_string(), message, 500)
    }
}

#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
    Duplicate(String),
    ConnectionError(String),
    QueryError(String),
    TransactionError(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound(msg) => write!(f, "Recurso no encontrado: {}", msg),
            RepositoryError::Duplicate(msg) => write!(f, "Recurso duplicado: {}", msg),
            RepositoryError::ConnectionError(msg) => write!(f, "Error de conexión: {}", msg),
            RepositoryError::QueryError(msg) => write!(f, "Error de consulta: {}", msg),
            RepositoryError::TransactionError(msg) => write!(f, "Error de transacción: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

impl From<diesel::result::Error> for RepositoryError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => {
                RepositoryError::NotFound("Registro no encontrado".to_string())
            }
            diesel::result::Error::DatabaseError(_, info) => {
                RepositoryError::QueryError(info.message().to_string())
            }
            _ => RepositoryError::QueryError(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_response_validation() {
        let response = ErrorResponse::validation("Campo inválido".to_string());
        
        assert_eq!(response.error, "VALIDATION_ERROR");
        assert_eq!(response.status, 400);
    }
    
    #[test]
    fn test_error_response_not_found() {
        let response = ErrorResponse::not_found("Recurso no encontrado".to_string());
        
        assert_eq!(response.error, "NOT_FOUND");
        assert_eq!(response.status, 404);
    }
    
    #[test]
    fn test_error_response_duplicate() {
        let response = ErrorResponse::duplicate("Recurso duplicado".to_string());
        
        assert_eq!(response.error, "DUPLICATE_RESOURCE");
        assert_eq!(response.status, 409);
    }
    
    #[test]
    fn test_error_response_internal() {
        let response = ErrorResponse::internal("Error interno".to_string());
        
        assert_eq!(response.error, "INTERNAL_ERROR");
        assert_eq!(response.status, 500);
    }
    
    #[test]
    fn test_api_error_display() {
        let error = ApiError::ValidationError("Campo requerido".to_string());
        let display = format!("{}", error);
        
        assert!(display.contains("Error de validación"));
        assert!(display.contains("Campo requerido"));
    }
    
    #[test]
    fn test_repository_error_display() {
        let error = RepositoryError::NotFound("ID no encontrado".to_string());
        let display = format!("{}", error);
        
        assert!(display.contains("Recurso no encontrado"));
    }
}

// === ARCHIVO: Cargo.toml ===
[package]
name = "loan-management-api"
version = "1.0.0"
edition = "2021"
description = "API REST para gestión de solicitudes de préstamo en plataforma de préstamos en línea"
license = "MIT"

[dependencies]
actix-web = "4.5.1"
actix-rt = "2.9.1"
diesel = { version = "2.1.0", features = ["postgres", "r2d2", "chrono"] }
diesel_migrations = "2.1.0"
dotenv = "0.15.0"
serde = { version = "1.0.196", features = ["derive"] }
serde_json = "1.0.111"
validator = { version = "2.1.0", features = ["derive"] }
thiserror = "1.0.56"
tokio = { version = "1.36.0", features = ["full"] }
postgres = "0.19.7"
r2d2 = "0.8.10"
r2d2_diesel = "1.0.0"
async-trait = "0.1.77"
chrono = { version = "0.4.31", features = ["serde"] }
uuid = { version = "1.7.0", features = ["v4", "serde"] }

[lib]
path = "src/lib.rs"

[[bin]]
name = "loan-management-api"
path = "src/main.rs"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1

// === ARCHIVO: src/main.rs ===
use actix_web::{web, App, HttpServer, middleware};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

mod domain;
mod application;
mod infrastructure;
mod interfaces;
mod config;

use config::database::establish_connection;
use interfaces::controllers::loan_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let bind_address = env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    
    println!("Iniciando servidor en {}", bind_address);
    println!("Conectando a base de datos: {}", database_url);
    
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(10)
        .min_idle(Some(2))
        .max_lifetime(std::time::Duration::from_secs(1800))
        .idle_timeout(std::time::Duration::from_secs(600))
        .test_on_check_out(true)
        .build(connection_manager)
        .expect("Failed to create pool");
    
    let pool_clone = pool.clone();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool_clone.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(loan_controller::configure)
            .route("/health", web::get().to(health_check))
    })
    .bind(&bind_address)?
    .run()
    .await
}

async fn health_check() -> actix_web::Result<impl actix_web::Responder> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "loan-management-api",
        "version": "1.0.0"
    })))
}

// === ARCHIVO: src/infrastructure/persistence/diesel_loan_repository.rs ===
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{PgConnection, Insertable, Queryable};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::models::loan_request::{LoanRequest, LoanStatus};
use crate::domain::repositories::loan_repository::{LoanRepository, RepositoryError};
use crate::infrastructure::persistence::schema::loan_requests;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Queryable, Insertable, AsChangeset, Clone, Debug)]
#[diesel(table_name = loan_requests)]
#[diesel(treat_none_as_default = false)]
pub struct LoanRequestEntity {
    pub id: String,
    pub request_id: String,
    pub applicant_name: String,
    pub amount: f64,
    pub term_months: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<LoanRequest> for LoanRequestEntity {
    fn from(req: LoanRequest) -> Self {
        let now = Utc::now();
        LoanRequestEntity {
            id: req.id().to_string(),
            request_id: req.request_id().to_string(),
            applicant_name: req.applicant_name().to_string(),
            amount: req.amount(),
            term_months: req.term_months(),
            status: req.status().to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<LoanRequestEntity> for LoanRequest {
    fn from(entity: LoanRequestEntity) -> Self {
        let status = LoanStatus::from(entity.status.as_str());
        LoanRequest::restore(
            entity.id,
            entity.request_id,
            entity.applicant_name,
            entity.amount,
            entity.term_months,
            status,
            entity.created_at,
            entity.updated_at,
        )
    }
}

pub struct DieselLoanRepository {
    pool: DbPool,
}

impl DieselLoanRepository {
    pub fn new(pool: DbPool) -> Self {
        DieselLoanRepository { pool }
    }

    fn get_connection(&self) -> Result<DbConnection, RepositoryError> {
        self.pool
            .get()
            .map_err(|e| RepositoryError::ConnectionError(e.to_string()))
    }

    fn map_diesel_error(e: diesel::result::Error) -> RepositoryError {
        match e {
            diesel::result::Error::NotFound => {
                RepositoryError::NotFound("Loan request not found".to_string())
            }
            diesel::result::Error::DatabaseError(_, _) => {
                RepositoryError::DatabaseError(e.to_string())
            }
            diesel::result::Error::QueryBuilderError(_) => {
                RepositoryError::QueryError(e.to_string())
            }
            _ => RepositoryError::QueryError(e.to_string()),
        }
    }
}

#[async_trait]
impl LoanRepository for DieselLoanRepository {
    async fn create(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        let pool = self.pool.clone();
        
        let result = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let entity = LoanRequestEntity::from(loan_request);
            
            diesel::insert_into(loan_requests::table)
                .values(&entity)
                .execute(&conn)
                .map_err(|e| {
                    if let diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) = e {
                        RepositoryError::Duplicate(format!("Loan request with id {} already exists", entity.request_id))
                    } else {
                        RepositoryError::DatabaseError(e.to_string())
                    }
                })?;
            
            Ok::<_, RepositoryError>(entity)
        })
        .await
        .map_err(|e| RepositoryError::InternalError(e.to_string()))??;
        
        Ok(LoanRequest::from(result))
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        let pool = self.pool.clone();
        let id = id.to_string();
        
        let result = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let entity = loan_requests::table
                .filter(loan_requests::id.eq(&id))
                .first::<LoanRequestEntity>(&conn)
                .optional()
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(entity)
        })
        .await
        .map_err(|e| RepositoryError::InternalError(e.to_string()))??;
        
        Ok(result.map(LoanRequest::from))
    }

    async fn find_by_request_id(&self, request_id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        let pool = self.pool.clone();
        let request_id = request_id.to_string();
        
        let result = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let entity = loan_requests::table
                .filter(loan_requests::request_id.eq(&request_id))
                .first::<LoanRequestEntity>(&conn)
                .optional()
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(entity)
        })
        .await
        .map_err(|e| RepositoryError::InternalError(e.to_string()))??;
        
        Ok(result.map(LoanRequest::from))
    }

    async fn find_all(&self) -> Result<Vec<LoanRequest>, RepositoryError> {
        let pool = self.pool.clone();
        
        let results = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let entities = loan_requests::table
                .order(loan_requests::created_at.desc())
                .load::<LoanRequestEntity>(&conn)
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(entities)
        })
        .await
        .map_err(|e| RepositoryError::InternalError(e.to_string()))??;
        
        Ok(results.into_iter().map(LoanRequest::from).collect())
    }

    async fn update(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        let pool = self.pool.clone();
        let entity = LoanRequestEntity::from(loan_request);
        
        let updated_entity = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let rows = diesel::update(loan_requests::table)
                .filter(loan_requests::id.eq(&entity.id))
                .set((
                    loan_requests::applicant_name.eq(&entity.applicant_name),
                    loan_requests::amount.eq(entity.amount),
                    loan_requests::term_months.eq(entity.term_months),
                    loan_requests::status.eq(&entity.status),
                    loan_requests::updated_at.eq(Utc::now()),
                ))
                .execute(&conn)
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            
            if rows == 0 {
                return Err(RepositoryError::NotFound(
                    format!("Loan request with id {} not found", entity.id)
                ));
            }
            
            let updated = loan_requests::table
                .filter(loan_requests::id.eq(&entity.id))
                .first::<LoanRequestEntity>(&conn)
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(updated)
        })
        .await
        .map_err(|e| RepositoryError::InternalError(e.to_string()))??;
        
        Ok(LoanRequest::from(updated_entity))
    }

    async fn delete(&self, id: &str) -> Result<bool, RepositoryError> {
        let pool = self.pool.clone();
        let id = id.to_string();
        
        let deleted = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let rows = diesel::delete(loan_requests::table)
                .filter(loan_requests::id.eq(&id))
                .execute(&conn)
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            
            Ok(rows > 0)
        })
        .await
        .map_err(|e| RepositoryError::InternalError(e.to_string()))??;
        
        Ok(deleted)
    }

    async fn exists_by_request_id(&self, request_id: &str) -> Result<bool, RepositoryError> {
        let pool = self.pool.clone();
        let request_id = request_id.to_string();
        
        let exists = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let count: i64 = loan_requests::table
                .filter(loan_requests::request_id.eq(&request_id))
                .count()
                .get_result(&conn)
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(count > 0)
        })
        .await
        .map_err(|e| RepositoryError::InternalError(e.to_string()))??;
        
        Ok(exists)
    }
}

mod schema {
    pub mod loan_requests {
        diesel::table! {
            loan_requests (id) {
                id -> Text,
                request_id -> Text,
                applicant_name -> Text,
                amount -> Float,
                term_months -> Integer,
                status -> Text,
                created_at -> Timestamp,
                updated_at -> Timestamp,
            }
        }
    }
}


// === ARCHIVO: src/application/services/loan_service.rs ===

use crate::domain::models::loan_request::{LoanRequest, LoanStatus};
use crate::domain::repositories::loan_repository::LoanRepository;
use crate::interfaces::dto::loan_dto::{CreateLoanRequestDto, LoanResponseDto};
use crate::interfaces::controllers::errors::{ApiError, RepositoryError};
use async_trait::async_trait;
use std::sync::Arc;

pub struct LoanService<R: LoanRepository> {
    repository: Arc<R>,
}

impl<R: LoanRepository> LoanService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn create_loan(&self, dto: CreateLoanRequestDto) -> Result<LoanResponseDto, ApiError> {
        dto.to_entity().map_err(|e| ApiError::ValidationError(e))?;

        let exists = self.repository
            .exists_by_request_id(&dto.request_id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        if exists {
            return Err(ApiError::DuplicateResource(
                format!("La solicitud con ID {} ya existe", dto.request_id)
            ));
        }

        let loan_request = LoanRequest::new(
            dto.applicant_name.clone(),
            dto.request_id.clone(),
            dto.amount,
            dto.term_months,
        );

        if !loan_request.is_valid_amount() {
            return Err(ApiError::ValidationError(
                "El monto debe estar entre 100 y 10000 unidades".to_string()
            ));
        }

        if !loan_request.is_valid_term() {
            return Err(ApiError::ValidationError(
                "El plazo debe estar entre 6 y 60 meses".to_string()
            ));
        }

        let created = self.repository
            .create(loan_request)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        Ok(LoanResponseDto::from(created))
    }

    pub async fn get_loan(&self, id: &str) -> Result<LoanResponseDto, ApiError> {
        let loan = self.repository
            .find_by_id(id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        match loan {
            Some(req) => Ok(LoanResponseDto::from(req)),
            None => Err(ApiError::NotFound(
                format!("Solicitud de préstamo con ID {} no encontrada", id)
            )),
        }
    }

    pub async fn get_all_loans(&self) -> Result<Vec<LoanResponseDto>, ApiError> {
        let loans = self.repository
            .find_all()
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        Ok(loans.into_iter()
            .map(LoanResponseDto::from)
            .collect())
    }

    pub async fn update_loan_status(&self, id: &str, status: LoanStatus) -> Result<LoanResponseDto, ApiError> {
        let mut loan = self.repository
            .find_by_id(id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        match loan {
            Some(ref mut req) => {
                req.update_status(status);
                let updated = self.repository
                    .update(req.clone())
                    .await
                    .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
                Ok(LoanResponseDto::from(updated))
            },
            None => Err(ApiError::NotFound(
                format!("Solicitud de préstamo con ID {} no encontrada", id)
            )),
        }
    }

    pub async fn delete_loan(&self, id: &str) -> Result<bool, ApiError> {
        let exists = self.repository
            .find_by_id(id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        if exists.is_none() {
            return Err(ApiError::NotFound(
                format!("Solicitud de préstamo con ID {} no encontrada", id)
            ));
        }

        self.repository
            .delete(id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::loan_request::LoanStatus;
    use crate::interfaces::dto::loan_dto::CreateLoanRequestDto;
    use std::collections::HashMap;
    use std::sync::Mutex;

    struct MockRepository {
        loans: Mutex<HashMap<String, LoanRequest>>,
    }

    impl MockRepository {
        fn new() -> Self {
            Self { loans: Mutex::new(HashMap::new()) }
        }
    }

    #[async_trait]
    impl LoanRepository for MockRepository {
        async fn create(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
            let mut loans = self.loans.lock().unwrap();
            loans.insert(loan_request.id.clone(), loan_request.clone());
            Ok(loan_request)
        }

        async fn find_by_id(&self, id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
            let loans = self.loans.lock().unwrap();
            Ok(loans.get(id).cloned())
        }

        async fn find_by_request_id(&self, request_id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
            let loans = self.loans.lock().unwrap();
            Ok(loans.values().find(|r| r.request_id == request_id).cloned())
        }

        async fn find_all(&self) -> Result<Vec<LoanRequest>, RepositoryError> {
            let loans = self.loans.lock().unwrap();
            Ok(loans.values().cloned().collect())
        }

        async fn update(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
            let mut loans = self.loans.lock().unwrap();
            loans.insert(loan_request.id.clone(), loan_request.clone());
            Ok(loan_request)
        }

        async fn delete(&self, id: &str) -> Result<bool, RepositoryError> {
            let mut loans = self.loans.lock().unwrap();
            Ok(loans.remove(id).is_some())
        }

        async fn exists_by_request_id(&self, request_id: &str) -> Result<bool, RepositoryError> {
            let loans = self.loans.lock().unwrap();
            Ok(loans.values().any(|r| r.request_id == request_id))
        }
    }

    #[tokio::test]
    async fn test_create_loan_success() {
        let repository = Arc::new(MockRepository::new());
        let service = LoanService::new(repository);

        let dto = CreateLoanRequestDto {
            applicant_name: "Juan Pérez".to_string(),
            request_id: "REQ-001".to_string(),
            amount: 5000.0,
            term_months: 24,
        };

        let result = service.create_loan(dto).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_duplicate_loan() {
        let repository = Arc::new(MockRepository::new());
        let service = LoanService::new(repository);

        let dto = CreateLoanRequestDto {
            applicant_name: "Juan Pérez".to_string(),
            request_id: "REQ-002".to_string(),
            amount: 5000.0,
            term_months: 24,
        };

        let _ = service.create_loan(dto.clone()).await;
        let result = service.create_loan(dto).await;

        assert!(result.is_err());
        match result {
            Err(ApiError::DuplicateResource(_)) => {},
            _ => panic!("Expected DuplicateResource error"),
        }
    }

    #[tokio::test]
    async fn test_create_loan_invalid_amount() {
        let repository = Arc::new(MockRepository::new());
        let service = LoanService::new(repository);

        let dto = CreateLoanRequestDto {
            applicant_name: "Juan Pérez".to_string(),
            request_id: "REQ-003".to_string(),
            amount: 50.0,
            term_months: 24,
        };

        let result = service.create_loan(dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_loan_not_found() {
        let repository = Arc::new(MockRepository::new());
        let service = LoanService::new(repository);

        let result = service.get_loan("non-existent-id").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_loan_status() {
        let repository = Arc::new(MockRepository::new());
        let service = LoanService::new(repository);

        let dto = CreateLoanRequestDto {
            applicant_name: "Juan Pérez".to_string(),
            request_id: "REQ-004".to_string(),
            amount: 5000.0,
            term_months: 24,
        };

        let created = service.create_loan(dto).await.unwrap();
        let result = service.update_loan_status(&created.id, LoanStatus::Approved).await;
        assert!(result.is_ok());
    }
}

// === ARCHIVO: src/config/database.rs ===

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;
use std::time::Duration;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> Result<DbPool, Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar configurada en el archivo .env");

    let max_pool_size = env::var("DB_MAX_POOL_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let min_idle = env::var("DB_MIN_IDLE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2);

    let connection_timeout = env::var("DB_CONNECTION_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .max_size(max_pool_size)
        .min_idle(Some(min_idle))
        .connection_timeout(Duration::from_secs(connection_timeout))
        .test_on_check_out(true)
        .build(manager)?;

    Ok(pool)
}

pub fn get_connection(pool: &DbPool) -> Result<DbConnection, Box<dyn std::error::Error>> {
    pool.get().map_err(|e| {
        Box::new(e) as Box<dyn std::error::Error>
    })
}

pub fn run_migrations(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    use diesel_migrations::MigrationHarness;

    let mut connection = pool.get()?;

    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = {
    #[cfg(feature = "embedded_migrations")]
    {
        diesel_migrations::embed_migrations!("migrations")
    }
    #[cfg(not(feature = "embedded_migrations"))]
    {
        diesel_migrations::EmbeddedMigrations::new()
    }
};

pub mod schema {
    diesel::table! {
        loan_requests (id) {
            id -> Text,
            request_id -> Text,
            applicant_name -> Text,
            amount -> Float8,
            term_months -> Integer,
            status -> Text,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_database_url_required() {
        env::remove_var("DATABASE_URL");
        let result = create_db_pool();
        assert!(result.is_err());
    }

    #[test]
    fn test_pool_size_defaults() {
        env::set_var("DATABASE_URL", "postgres://user:pass@localhost/test");
        env::remove_var("DB_MAX_POOL_SIZE");
        env::remove_var("DB_MIN_IDLE");
        env::remove_var("DB_CONNECTION_TIMEOUT");

        let result = create_db_pool();
        assert!(result.is_err() || result.unwrap().size() == 10);

        env::remove_var("DATABASE_URL");
    }

    #[test]
    fn test_custom_pool_size() {
        env::set_var("DATABASE_URL", "postgres://user:pass@localhost/test");
        env::set_var("DB_MAX_POOL_SIZE", "20");
        env::set_var("DB_MIN_IDLE", "5");
        env::set_var("DB_CONNECTION_TIMEOUT", "60");

        let result = create_db_pool();
        if let Ok(pool) = result {
            assert_eq!(pool.size(), 20);
        }

        env::remove_var("DATABASE_URL");
        env::remove_var("DB_MAX_POOL_SIZE");
        env::remove_var("DB_MIN_IDLE");
        env::remove_var("DB_CONNECTION_TIMEOUT");
    }
}


// === ARCHIVO: migrations/2024-01-01-000000_create_loan_requests_table/up.sql ===
-- Migration: Create loan_requests table
-- Direction: up
-- Description: Creates the loan_requests table for storing loan applications in the loan management system.
-- This table stores applicant information, loan amount, term, and status with proper constraints.

-- Enable UUID extension for generating unique identifiers
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the loan_requests table with all required fields and constraints
CREATE TABLE loan_requests (
    -- Primary key: UUID-based identifier for global uniqueness across distributed systems
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Request identifier: unique string for idempotency - prevents duplicate submissions
    -- This field is critical for the duplicate detection requirement in the domain
    request_id VARCHAR(64) NOT NULL UNIQUE,
    
    -- Applicant name: required field storing the full name of the loan applicant
    -- Stored as VARCHAR to support international names with varying lengths
    applicant_name VARCHAR(255) NOT NULL,
    
    -- Loan amount: decimal field with precision for currency values
    -- Constraint: amount must be between 100 and 10000 units per domain requirements
    amount NUMERIC(12, 2) NOT NULL CHECK (amount >= 100 AND amount <= 10000),
    
    -- Term in months: integer field storing the loan repayment period
    -- Constraint: term must be between 6 and 60 months per domain requirements
    term_months INTEGER NOT NULL CHECK (term_months >= 6 AND term_months <= 60),
    
    -- Status: current state of the loan application
    -- Values: pending, approved, rejected, cancelled, under_review
    -- Default: pending - new applications start in pending status
    status VARCHAR(20) NOT NULL DEFAULT 'pending'::character varying,
    
    -- Rejection reason: optional field storing why a loan was rejected
    -- Only populated when status is 'rejected'
    rejection_reason TEXT,
    
    -- Approval date: timestamp when the loan was approved
    -- Nullable - only populated when status changes to 'approved'
    approved_at TIMESTAMP WITH TIME ZONE,
    
    -- Timestamps: standard audit fields for tracking record lifecycle
    -- created_at: automatically set when the record is inserted
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- updated_at: automatically updated on every modification via trigger
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create index on request_id for fast duplicate lookups
-- This index is crucial for the idempotency feature - O(1) lookup performance
CREATE INDEX idx_loan_requests_request_id ON loan_requests(request_id);

-- Create index on status for filtering by application status
-- Supports queries like "get all pending applications" efficiently
CREATE INDEX idx_loan_requests_status ON loan_requests(status);

-- Create index on created_at for temporal queries and ordering
-- Supports "get most recent applications" type queries
CREATE INDEX idx_loan_requests_created_at ON loan_requests(created_at DESC);

-- Create index on applicant_name for searching by name
-- Supports partial match searches (requires additional setup with pg_trgm)
CREATE INDEX idx_loan_requests_applicant_name ON loan_requests(applicant_name);

-- Create composite index for common query patterns: status + created_at
-- Optimizes queries like "get pending applications ordered by date"
CREATE INDEX idx_loan_requests_status_created ON loan_requests(status, created_at DESC);

-- Create trigger function to automatically update the updated_at timestamp
-- This ensures audit trail accuracy without manual updates in application code
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Attach the trigger to the loan_requests table
-- Fires BEFORE each UPDATE operation on any column
CREATE TRIGGER update_loan_requests_updated_at
    BEFORE UPDATE ON loan_requests
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Add comment to document the table purpose for future developers
COMMENT ON TABLE loan_requests IS 'Stores loan applications from the loan management platform. Each record represents a single loan request with applicant details, amount, term, and current processing status.';

-- Add comments to document critical columns
COMMENT ON COLUMN loan_requests.request_id IS 'Unique identifier for idempotency - prevents duplicate loan submissions from the same request';
COMMENT ON COLUMN loan_requests.amount IS 'Loan amount in platform currency - constrained to 100-10000 range per business rules';
COMMENT ON COLUMN loan_requests.term_months IS 'Repayment term in months - constrained to 6-60 range per business rules';
COMMENT ON COLUMN loan_requests.status IS 'Current processing status: pending, approved, rejected, cancelled, under_review';

// === ARCHIVO: migrations/2024-01-01-000000_create_loan_requests_table/down.sql ===
-- Migration: Create loan_requests table
-- Direction: down
-- Description: Reverts the loan_requests table creation by removing all associated objects.
-- This migration is executed when rolling back this database change.

-- First, remove the trigger that updates timestamps
-- Must drop trigger before dropping the function it references
DROP TRIGGER IF EXISTS update_loan_requests_updated_at ON loan_requests;

-- Drop the trigger function that was created for auto-updating timestamps
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop all indexes created for query optimization
-- These indexes improve query performance but are not required for table existence
DROP INDEX IF EXISTS idx_loan_requests_status_created;
DROP INDEX IF EXISTS idx_loan_requests_applicant_name;
DROP INDEX IF EXISTS idx_loan_requests_created_at;
DROP INDEX IF EXISTS idx_loan_requests_status;
DROP INDEX IF EXISTS idx_loan_requests_request_id;

-- Drop the main table
-- This permanently deletes all loan request data - cannot be undone
DROP TABLE IF EXISTS loan_requests;

-- Note: The uuid-ossp extension is NOT dropped here because:
-- 1. It might be used by other tables in the database
-- 2. Extensions are database-level objects, not schema-level
-- 3. Dropping extensions can cause issues with other objects
-- If this is the only table using UUIDs, manually run: DROP EXTENSION IF EXISTS "uuid-ossp";

-- Verify cleanup: this query should return no rows if successful
-- SELECT * FROM information_schema.tables WHERE table_name = 'loan_requests';


// === ARCHIVO: tests/loan_service_tests.rs ===
use loan_management_api::domain::models::loan_request::{LoanRequest, LoanStatus};
use loan_management_api::domain::repositories::loan_repository::{LoanRepository, RepositoryError};
use loan_management_api::application::services::loan_service::LoanService;
use loan_management_api::interfaces::dto::loan_dto::CreateLoanRequestDto;
use std::sync::Arc;
use tokio::sync::Mutex;

struct MockRepository {
    loans: Arc<Mutex<Vec<LoanRequest>>>,
    should_fail: Arc<Mutex<bool>>,
}

impl MockRepository {
    fn new() -> Self {
        Self {
            loans: Arc::new(Mutex::new(Vec::new())),
            should_fail: Arc::new(Mutex::new(false)),
        }
    }

    fn with_initial_data(initial: Vec<LoanRequest>) -> Self {
        Self {
            loans: Arc::new(Mutex::new(initial)),
            should_fail: Arc::new(Mutex::new(false)),
        }
    }
}

#[async_trait::async_trait]
impl LoanRepository for MockRepository {
    async fn create(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let mut loans = self.loans.lock().await;
        loans.push(loan_request.clone());
        Ok(loan_request)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let loans = self.loans.lock().await;
        Ok(loans.iter().find(|l| l.id.as_str() == id).cloned())
    }

    async fn find_by_request_id(&self, request_id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let loans = self.loans.lock().await;
        Ok(loans.iter().find(|l| l.request_id.as_str() == request_id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<LoanRequest>, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let loans = self.loans.lock().await;
        Ok(loans.clone())
    }

    async fn update(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let mut loans = self.loans.lock().await;
        if let Some(idx) = loans.iter().position(|l| l.id == loan_request.id) {
            loans[idx] = loan_request.clone();
            return Ok(loan_request);
        }
        Err(RepositoryError::NotFound(format!("Loan with id {} not found", loan_request.id)))
    }

    async fn delete(&self, id: &str) -> Result<bool, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let mut loans = self.loans.lock().await;
        let len_before = loans.len();
        loans.retain(|l| l.id.as_str() != id);
        Ok(loans.len() < len_before)
    }

    async fn exists_by_request_id(&self, request_id: &str) -> Result<bool, RepositoryError> {
        if *self.should_fail.lock().await {
            return Err(RepositoryError::ConnectionError("Mocked failure".to_string()));
        }
        let loans = self.loans.lock().await;
        Ok(loans.iter().any(|l| l.request_id.as_str() == request_id))
    }
}

#[tokio::test]
async fn test_service_creates_loan_successfully() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Juan Pérez".to_string(),
        amount: 5000.0,
        term_months: 24,
        request_id: "REQ-001".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_ok());
    let loan = result.unwrap();
    assert_eq!(loan.applicant_name, "Juan Pérez");
    assert_eq!(loan.amount, 5000.0);
    assert_eq!(loan.term_months, 24);
}

#[tokio::test]
async fn test_service_rejects_amount_below_minimum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Ana López".to_string(),
        amount: 50.0,
        term_months: 12,
        request_id: "REQ-002".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("monto") || err.to_string().contains("amount"));
}

#[tokio::test]
async fn test_service_rejects_amount_above_maximum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Carlos García".to_string(),
        amount: 50000.0,
        term_months: 12,
        request_id: "REQ-003".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_rejects_term_below_minimum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "María Rodríguez".to_string(),
        amount: 1000.0,
        term_months: 3,
        request_id: "REQ-004".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_rejects_term_above_maximum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Pedro Sánchez".to_string(),
        amount: 1000.0,
        term_months: 120,
        request_id: "REQ-005".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_rejects_duplicate_request_id() {
    let existing_loan = LoanRequest::new(
        "1".to_string(),
        "EXISTING-REQ".to_string(),
        "Cliente Existente".to_string(),
        3000.0,
        12,
        LoanStatus::Pending,
    );
    let mock_repo = MockRepository::with_initial_data(vec![existing_loan]);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Nuevo Cliente".to_string(),
        amount: 2000.0,
        term_months: 6,
        request_id: "EXISTING-REQ".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().to_lowercase().contains("duplic"));
}

#[tokio::test]
async fn test_service_allows_duplicate_request_id_after_deletion() {
    let existing_loan = LoanRequest::new(
        "1".to_string(),
        "REQ-TO-DELETE".to_string(),
        "Cliente a Eliminar".to_string(),
        3000.0,
        12,
        LoanStatus::Pending,
    );
    let mock_repo = MockRepository::with_initial_data(vec![existing_loan]);
    let service = LoanService::new(Arc::new(mock_repo.clone()));
    
    let delete_result = service.delete_loan("1".to_string()).await;
    assert!(delete_result.is_ok());
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Nuevo Cliente".to_string(),
        amount: 2000.0,
        term_months: 6,
        request_id: "REQ-TO-DELETE".to_string(),
    };
    
    let create_result = service.create_loan(dto).await;
    assert!(create_result.is_ok());
}

#[tokio::test]
async fn test_service_get_all_loans() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
        LoanRequest::new("2".to_string(), "REQ-2".to_string(), "Cliente 2".to_string(), 2000.0, 24, LoanStatus::Approved),
        LoanRequest::new("3".to_string(), "REQ-3".to_string(), "Cliente 3".to_string(), 3000.0, 36, LoanStatus::Rejected),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let result = service.get_all_loans().await;
    assert!(result.is_ok());
    let loans = result.unwrap();
    assert_eq!(loans.len(), 3);
}

#[tokio::test]
async fn test_service_get_loan_by_id() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
        LoanRequest::new("2".to_string(), "REQ-2".to_string(), "Cliente 2".to_string(), 2000.0, 24, LoanStatus::Approved),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let result = service.get_loan_by_id("1".to_string()).await;
    assert!(result.is_ok());
    let loan = result.unwrap();
    assert_eq!(loan.id, "1");
}

#[tokio::test]
async fn test_service_get_loan_by_id_not_found() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let result = service.get_loan_by_id("nonexistent".to_string()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_update_loan_status() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let result = service.update_loan_status("1".to_string(), LoanStatus::Approved).await;
    assert!(result.is_ok());
    let loan = result.unwrap();
    assert_eq!(loan.status, LoanStatus::Approved);
}

#[tokio::test]
async fn test_service_delete_loan() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let result = service.delete_loan("1".to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_validates_boundary_amount_minimum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Test User".to_string(),
        amount: 100.0,
        term_months: 12,
        request_id: "REQ-BOUNDARY-1".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_validates_boundary_amount_maximum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Test User".to_string(),
        amount: 10000.0,
        term_months: 12,
        request_id: "REQ-BOUNDARY-2".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_validates_boundary_term_minimum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Test User".to_string(),
        amount: 5000.0,
        term_months: 6,
        request_id: "REQ-BOUNDARY-3".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_validates_boundary_term_maximum() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let dto = CreateLoanRequestDto {
        applicant_name: "Test User".to_string(),
        amount: 5000.0,
        term_months: 60,
        request_id: "REQ-BOUNDARY-4".to_string(),
    };
    
    let result = service.create_loan(dto).await;
    assert!(result.is_ok());
}

// === ARCHIVO: tests/loan_controller_tests.rs ===
use actix_web::{test, web, App};
use loan_management_api::interfaces::controllers::loan_controller::configure_loan_routes;
use loan_management_api::interfaces::controllers::errors::ApiError;
use loan_management_api::domain::models::loan_request::{LoanRequest, LoanStatus};
use loan_management_api::domain::repositories::loan_repository::{LoanRepository, RepositoryError};
use loan_management_api::application::services::loan_service::LoanService;
use loan_management_api::interfaces::dto::loan_dto::{CreateLoanRequestDto, LoanResponseDto, LoanListResponseDto, ApiResponse};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::json;

struct MockRepository {
    loans: Arc<Mutex<Vec<LoanRequest>>>,
}

impl MockRepository {
    fn new() -> Self {
        Self {
            loans: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn with_initial_data(initial: Vec<LoanRequest>) -> Self {
        Self {
            loans: Arc::new(Mutex::new(initial)),
        }
    }
}

#[async_trait::async_trait]
impl LoanRepository for MockRepository {
    async fn create(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        let mut loans = self.loans.lock().await;
        loans.push(loan_request.clone());
        Ok(loan_request)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        let loans = self.loans.lock().await;
        Ok(loan_request.iter().find(|l| l.id.as_str() == id).cloned())
    }

    async fn find_by_request_id(&self, request_id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        let loans = self.loans.lock().await;
        Ok(loans.iter().find(|l| l.request_id.as_str() == request_id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<LoanRequest>, RepositoryError> {
        let loans = self.loans.lock().await;
        Ok(loans.clone())
    }

    async fn update(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        let mut loans = self.loans.lock().await;
        if let Some(idx) = loans.iter().position(|l| l.id == loan_request.id) {
            loans[idx] = loan_request.clone();
            return Ok(loan_request);
        }
        Err(RepositoryError::NotFound(format!("Loan with id {} not found", loan_request.id)))
    }

    async fn delete(&self, id: &str) -> Result<bool, RepositoryError> {
        let mut loans = self.loans.lock().await;
        let len_before = loans.len();
        loans.retain(|l| l.id.as_str() != id);
        Ok(loans.len() < len_before)
    }

    async fn exists_by_request_id(&self, request_id: &str) -> Result<bool, RepositoryError> {
        let loans = self.loans.lock().await;
        Ok(loans.iter().any(|l| l.request_id.as_str() == request_id))
    }
}

#[actix_web::test]
async fn test_create_loan_endpoint_success() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Juan Pérez",
            "amount": 5000.0,
            "term_months": 24,
            "request_id": "REQ-TEST-001"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success() || resp.status().is_created());
}

#[actix_web::test]
async fn test_create_loan_endpoint_invalid_amount() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Juan Pérez",
            "amount": 50.0,
            "term_months": 24,
            "request_id": "REQ-TEST-002"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_create_loan_endpoint_invalid_term() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Juan Pérez",
            "amount": 5000.0,
            "term_months": 3,
            "request_id": "REQ-TEST-003"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_create_loan_endpoint_duplicate() {
    let existing = LoanRequest::new(
        "1".to_string(),
        "REQ-DUP".to_string(),
        "Cliente Existente".to_string(),
        3000.0,
        12,
        LoanStatus::Pending,
    );
    let mock_repo = MockRepository::with_initial_data(vec![existing]);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Nuevo Cliente",
            "amount": 2000.0,
            "term_months": 6,
            "request_id": "REQ-DUP"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_get_all_loans_endpoint() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
        LoanRequest::new("2".to_string(), "REQ-2".to_string(), "Cliente 2".to_string(), 2000.0, 24, LoanStatus::Approved),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/loans")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_loan_by_id_endpoint() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/loans/1")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_loan_by_id_not_found() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/loans/nonexistent")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_not_found());
}

#[actix_web::test]
async fn test_update_loan_status_endpoint() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::put()
        .uri("/api/loans/1/status")
        .set_json(&json!({
            "status": "approved"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_delete_loan_endpoint() {
    let loans = vec![
        LoanRequest::new("1".to_string(), "REQ-1".to_string(), "Cliente 1".to_string(), 1000.0, 12, LoanStatus::Pending),
    ];
    let mock_repo = MockRepository::with_initial_data(loans);
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::delete()
        .uri("/api/loans/1")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success() || resp.status().is_no_content());
}

#[actix_web::test]
async fn test_create_loan_missing_applicant_name() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "amount": 5000.0,
            "term_months": 24,
            "request_id": "REQ-TEST-004"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_create_loan_missing_request_id() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Juan Pérez",
            "amount": 5000.0,
            "term_months": 24
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_create_loan_negative_amount() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Juan Pérez",
            "amount": -100.0,
            "term_months": 24,
            "request_id": "REQ-TEST-005"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_create_loan_zero_term() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/loans")
        .set_json(&json!({
            "applicant_name": "Juan Pérez",
            "amount": 5000.0,
            "term_months": 0,
            "request_id": "REQ-TEST-006"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_get_all_loans_empty() {
    let mock_repo = MockRepository::new();
    let service = LoanService::new(Arc::new(mock_repo));
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .configure(configure_loan_routes)
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/loans")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}


// === ARCHIVO: src/domain/mod.rs ===
pub mod models;
pub mod repositories;

// === ARCHIVO: src/domain/models/mod.rs ===
pub mod loan_request;
pub use loan_request::{LoanRequest, LoanStatus, ValidationError};

// === ARCHIVO: src/domain/repositories/mod.rs ===
pub mod loan_repository;
pub use loan_repository::{LoanRepository, LoanFilter};

// === ARCHIVO: src/domain/models/loan_request.rs ===
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LoanStatus {
    Pending,
    Approved,
    Rejected,
}

impl Default for LoanStatus {
    fn default() -> Self {
        LoanStatus::Pending
    }
}

impl std::fmt::Display for LoanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoanStatus::Pending => write!(f, "pending"),
            LoanStatus::Approved => write!(f, "approved"),
            LoanStatus::Rejected => write!(f, "rejected"),
        }
    }
}

impl From<String> for LoanStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "approved" => LoanStatus::Approved,
            "rejected" => LoanStatus::Rejected,
            _ => LoanStatus::Pending,
        }
    }
}

impl From<&str> for LoanStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "approved" => LoanStatus::Approved,
            "rejected" => LoanStatus::Rejected,
            _ => LoanStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoanRequest {
    #[validate(length(min = 2, max = 100, message = "El nombre debe tener entre 2 y 100 caracteres"))]
    pub applicant_name: String,
    
    #[validate(range(min = 100, max = 10000, message = "El monto debe estar entre 100 y 10000"))]
    pub amount: f64,
    
    #[validate(range(min = 6, max = 60, message = "El plazo debe estar entre 6 y 60 meses"))]
    pub term_months: i32,
    
    #[validate(uuid(message = "El identificador debe ser un UUID válido"))]
    pub request_id: String,
    
    pub id: String,
    
    pub status: LoanStatus,
    
    pub created_at: Option<DateTime<Utc>>,
    
    pub updated_at: Option<DateTime<Utc>>,
}

impl LoanRequest {
    pub fn new(
        id: String,
        request_id: String,
        applicant_name: String,
        amount: f64,
        term_months: i32,
        status: LoanStatus,
    ) -> Self {
        let now = Utc::now();
        LoanRequest {
            id,
            request_id,
            applicant_name,
            amount,
            term_months,
            status,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }
    
    pub fn restore(
        id: String,
        request_id: String,
        applicant_name: String,
        amount: f64,
        term_months: i32,
        status: LoanStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        LoanRequest {
            id,
            request_id,
            applicant_name,
            amount,
            term_months,
            status,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        }
    }
    
    pub fn update_status(&mut self, status: LoanStatus) {
        self.status = status;
        self.updated_at = Some(Utc::now());
    }
    
    pub fn is_valid_amount(&self) -> bool {
        self.amount >= 100.0 && self.amount <= 10000.0
    }
    
    pub fn is_valid_term(&self) -> bool {
        self.term_months >= 6 && self.term_months <= 60
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn request_id(&self) -> &str {
        &self.request_id
    }
    
    pub fn applicant_name(&self) -> &str {
        &self.applicant_name
    }
    
    pub fn amount(&self) -> f64 {
        self.amount
    }
    
    pub fn term_months(&self) -> i32 {
        self.term_months
    }
    
    pub fn status(&self) -> &LoanStatus {
        &self.status
    }
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    AmountOutOfRange,
    TermOutOfRange,
    InvalidName,
    MissingRequestId,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::AmountOutOfRange => {
                write!(f, "El monto debe estar entre 100 y 10000 unidades")
            }
            ValidationError::TermOutOfRange => {
                write!(f, "El plazo debe estar entre 6 y 60 meses")
            }
            ValidationError::InvalidName => {
                write!(f, "El nombre del solicitante es inválido")
            }
            ValidationError::MissingRequestId => {
                write!(f, "El identificador de solicitud es requerido")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loan_request_creation_with_valid_data() {
        let request = LoanRequest::new(
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
            "REQ-001".to_string(),
            "Juan Pérez".to_string(),
            5000.0,
            24,
            LoanStatus::Pending,
        );
        
        assert_eq!(request.applicant_name, "Juan Pérez");
        assert_eq!(request.amount, 5000.0);
        assert_eq!(request.term_months, 24);
    }
    
    #[test]
    fn test_loan_request_status_update() {
        let mut request = LoanRequest::new(
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
            "REQ-001".to_string(),
            "Juan Pérez".to_string(),
            5000.0,
            24,
            LoanStatus::Pending,
        );
        
        request.update_status(LoanStatus::Approved);
        
        assert_eq!(request.status, LoanStatus::Approved);
        assert!(request.updated_at.is_some());
    }
}

// === ARCHIVO: src/domain/repositories/loan_repository.rs ===
use crate::domain::models::loan_request::LoanRequest;
use crate::interfaces::controllers::errors::RepositoryError;
use async_trait::async_trait;

#[async_trait]
pub trait LoanRepository: Send + Sync {
    async fn create(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError>;
    
    async fn find_by_id(&self, id: &str) -> Result<Option<LoanRequest>, RepositoryError>;
    
    async fn find_by_request_id(&self, request_id: &str) -> Result<Option<LoanRequest>, RepositoryError>;
    
    async fn find_all(&self) -> Result<Vec<LoanRequest>, RepositoryError>;
    
    async fn update(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError>;
    
    async fn delete(&self, id: &str) -> Result<bool, RepositoryError>;
    
    async fn exists_by_request_id(&self, request_id: &str) -> Result<bool, RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct LoanFilter {
    pub status: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub applicant_name_contains: Option<String>,
}

impl Default for LoanFilter {
    fn default() -> Self {
        LoanFilter {
            status: None,
            min_amount: None,
            max_amount: None,
            applicant_name_contains: None,
        }
    }
}

impl LoanFilter {
    pub fn with_status(status: String) -> Self {
        LoanFilter {
            status: Some(status),
            ..Default::default()
        }
    }
    
    pub fn with_amount_range(min: f64, max: f64) -> Self {
        LoanFilter {
            min_amount: Some(min),
            max_amount: Some(max),
            ..Default::default()
        }
    }
    
    pub fn with_applicant_name(name: String) -> Self {
        LoanFilter {
            applicant_name_contains: Some(name),
            ..Default::default()
        }
    }
}

// === ARCHIVO: src/application/mod.rs ===
pub mod services;

// === ARCHIVO: src/application/services/mod.rs ===
pub mod loan_service;
pub use loan_service::LoanService;

// === ARCHIVO: src/application/services/loan_service.rs ===
use crate::domain::models::loan_request::{LoanRequest, LoanStatus};
use crate::domain::repositories::loan_repository::LoanRepository;
use crate::interfaces::dto::loan_dto::{CreateLoanRequestDto, LoanResponseDto};
use crate::interfaces::controllers::errors::{ApiError, RepositoryError};
use async_trait::async_trait;
use std::sync::Arc;

pub struct LoanService<R: LoanRepository> {
    repository: Arc<R>,
}

impl<R: LoanRepository> LoanService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn create_loan(&self, dto: CreateLoanRequestDto) -> Result<LoanResponseDto, ApiError> {
        let exists = self.repository
            .exists_by_request_id(&dto.request_id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        if exists {
            return Err(ApiError::DuplicateResource(
                format!("La solicitud con ID {} ya existe", dto.request_id)
            ));
        }

        let loan_request = LoanRequest::new(
            uuid::Uuid::new_v4().to_string(),
            dto.request_id.clone(),
            dto.applicant_name.clone(),
            dto.amount,
            dto.term_months,
            LoanStatus::Pending,
        );

        if !loan_request.is_valid_amount() {
            return Err(ApiError::ValidationError(
                "El monto debe estar entre 100 y 10000 unidades".to_string()
            ));
        }

        if !loan_request.is_valid_term() {
            return Err(ApiError::ValidationError(
                "El plazo debe estar entre 6 y 60 meses".to_string()
            ));
        }

        let created = self.repository
            .create(loan_request)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        Ok(LoanResponseDto::from(created))
    }

    pub async fn get_loan(&self, id: &str) -> Result<LoanResponseDto, ApiError> {
        let loan = self.repository
            .find_by_id(id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        match loan {
            Some(req) => Ok(LoanResponseDto::from(req)),
            None => Err(ApiError::NotFound(
                format!("Solicitud de préstamo con ID {} no encontrada", id)
            )),
        }
    }

    pub async fn get_all_loans(&self) -> Result<Vec<LoanResponseDto>, ApiError> {
        let loans = self.repository
            .find_all()
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        Ok(loans.into_iter()
            .map(LoanResponseDto::from)
            .collect())
    }

    pub async fn update_loan_status(&self, id: &str, status: LoanStatus) -> Result<LoanResponseDto, ApiError> {
        let mut loan = self.repository
            .find_by_id(id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        match loan {
            Some(ref mut req) => {
                req.update_status(status);
                let updated = self.repository
                    .update(req.clone())
                    .await
                    .map_err(|e| ApiError::RepositoryError(e.to_string()))?;
                Ok(LoanResponseDto::from(updated))
            },
            None => Err(ApiError::NotFound(
                format!("Solicitud de préstamo con ID {} no encontrada", id)
            )),
        }
    }

    pub async fn delete_loan(&self, id: &str) -> Result<bool, ApiError> {
        let exists = self.repository
            .find_by_id(id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))?;

        if exists.is_none() {
            return Err(ApiError::NotFound(
                format!("Solicitud de préstamo con ID {} no encontrada", id)
            ));
        }

        self.repository
            .delete(id)
            .await
            .map_err(|e| ApiError::RepositoryError(e.to_string()))
    }
}

// === ARCHIVO: src/infrastructure/mod.rs ===
pub mod persistence;

// === ARCHIVO: src/infrastructure/persistence/mod.rs ===
pub mod diesel_loan_repository;
pub use diesel_loan_repository::{DieselLoanRepository, DbPool};

// === ARCHIVO: src/infrastructure/persistence/diesel_loan_repository.rs ===
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{PgConnection, Insertable, Queryable, AsChangeset};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::models::loan_request::{LoanRequest, LoanStatus};
use crate::domain::repositories::loan_repository::{LoanRepository, RepositoryError};
use crate::infrastructure::persistence::schema::loan_requests;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Queryable, Insertable, AsChangeset, Clone, Debug)]
#[diesel(table_name = loan_requests)]
#[diesel(treat_none_as_default = false)]
pub struct LoanRequestEntity {
    pub id: String,
    pub request_id: String,
    pub applicant_name: String,
    pub amount: f64,
    pub term_months: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<LoanRequest> for LoanRequestEntity {
    fn from(req: LoanRequest) -> Self {
        let now = Utc::now();
        LoanRequestEntity {
            id: req.id().to_string(),
            request_id: req.request_id().to_string(),
            applicant_name: req.applicant_name().to_string(),
            amount: req.amount(),
            term_months: req.term_months(),
            status: req.status().to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<LoanRequestEntity> for LoanRequest {
    fn from(entity: LoanRequestEntity) -> Self {
        let status = LoanStatus::from(entity.status.as_str());
        LoanRequest::restore(
            entity.id,
            entity.request_id,
            entity.applicant_name,
            entity.amount,
            entity.term_months,
            status,
            entity.created_at,
            entity.updated_at,
        )
    }
}

pub struct DieselLoanRepository {
    pool: DbPool,
}

impl DieselLoanRepository {
    pub fn new(pool: DbPool) -> Self {
        DieselLoanRepository { pool }
    }

    fn get_connection(&self) -> Result<DbConnection, RepositoryError> {
        self.pool
            .get()
            .map_err(|e| RepositoryError::ConnectionError(e.to_string()))
    }
}

#[async_trait]
impl LoanRepository for DieselLoanRepository {
    async fn create(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        let pool = self.pool.clone();
        
        let result = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let entity = LoanRequestEntity::from(loan_request);
            
            diesel::insert_into(loan_requests::table)
                .values(&entity)
                .execute(&conn)
                .map_err(|e| {
                    if let diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) = e {
                        RepositoryError::Duplicate(format!("Loan request with id {} already exists", entity.request_id))
                    } else {
                        RepositoryError::DatabaseError(e.to_string())
                    }
                })?;
            
            Ok::<_, RepositoryError>(entity)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(LoanRequest::from(result))
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        let pool = self.pool.clone();
        let id = id.to_string();
        
        let result = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let entity = loan_requests::table
                .filter(loan_requests::id.eq(&id))
                .first::<LoanRequestEntity>(&conn)
                .optional()
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(entity)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(result.map(LoanRequest::from))
    }

    async fn find_by_request_id(&self, request_id: &str) -> Result<Option<LoanRequest>, RepositoryError> {
        let pool = self.pool.clone();
        let request_id = request_id.to_string();
        
        let result = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let entity = loan_requests::table
                .filter(loan_requests::request_id.eq(&request_id))
                .first::<LoanRequestEntity>(&conn)
                .optional()
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(entity)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(result.map(LoanRequest::from))
    }

    async fn find_all(&self) -> Result<Vec<LoanRequest>, RepositoryError> {
        let pool = self.pool.clone();
        
        let results = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let entities = loan_requests::table
                .order(loan_requests::created_at.desc())
                .load::<LoanRequestEntity>(&conn)
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(entities)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(results.into_iter().map(LoanRequest::from).collect())
    }

    async fn update(&self, loan_request: LoanRequest) -> Result<LoanRequest, RepositoryError> {
        let pool = self.pool.clone();
        let entity = LoanRequestEntity::from(loan_request);
        
        let updated_entity = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let rows = diesel::update(loan_requests::table)
                .filter(loan_requests::id.eq(&entity.id))
                .set((
                    loan_requests::applicant_name.eq(&entity.applicant_name),
                    loan_requests::amount.eq(entity.amount),
                    loan_requests::term_months.eq(entity.term_months),
                    loan_requests::status.eq(&entity.status),
                    loan_requests::updated_at.eq(Utc::now()),
                ))
                .execute(&conn)
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            
            if rows == 0 {
                return Err(RepositoryError::NotFound(
                    format!("Loan request with id {} not found", entity.id)
                ));
            }
            
            let updated = loan_requests::table
                .filter(loan_requests::id.eq(&entity.id))
                .first::<LoanRequestEntity>(&conn)
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(updated)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(LoanRequest::from(updated_entity))
    }

    async fn delete(&self, id: &str) -> Result<bool, RepositoryError> {
        let pool = self.pool.clone();
        let id = id.to_string();
        
        let deleted = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let rows = diesel::delete(loan_requests::table)
                .filter(loan_requests::id.eq(&id))
                .execute(&conn)
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            
            Ok(rows > 0)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(deleted)
    }

    async fn exists_by_request_id(&self, request_id: &str) -> Result<bool, RepositoryError> {
        let pool = self.pool.clone();
        let request_id = request_id.to_string();
        
        let exists = tokio::task::spawn_blocking(move || {
            let conn = pool.get()
                .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;
            
            let count: i64 = loan_requests::table
                .filter(loan_requests::request_id.eq(&request_id))
                .count()
                .get_result(&conn)
                .map_err(|e| RepositoryError::QueryError(e.to_string()))?;
            
            Ok(count > 0)
        })
        .await
        .map_err(|e| RepositoryError::ConnectionError(e.to_string()))??;
        
        Ok(exists)
    }
}

pub mod schema {
    pub mod loan_requests {
        diesel::table! {
            loan_requests (id) {
                id -> Text,
                request_id -> Text,
                applicant_name -> Text,
                amount -> Float8,
                term_months -> Integer,
                status -> Text,
                created_at -> Timestamp,
                updated_at -> Timestamp,
            }
        }
    }
}

// === ARCHIVO: src/interfaces/mod.rs ===
pub mod controllers;
pub mod dto;

// === ARCHIVO: src/interfaces/controllers/mod.rs ===
pub mod loan_controller;
pub mod errors;
pub use errors::{ApiError, ApiResponse};

// === ARCHIVO: src/interfaces/dto/mod.rs ===
pub mod loan_dto;
pub use loan_dto::{CreateLoanRequestDto, LoanResponseDto, UpdateLoanStatusDto, LoanListResponseDto, ApiResponse as DtoApiResponse};

// === ARCHIVO: src/config/mod.rs ===
pub mod database;

// === ARCHIVO: src/lib.rs ===
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;
pub mod config;

pub use domain::models::loan_request::{LoanRequest, LoanStatus, ValidationError};
pub use domain::repositories::loan_repository::{LoanRepository, LoanFilter};
pub use application::services::loan_service::LoanService;
pub use infrastructure::persistence::diesel_loan_repository::{DieselLoanRepository, DbPool};
pub use interfaces::controllers::errors::{ApiError, ApiResponse};
pub use interfaces::dto::loan_dto::{CreateLoanRequestDto, LoanResponseDto, UpdateLoanStatusDto, LoanListResponseDto};

```
