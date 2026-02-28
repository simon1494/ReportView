# Trabajo Práctico Final – Marketplace Descentralizado

**Materia:** Seminario de Lenguajes – Opción Rust  
**Tecnología:** Rust + Ink! + Substrate  
**Cobertura de tests requerida:** ≥ 85%  
**Entregas:**  
- ⭕ Primera entrega obligatoria: **18 de julio**  
- ✅ Entrega final completa: **Antes de finalizar 2025**

<img width="8334" height="4167" alt="image" src="https://github.com/user-attachments/assets/9bc5857c-5349-45ab-9e2b-a3edac75840b" />

---

## 📜 Introducción

El presente trabajo práctico final tiene como objetivo integrar los conocimientos adquiridos durante el cursado de la materia **Seminario de Lenguajes – Opción Rust**, aplicando conceptos de programación en Rust orientados al desarrollo de contratos inteligentes sobre la plataforma **Substrate** utilizando el framework **Ink!**.

La consigna propone desarrollar una **plataforma descentralizada de compra-venta de productos**, inspirada en modelos como MercadoLibre, pero ejecutada completamente en un entorno blockchain. El sistema deberá dividirse en **dos contratos inteligentes**: uno encargado de gestionar la lógica principal del marketplace y otro destinado a la generación de reportes a partir de los datos públicos del primero.

El proyecto busca que el estudiante no solo practique la sintaxis y semántica de Rust, sino que también comprenda el diseño modular de contratos inteligentes, la separación de responsabilidades, la validación de roles y permisos, y la importancia de la transparencia, trazabilidad y reputación en contextos descentralizados.

Se espera que las entregas incluyan una implementación funcional, correctamente testeada, documentada y con una cobertura de pruebas mínima del 85%.

---

## Contrato 2 – `ReportesView` (solo lectura)

### Funcionalidades
- Consultar top 5 vendedores con mejor reputación.
- Consultar top 5 compradores con mejor reputación.
- Ver productos más vendidos.
- Estadísticas por categoría: total de ventas, calificación promedio.
- Cantidad de órdenes por usuario.

**Nota:** este contrato solo puede leer datos del contrato 1. No puede emitir calificaciones, modificar órdenes ni publicar productos.

---

## 📊 Requisitos generales

- ✅ Cobertura de tests ≥ 85% entre ambos contratos.
- ✅ Tests deben contemplar:
  - Flujos completos de compra y calificación.
  - Validaciones y errores esperados.
  - Permisos por rol.
- ✅ Código comentado y bien estructurado.

---

## 🌟 Entrega Final – Fin de año

Incluye:
- Toda la funcionalidad de ambos contratos.
- Reputación completa bidireccional.
- Reportes por lectura (contrato 2).
- Tests con cobertura ≥ 85%.
- Documentación técnica clara.

### Bonus (hasta +20%):
- Sistema de disputas.
- Simulación de pagos.

---

## 📦 Contrato `ReportView` – Documentación Técnica

### Descripción general

`ReportView` es un contrato inteligente de **solo lectura** desplegado sobre Substrate/Ink! cuya única responsabilidad es generar reportes y estadísticas a partir de los datos del contrato principal del marketplace descentralizado.

Este contrato **no almacena ni modifica datos propios**. Toda la información que consume proviene de llamadas a la API pública del contrato de marketplace ya desplegado on-chain. La comunicación se realiza a través de una referencia cross-contract (`SistemaRef`) que apunta al address del contrato original en la misma blockchain.

---

### 🏗️ Arquitectura interna

El contrato organiza su lógica interna mediante **tres traits** que agrupan las operaciones según el dominio de datos que consultan:

#### `ConsultasProductos`
Agrupa las operaciones relacionadas con productos y sus ventas:
- `_mapear_nombres_productos`: genera un mapeo entre publicaciones y los productos que representan.
- `_contar_ventas_por_producto`: itera sobre órdenes recibidas y acumula las unidades vendidas por producto.
- `_productos_mas_vendidos`: combina las dos anteriores y devuelve los productos ordenados por ventas descendentes.

#### `ConsultasCategorias`
Agrupa las operaciones de estadísticas por categoría:
- `_mapear_productos_por_categoria`: relaciona cada categoría con los IDs de sus productos.
- `_contar_ventas_por_categoria`: suma las ventas de todos los productos que pertenecen a cada categoría.
- `_calcular_promedio_calificaciones_categoria`: calcula el promedio de calificaciones de órdenes recibidas dentro de cada categoría.
- `_extraer_calificaciones_de_ordenes`: filtra y extrae calificaciones válidas de un conjunto de órdenes.
- `_calcular_promedio_de_calificaciones`: calcula el promedio aritmético y lo formatea como `"X.Y"`.
- `_estadisticas_por_categoria`: combina ventas y calificaciones en un único vector de estadísticas.

#### `ConsultasUsuarios`
Agrupa las operaciones relacionadas con usuarios y su actividad:
- `_cantidad_de_ordenes_por_usuario`: cuenta cuántas órdenes generó cada usuario como comprador.
- `_filtrar_usuarios_por_rol_desc`: filtra usuarios por rol (`Comprador` o `Vendedor`) y los ordena por calificación de mayor a menor.
- `_listar_mejores_cinco_usuarios`: recorta un vector ya ordenado a los primeros 5 y extrae sus calificaciones.
- `_mejores_usuarios_por_rol`: orquesta las dos anteriores y valida que el rol sea `Comprador` o `Vendedor` (no `Ambos`).

Todos los traits son implementados por el struct `Reportes`, que actúa como punto de entrada único del contrato.

---

### 🌐 Interfaz pública

Las siguientes funciones son los mensajes públicos del contrato, accesibles por cualquier usuario:

| Mensaje | Descripción | Retorno |
|---|---|---|
| `listar_cantidad_de_ordenes_por_usuario()` | Devuelve todos los usuarios con la cantidad de órdenes que generaron como compradores | `Vec<OrdenesPorUsuario>` |
| `listar_mejores_usuarios_por_rol(target_role)` | Devuelve los 5 usuarios mejor calificados en el rol indicado (`Comprador` o `Vendedor`), ordenados descendentemente | `Vec<RatingPorUsuario>` |
| `listar_productos_mas_vendidos()` | Devuelve todos los productos ordenados por unidades vendidas (de mayor a menor) | `Vec<VentasPorProducto>` |
| `listar_estadisticas_por_categoria()` | Devuelve por cada categoría el total de ventas y el promedio de calificaciones de sus productos | `Vec<EstadisticasPorCategoria>` |

Todos los mensajes retornan un `Result<Vec<T>, ErroresReportes>`. En caso de éxito, el vector puede estar vacío si no hay datos; en caso de error se devuelve un `ErroresReportes`.

#### Posibles errores (`ErroresReportes`)

| Variante | Causa |
|---|---|
| `EleccionNoDisponible` | Se pasó `Rol::Ambos` como argumento a `listar_mejores_usuarios_por_rol` |

---

### 👤 Acciones disponibles para el usuario

Cualquier cuenta puede invocar los mensajes de lectura del contrato sin restricciones de rol ni permisos:

- **Consultar actividad de compradores**: ver cuántas órdenes generó cada usuario.
- **Ver los mejores compradores**: obtener el top 5 de compradores ordenados por calificación.
- **Ver los mejores vendedores**: obtener el top 5 de vendedores ordenados por calificación.
- **Ver productos más vendidos**: obtener el ranking de productos por unidades vendidas.
- **Ver estadísticas por categoría**: obtener ventas totales y calificación promedio por categoría.

Este contrato **no permite** modificar ningún dato: no emite calificaciones, no crea ni modifica órdenes, publicaciones ni usuarios. Su rol es exclusivamente de análisis y consulta.

---

### 🚀 Despliegue

Al desplegar este contrato es **obligatorio** pasarle al constructor la dirección on-chain del contrato de marketplace original.
Sin este parámetro el contrato no puede obtener datos y todas las consultas devolverán vectores vacíos. El `address` debe corresponder a un contrato `Sistema` correctamente desplegado en la misma red.

