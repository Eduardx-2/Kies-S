# Kies-S

<p align="center">
  <img width="40%" height="90%" src="https://github.com/Eduardx-2/Kies-S/blob/main/kiesLog.png">
</p>
Kies-S es una herramienta de tipo command client para consultas a bases de datos. Construido usando RUST, sin uso de ORM.

<h2>Caracteristicas</h2>

- Mantiene todo localmente. (SQLITE)
- Facilita las consultas.
- Manejo interactivo con el usuario sobre las consultas.
- Comandos especificos que abrevian codigo SQL.

<h2>Instalación y Uso</h2>

 - ~arch@linux# `git clone https://github.com/Eduardx-2/Kies-S`
 - ~arch@linux# `cargo run --realese` o ~arch@linux# `cargo run -- <comando>(input o config)`
 - La función principal se dispara usando argumentos (input o config) -> `cargo run -- input` 

<h2>Configuración</h2>

* Las interacción con la bases de datos se realizan usando cargas SQL compuestas por variables dentro de un archivo de configuración.
* Tiene dos metodos de uso: -- input -> se encarga de manejar las consultas que el usuario necesite realizar haciendo uso de comandos basado en linea (["insert_info","show_info","update", "updt"]), metodo de configuración -- config se encarga de insertar las variables en el archivo de configuración.
* `-- TABLES: indica las tablas de las bases de datos
-- ROWSD: indica los nombres de las filas
-- SETTUPD: indica el update a las bases de datos
-- DBD: indica el nombre de la base de datos `

- Ejemplo de uso usando comando show_info
- <p align="center">
    <img width="80%" height="50%" src="https://github.com/Eduardx-2/Kies-S/blob/main/kiesAct.png">
  </p>
- soporte con SQLITE -> proxima actualización soporte (MARIADB,POSTGRESQL)

