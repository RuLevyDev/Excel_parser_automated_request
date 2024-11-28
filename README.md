<div align="center">
<!-- Title: -->
<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1024px-Rust_programming_language_black_logo.svg.png" width="100" height="100">

<h1><a href="https://github.com/RuLevyDev
/">Ruben Levy
</a> -Rust</h1>

<!-- Labels: -->

<a href="https://github.com/RuLevyDev
/
Excel_parser_automated_request/actions/workflows/build.yml">
  <img src="https://github.com/RuLevyDev
/
Excel_parser_automated_request/actions/workflows/build.yml/badge.svg" height="20" alt="Build workflow">
</a>



```markdown
# Proyecto Rust: Envío de Actividades a un API

Este proyecto en Rust se encarga de cargar actividades desde un archivo Excel, convertirlas a formato JSON y enviarlas a un endpoint API utilizando solicitudes POST

## Requisitos

1. **Rust**: Asegúrate de tener Rust instalado. Puedes instalarlo desde [https://www.rust-lang.org](https://www.rust-lang.org).
2. **Cargo**: Cargo es el sistema de construcción y gestor de dependencias de Rust, y debe instalarse automáticamente con Rust.
3. **Dependencias**: El proyecto usa varias bibliotecas de Rust que se gestionan a través de `Cargo.toml`.

## Instalación

Para comenzar, clona este repositorio:

```bash
git clone https://github.com/RuLevyDev/Excel_parser_automated_request.git
cd nombre-del-repositorio
```

Instala las dependencias necesarias:

```bash
cargo build
```

## Uso

### 1. **Configurar el archivo Excel**

Este proyecto espera un archivo Excel que contenga actividades organizadas por sección. El archivo debe estar en formato `.xlsx` y debe seguir la estructura esperada.


### 2. **Cargar Actividades desde Excel y Enviar al API**

El programa carga actividades desde el archivo Excel y envía cada actividad a un endpoint utilizando una solicitud POST. Asegúrate de que el archivo Excel esté en la misma carpeta o especifica la ruta correcta.

Para ejecutar el programa:

```bash
cargo run
```

El programa buscará el archivo `programming-table-2.xlsx` y lo procesará, enviando cada actividad a la URL configurada en el código.

### 3. **Respuestas del API**

El programa manejará las respuestas del API. Si una solicitud es exitosa, se imprimirá el mensaje de éxito; si ocurre un error (por ejemplo, `401 Unauthorized`), se imprimirá el error correspondiente.

## Estructura del Proyecto

El proyecto está organizado de la siguiente manera:

```
.
├── src/                            # Carpeta principal con el código fuente
│   ├── main.rs                     # Archivo principal que ejecuta la lógica
│   ├── models/                     # Modelos y estructuras de datos
│   │   ├── activity_section.rs     # Definición de la estructura de la sección de actividades
│   │   ├── activity.rs             # Definición de la estructura de una actividad
│   │   ├── content.rs              # Definición de la estructura de contenido dependiendo del Language code
│   │   └── mod.rs                  # Archivo para el módulo models, incluye importaciones de otros módulos
│   ├── utils/                      # Funciones auxiliares para cargar actividades y hacer solicitudes
│   │   ├── excel.rs                # Función para cargar actividades desde un archivo Excel
│   │   ├── post_requests.rs        # Función para realizar solicitudes POST a un servidor
│   │   ├── parser.rs               # Funciones de análisis de datos (por ejemplo, para convertir los datos de Excel a estructuras)
│   │   └── mod.rs                  # Archivo para el módulo utils, incluye importaciones de otros módulos
├── Cargo.toml                      # Archivo de configuración del proyecto y dependencias
├── programming-table-2.xlsx        # Ejemplo de archivo Excel (reemplázalo con tu archivo real)
└── README.md                       # Documentación del proyecto

```

## Dependencias

Este proyecto utiliza las siguientes dependencias:

- `reqwest`: Para realizar solicitudes HTTP.
- `serde`: Para la serialización y deserialización de datos JSON.
- `serde_json`: Para manejar el formato JSON.
- `openpyxl`: Para leer archivos Excel (en caso de que se necesite leer datos Excel en formato `.xlsx`).

Estas dependencias se definen en el archivo `Cargo.toml`.

## Contribuciones

Si deseas contribuir al proyecto, por favor haz un fork del repositorio y envía tus cambios a través de una solicitud de extracción (pull request).

## Licencia

Este proyecto está bajo la Licencia MIT. Consulta el archivo [LICENSE](LICENSE) para más detalles.

## Contacto

Si tienes preguntas o comentarios, puedes ponerte en contacto conmigo a través de mi correo electrónico: [rulevydeveloper@gmail.com](mailto:rulevydeveloper@gmail.com).

```
 guía clara para otros desarrolladores sobre cómo configurar y ejecutar el proyecto en Rust.
