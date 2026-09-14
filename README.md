# Desarrollo de una API REST en Rust con Actix Web y Diesel ORM

Necesitamos un servicio que gestione solicitudes de préstamos en una plataforma de préstamos en línea. El sistema debe registrar solicitudes de préstamo con información del solicitante (nombre, monto, plazo) y validar que el monto sea entre 100 y 10 000 unidades y el plazo entre 6 y 60 meses. El sistema debe asegurar que no se registren solicitudes duplicadas y debe manejar errores de forma idempotente. Los datos deben persistir en una base de datos PostgreSQL.

## Informacion General

| Campo | Valor |
|-------|-------|
| **Tema** | rust-actix-web |
| **Nivel** | junior-l1 |
| **Tipo** | practical |
| **Tiempo estimado** | 8 horas |

## Fases del Reto

### Fase 0: Configuración del Proyecto

**Objetivo:** Obtener el proyecto base funcional enviando el Código Base a un asistente de IA, que lo analizará, corregirá errores y generará un ZIP listo para usar.

**Tiempo estimado:** 15-30 minutos

**Instrucciones:**

- Asegúrate de tener instalado para ejecutar el proyecto: Node.js 18+, npm, VS Code o similar.
- Copia todo el contenido del campo **Código Base** de este reto — incluyendo el texto de instrucciones que aparece al inicio.
- Abre un asistente de IA (Claude en claude.ai, ChatGPT o Gemini — se recomienda Claude), pega el contenido copiado en el chat y envíalo.
- El asistente analizará los archivos, corregirá errores y generará un archivo ZIP descargable. Descárgalo y extráelo en la carpeta donde quieras trabajar.
- Ejecuta `npm install && npm run build` (o `npm start`). Si no hay errores, estás listo.

**Entregable:** El proyecto compila/arranca sin errores.

<details>
<summary>Pistas de conocimiento</summary>

- Copia el Código Base completo incluyendo el texto de instrucciones al inicio — esas instrucciones le indican al asistente exactamente qué hacer con los archivos.
- Si el asistente no genera el ZIP automáticamente al terminar el análisis, escríbele: "genera el ZIP ahora".
- Si el proyecto tiene errores al arrancar, comparte el mensaje de error con el mismo asistente para que lo corrija.

</details>

### Fase 1: Registro de solicitudes

**Objetivo:** Implementar la funcionalidad para registrar solicitudes de préstamo con validación de campos y manejo de duplicados.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- El sistema debe registrar solicitudes de préstamo con nombre, monto y plazo.
- Validar que el monto esté entre 100 y 10 000 unidades y el plazo entre 6 y 60 meses.
- Asegurar que no se registren solicitudes duplicadas utilizando un identificador único.
- Manejar errores de forma idempotente.

**Entregable:** Servicio que registra solicitudes de préstamo con validación y manejo de duplicados.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo representar y validar los datos de entrada.
- Piensa en cómo asegurar la idempotencia en el registro de solicitudes.

</details>

### Fase 2: Persistencia de datos

**Objetivo:** Implementar la persistencia de las solicitudes de préstamo en una base de datos PostgreSQL.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- Persistir las solicitudes de préstamo en una base de datos PostgreSQL.
- Asegurar que la persistencia sea idempotente y consistente.
- Manejar posibles errores de conexión o escritura en la base de datos.

**Entregable:** Servicio que registra y persiste solicitudes de préstamo en una base de datos PostgreSQL.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo modelar la base de datos para almacenar las solicitudes de préstamo.
- Piensa en cómo manejar errores de conexión o escritura en la base de datos.

</details>

### Fase 3: Manejo de errores y recuperación

**Objetivo:** Implementar el manejo de errores y la recuperación ante posibles fallos en el sistema.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Implementar mecanismos para manejar errores y recuperarse ante posibles fallos en el sistema.
- Asegurar que el sistema continúe operando de forma idempotente y consistente ante errores.
- Documentar las decisiones tomadas y las consecuencias de estas.

**Entregable:** Servicio que registra, persiste y maneja errores en las solicitudes de préstamo de forma idempotente y consistente.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo implementar mecanismos de reintento y recuperación ante errores.
- Piensa en cómo documentar las decisiones tomadas y las consecuencias de estas.

</details>

## Dimensiones Evaluadas

- **queEs**: ¿Qué es una solicitud de préstamo y cómo se registra en el sistema?
- **paraQueSirve**: ¿Para qué sirve validar los campos de una solicitud de préstamo?
- **comoSeUsa**: ¿Cómo se usa un identificador único para evitar solicitudes duplicadas?
- **erroresComunes**: ¿Cuáles son los errores comunes al registrar y persistir solicitudes de préstamo?
- **queDecisionesImplica**: ¿Qué decisiones implica el manejo de errores y la recuperación ante fallos en el sistema?

## Criterios de Evaluacion

- Implementación correcta de la funcionalidad para registrar solicitudes de préstamo con validación y manejo de duplicados.
- Persistencia correcta de las solicitudes de préstamo en una base de datos PostgreSQL.
- Manejo correcto de errores y recuperación ante posibles fallos en el sistema.
- Documentación clara y concisa de las decisiones tomadas y las consecuencias de estas.

## Como trabajar con un asistente de IA

Hay dos caminos, elegi uno:

- **AGENTS.md** (recomendado) — instrucciones nativas del repo. Abri esta carpeta con tu agente local (Claude Code, Cursor, Codex, Copilot, Gemini) y las carga solo. Sabe que archivos faltan y con que comando se verifica, y completa el scaffold escribiendo en disco.
- **PROMPT_MEJORA.md** — para copiar y pegar en un chat (claude.ai, ChatGPT). Devuelve un ZIP con el proyecto. Sirve si no tenes un agente en el IDE.

Ninguno de los dos resuelve las fases del reto: eso es tu trabajo.

## Verificacion

El proyecto esta listo para trabajar cuando este comando corre sin errores:

```bash
el comando de build o arranque canonico del stack elegido
```

---

*Reto generado automaticamente por Challenge Generator - Pragma*
