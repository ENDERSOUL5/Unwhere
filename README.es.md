<div align="center">

# 🗑️ unwhere

**Desinstalador de paquetes universal para Linux**

Busca y elimina paquetes en **dnf**, **flatpak** y **pacman** desde una sola línea de comandos.

[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green)](#licencia)
[![Build](https://img.shields.io/github/actions/workflow/status/endersoul/unwhere/rust.yml?branch=master)](https://github.com/endersoul/unwhere/actions)

<p align="right">
  <a href="README.md">🇬🇧 English</a>
</p>

</div>

---

## 📋 Tabla de contenidos

- [Descripción](#descripción)
- [Características](#características)
- [Gestores de paquetes soportados](#gestores-de-paquetes-soportados)
- [Instalación](#instalación)
- [Uso](#uso)
- [Ejemplos](#ejemplos)
- [Cómo funciona](#cómo-funciona)
- [Construcción desde el código fuente](#construcción-desde-el-código-fuente)
- [Acerca de este proyecto](#acerca-de-este-proyecto)
- [Licencia](#licencia)

---

## Descripción

`unwhere` es una herramienta de línea de comandos escrita en Rust que simplifica la desinstalación de paquetes en distribuciones Linux. En lugar de recordar qué gestor de paquetes instaló cada aplicación, `unwhere` busca automáticamente en todos los gestores instalados y te permite eliminar lo que necesitas con un solo comando.

---

## Características

- 🔍 **Búsqueda universal** — Busca paquetes en múltiples gestores simultáneamente
- 🎯 **Búsqueda inteligente** — Usa expresiones regex con insensibilidad a mayúsculas/minúsculas
- ⚡ **Eliminación directa** — Si solo encuentra una coincidencia, la elimina automáticamente
- 🤔 **Selección interactiva** — Si encuentra varias coincidencias, te muestra opciones para elegir
- 🔒 **Manejo de permisos** — Ejecuta comandos con `sudo` cuando es necesario (dnf, pacman)
- 🦀 **Rendimiento** — Escrito en Rust para velocidad y fiabilidad

---

## Gestores de paquetes soportados

| Gestor      | Comando de listado                         | Comando de eliminación | ¿Requiere sudo? |
| ----------- | ------------------------------------------ | ---------------------- | --------------- |
| **dnf**     | `dnf list --installed`                     | `dnf rm`               | ✅ Sí            |
| **flatpak** | `flatpak list --app --columns=application` | `flatpak uninstall`    | ❌ No            |
| **pacman**  | `pacman -Q`                                | `pacman -R`            | ✅ Sí            |

---

## Instalación

### Desde GitHub Releases

```bash
# Descargar la última versión (reemplaza la URL con la versión correcta)
wget https://github.com/endersoul/unwhere/releases/latest/download/unwhere -O /usr/local/bin/unwhere

# Dar permisos de ejecución
chmod +x /usr/local/bin/unwhere
```

### Compilar desde el código fuente

```bash
# Clonar el repositorio
git clone https://github.com/endersoul/unwhere.git
cd unwhere

# Compilar en modo release
cargo build --release

# El binario estará en target/release/unwhere
# Copiarlo a una ubicación en tu PATH
cp target/release/unwhere /usr/local/bin/
```

---

## Uso

```bash
unwhere <nombre_del_paquete>
```

### Flujo básico

1. Ejecutas `unwhere` con el nombre (o parte del nombre) de un paquete
2. `unwhere` busca en **dnf**, **flatpak** y **pacman**
3. Si encuentra **1 resultado** → lo elimina automáticamente
4. Si encuentra **varios resultados** → te muestra una lista y te pide que elijas
5. Si **no encuentra nada** → muestra "no match found"

---

## Ejemplos

### Eliminar un paquete específico

```bash
$ unwhere firefox

# Si "firefox" solo aparece en un gestor, se elimina directamente
```

### Buscar con nombre parcial

```bash
$ unwhere code

# Podría encontrar: vscode, codeblocks, code-server, etc.
# Te mostraría algo como:
# (0) vscode flatpak
# (1) codeblocks dnf
# choose the option
# 1
```

### Búsqueda con expresión regular

```bash
# Buscar todos los paquetes que contengan "lib"
$ unwhere lib

# Buscar paquetes que empiecen con "gnome-"
$ unwhere ^gnome-
```

---

## Cómo funciona

```
┌─────────────────────────────────────────┐
│              unwhere                     │
├─────────────────────────────────────────┤
│  1. Recibe el nombre del paquete        │
│  2. Compila una regex (case-insensitive)│
│  3. Itera sobre cada gestor instalado:  │
│     ┌───────┐ ┌─────────┐ ┌─────────┐  │
│     │  dnf  │ │ flatpak │ │ pacman  │  │
│     └───┬───┘ └────┬────┘ └────┬────┘  │
│         │          │           │        │
│         ▼          ▼           ▼        │
│  4. Lista paquetes de cada gestor       │
│  5. Filtra por regex                    │
│  6. Si 1 match → elimina               │
│  7. Si N matches → muestra opciones     │
│  8. Si 0 matches → "no match found"    │
└─────────────────────────────────────────┘
```

---

## Construcción desde el código fuente

### Requisitos

- [Rust](https://www.rust-lang.org/tools/install) (edición 2024 o superior)
- Cargo (incluido con Rust)

### Dependencias

| Crate   | Versión | Uso                            |
| ------- | ------- | ------------------------------ |
| `regex` | 1.13.1  | Búsqueda de paquetes con regex |

### Compilar

```bash
# Modo debug
cargo build

# Modo release (recomendado para uso diario)
cargo build --release
```

---

## Acerca de este proyecto

> Este es **mi primer proyecto** programado en Rust. Como tal, probablemente tiene fallos, áreas de mejora y código que no es 100% idiomático. Cualquier contribución, issue o feedback es bienvenido y me ayudará a mejorar tanto el proyecto como mis habilidades como desarrollador.

### Limitaciones conocidas

- La búsqueda depende del formato de salida de cada gestor de paquetes; si un gestor cambia su formato, podría dejar de funcionar correctamente
- No se muestran las versiones de los paquetes encontrados
- La lista de gestores soportados está hardcodeada (aunque agregar nuevos es sencillo)

### Roadmap futuro

- [ ] Agregar más gestores de paquetes (apt, snap, brew...)

---

## Licencia

Este proyecto está bajo la licencia MIT. Consulta el archivo [LICENSE](LICENSE) para más detalles.

---

<div align="center">

Hecho con 🦀 Rust

</div>
