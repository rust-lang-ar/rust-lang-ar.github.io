# Sitio de la Comunidad Argentina de Rust

Para contribuir al sitio:
- Crear un fork
- Editar o crear archivos markdown en la carpeta `./src`
- Commitear y pushear a tu fork.
- Crear un pull request contra `development` de `https://github.com/RustArgentina/rustargentina.github.io`. Es importante que sea contra esta branch y no otra.

## Notas

Github Pages, en el caso de un sitio de usuario o de organización, fuerza a que el contenido estático del sitio a deployar este en la carpeta raíz de la branch master (en el caso de sitios de proyectos, puede estar en la branch `gh-pages` o en la carpeta `./docs` de la branch `master`).

Por esta razón, los archivos fuente del sitio están en la branch `development`, y el Github Action workflow configurado en `.github/workflow/gh-pages.yml` se encarga de preparar un entorno en el cual se puede correr `mdbook` para generar el sitio estático y commitear y pushear ese contenido a la branch `master`.
