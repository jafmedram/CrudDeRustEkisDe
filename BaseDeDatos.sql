create table usuarios(
	nombre varchar(100) not null,
	correo varchar(150) not null,
	usuarioid varchar(30) not null
);
create unique index usuarios_usuarioid_idx on usuarios(usuarioid)