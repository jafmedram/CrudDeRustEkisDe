#![allow(dead_code)]
use std::error::Error;
use sqlx::FromRow;
use std::io;

#[derive(Debug, FromRow)]
struct Usuario {
    pub nombre: String,
    pub correo: String,
    pub usuarioid: String,
}

async fn create(usuario: &Usuario, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO usuarios (nombre, correo, usuarioid) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&usuario.nombre)
        .bind(&usuario.correo)
        .bind(&usuario.usuarioid)
        .execute(pool)
        .await?;

    Ok(())
}

async fn update(usuario: &Usuario, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE usuarios SET nombre = $1, correo = $2 WHERE usuarioid = $3";

    sqlx::query(query)
        .bind(&usuario.nombre)
        .bind(&usuario.correo)
        .bind(&usuario.usuarioid)
        .execute(pool)
        .await?;

    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Vec<Usuario>, Box<dyn Error>> {
    let q = "SELECT nombre, correo, usuarioid FROM usuarios";

    let usuarios: Vec<Usuario> = sqlx::query_as(q)
        .fetch_all(conn)
        .await?;

    for usuario in &usuarios {
        println!("--------------------");
        println!("Nombre: {}", usuario.nombre);
        println!("Correo: {}", usuario.correo);
        println!("Usuario ID: {}", usuario.usuarioid);
    };

    Ok(usuarios)
}

async fn delete(usuarioid: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let q = format!("DELETE FROM usuarios WHERE usuarioid = '{}'", usuarioid);

    sqlx::query(&q).execute(pool).await?;
    println!("Usuario eliminado exitosamente");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:Mimamamemim4.@localhost:5432/usuarios";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    loop {
        println!("Menú de opciones:");
        println!("1. Añadir usuario");
        println!("2. Editar usuario");
        println!("3. Leer usuarios");
        println!("4. Eliminar usuario");
        println!("5. Salir");

        println!("Ingrese su opción: ");
        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion)?;

        match opcion.trim() {
            "1" => {
                let usuario = capturar_usuario();
                create(&usuario, &pool).await?;
                println!("Añadiendo usuario...");
            }
            "2" => {
                let usuario = capturar_usuario();
                update(&usuario, &pool).await?;
                println!("Editando usuario...");
            }
            "3" => {
                println!("Leyendo usuarios...");
                read(&pool).await?;
                println!("Presione enter para continuar");
                io::stdin().read_line(&mut String::new())?;
            }
            "4" => {
                let identificador = capturar_identificador();
                delete(&identificador, &pool).await?;
                println!("Eliminando usuario...");
                println!("Presione enter para continuar");
                io::stdin().read_line(&mut String::new())?;
            }
            "5" => {
                println!("Saliendo del menú...");
                break;
            }
            _ => {
                println!("Opción inválida. Intente nuevamente.");
            }
        }
    }

    Ok(())
}

fn capturar_usuario() -> Usuario {
    println!("Ingrese el nombre del usuario:");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Error al leer la entrada");

    println!("Ingrese el correo del usuario:");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Error al leer la entrada");

    println!("Ingrese el Id del usuario:");
    let mut identificador = String::new();
    io::stdin().read_line(&mut identificador).expect("Error al leer la entrada");

    Usuario {
        nombre: nom.trim().to_string(),
        correo: email.trim().to_string(),
        usuarioid: identificador.trim().to_string(),
    }
}

fn capturar_identificador() -> String {
    println!("Ingrese el Id del usuario que desea eliminar:");
    let mut identificador = String::new();
    io::stdin().read_line(&mut identificador).expect("Error al leer la entrada");
    identificador.trim().to_string()
}
