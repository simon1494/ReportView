#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[allow(dead_code)]
#[ink::contract]
mod reportes {
    use scale_info::prelude::format;
    use ink::env::call::FromAccountId;
    use ink::prelude::{string::String, string::ToString, vec::Vec};
    use market::prelude::{Usuario, *};

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Debug, PartialEq)]
    pub enum ErroresReportes {
        NoHayProductosCreados,
        NoHayCategoriasCreadas,
        EleccionNoDisponible,
        NoHayUsuariosCreados,
        NoHayOrdenesCreadas,
        NoHayPublicacionesCreadas,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Clone)]
    pub struct OrdenesPorUsuario {
        pub nombre_usuario: String,
        pub cantidad_ordenes: u32,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Clone)]
    pub struct VentasPorProducto {
        pub nombre_producto: String,
        pub cantidad_ventas: u32,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Clone)]
    pub struct EstadisticasPorCategoria {
        pub nombre_categoria: String,
        pub cantidad_ventas: u32,
        pub promedio_calificacion: String,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Clone)]
    pub struct RatingPorUsuario {
        pub nombre_usuario: String,
        pub promedio_calificaciones: String,
    }

    pub trait ConsultasProductos {
        fn _productos_mas_vendidos(
            &self,
            productos: Vec<Producto>,
            ordenes: Vec<Orden>,
            publicaciones: Vec<Publicacion>,
        ) -> Vec<VentasPorProducto>;

        fn _mapear_nombres_productos(
            &self,
            publicaciones: Vec<Publicacion>,
            productos: &Vec<Producto>,
        ) -> Vec<(u32, u32, String)>;

        fn _contar_ventas_por_producto(
            &self,
            productos: &Vec<Producto>,
            ordenes: Vec<Orden>,
            publis_x_prod: Vec<(u32, u32, String)>,
        ) -> Vec<VentasPorProducto>;
    }

    pub trait ConsultasCategorias {
        fn _estadisticas_por_categoria(
            &self,
            categorias: Vec<Categoria>,
            productos: Vec<Producto>,
            publicaciones: Vec<Publicacion>,
            ordenes: Vec<Orden>,
        ) -> Vec<EstadisticasPorCategoria>;

        fn _mapear_productos_por_categoria(
            &self,
            categorias: &Vec<Categoria>,
            productos: &Vec<Producto>,
        ) -> Vec<(u32, String, Vec<u32>)>;

        fn _contar_ventas_por_categoria(
            &self,
            categoria_productos: &Vec<(u32, String, Vec<u32>)>,
            publicaciones: &Vec<Publicacion>,
            ordenes: &Vec<Orden>,
        ) -> Vec<(String, u32)>;

        fn _calcular_promedio_calificaciones_categoria(
            &self,
            categoria_productos: &Vec<(u32, String, Vec<u32>)>,
            publicaciones: &Vec<Publicacion>,
            ordenes: &Vec<Orden>,
        ) -> Vec<(String, String)>;

        fn _extraer_calificaciones_de_ordenes(&self, ordenes_categoria: &Vec<&Orden>) -> Vec<u8>;

        fn _calcular_promedio_de_calificaciones(&self, calificaciones: Vec<u8>) -> String;
    }

    pub trait ConsultasUsuarios {
        fn _cantidad_de_ordenes_por_usuario(
            &self,
            usuarios: Vec<Usuario>,
            ordenes: Vec<Orden>,
        ) -> Vec<OrdenesPorUsuario>;

        fn _mejores_usuarios_por_rol(
            &self,
            usuarios: Vec<Usuario>,
            target_role: &Rol,
        ) -> Vec<RatingPorUsuario>;

        fn _listar_mejores_cinco_usuarios(
            &self,
            usuario_filtrados: Vec<Usuario>,
            target_role: &Rol,
        ) -> Vec<RatingPorUsuario>;

        fn _filtrar_usuarios_por_rol_desc(
            &self,
            usuarios: Vec<Usuario>,
            target_role: &Rol,
        ) -> Vec<Usuario>;
    }

    #[ink(storage)]
    pub struct Reportes {
        pub original: SistemaRef,
    }

    impl Reportes {
        #[ink(constructor)]
        pub fn new(address: AccountId) -> Self {
            let original = SistemaRef::from_account_id(address);
            Self { original }
        }

        /// Retorna una listado con los nombres de todos los usuarios y sus órdenes generadas.
        #[ink(message)]
        pub fn listar_cantidad_de_ordenes_por_usuario(
            &self,
        ) -> Result<Vec<OrdenesPorUsuario>, ErroresReportes> {
            Ok(self._cantidad_de_ordenes_por_usuario(self._get_usuarios()?, self._get_ordenes()?))
        }

        #[ink(message)]
        /// Retorna una lista descendente de los cinco usuarios mejor calificados dentro de un rol.
        pub fn listar_mejores_usuarios_por_rol(
            &self,
            target_role: Rol,
        ) -> Result<Vec<RatingPorUsuario>, ErroresReportes> {
            self._eleccion_disponible(&target_role)?;
            Ok(self._mejores_usuarios_por_rol(self._get_usuarios()?, &target_role))
        }

        #[ink(message)]
        /// Retorna una lista descendente de los productos más vendidos.
        pub fn listar_productos_mas_vendidos(
            &self,
        ) -> Result<Vec<VentasPorProducto>, ErroresReportes> {
            Ok(self._productos_mas_vendidos(
                self._get_productos()?,
                self._get_ordenes()?,
                self._get_publicaciones()?,
            ))
        }

        #[ink(message)]
        /// Retorna estadísticas completas por categoría: ventas totales y calificación promedio.
        pub fn listar_estadisticas_por_categoria(
            &self,
        ) -> Result<Vec<EstadisticasPorCategoria>, ErroresReportes> {
            Ok(self._estadisticas_por_categoria(
                self._get_categorias()?,
                self._get_productos()?,
                self._get_publicaciones()?,
                self._get_ordenes()?,
            ))
        }

        fn _get_categorias(&self) -> Result<Vec<Categoria>, ErroresReportes> {
            let categorias: Vec<Categoria> = self.original.listar_categorias();
            if categorias.is_empty() {
                return Err(ErroresReportes::NoHayCategoriasCreadas);
            }
            Ok(categorias)
        }

        fn _get_productos(&self) -> Result<Vec<Producto>, ErroresReportes> {
            let productos: Vec<Producto> = self.original.listar_productos();
            if productos.is_empty() {
                return Err(ErroresReportes::NoHayProductosCreados);
            }
            Ok(productos)
        }

        fn _get_ordenes(&self) -> Result<Vec<Orden>, ErroresReportes> {
            let ordenes: Vec<Orden> = self.original.listar_ordenes();
            if ordenes.is_empty() {
                return Err(ErroresReportes::NoHayOrdenesCreadas);
            }
            Ok(ordenes)
        }

        fn _get_usuarios(&self) -> Result<Vec<Usuario>, ErroresReportes> {
            let usuarios: Vec<Usuario> = self.original.listar_usuarios();
            if usuarios.is_empty() {
                return Err(ErroresReportes::NoHayUsuariosCreados);
            }
            Ok(usuarios)
        }

        fn _get_publicaciones(&self) -> Result<Vec<Publicacion>, ErroresReportes> {
            let publicaciones = self.original.listar_publicaciones();
            if publicaciones.is_empty() {
                return Err(ErroresReportes::NoHayPublicacionesCreadas);
            }
            Ok(publicaciones)
        }

        fn _eleccion_disponible(&self, eleccion_rol: &Rol) -> Result<(), ErroresReportes> {
            if eleccion_rol == &Rol::Ambos {
                return Err(ErroresReportes::EleccionNoDisponible);
            }
            Ok(())
        }
    }

    impl ConsultasProductos for Reportes {
        fn _productos_mas_vendidos(
            &self,
            productos: Vec<Producto>,
            ordenes: Vec<Orden>,
            publicaciones: Vec<Publicacion>,
        ) -> Vec<VentasPorProducto> {
            let productos_x_publicaciones: Vec<(u32, u32, String)> =
                self._mapear_nombres_productos(publicaciones, &productos);

            let mut ventas_por_producto: Vec<VentasPorProducto> =
                self._contar_ventas_por_producto(&productos, ordenes, productos_x_publicaciones);

            ventas_por_producto.sort_by(|a: &VentasPorProducto, b: &VentasPorProducto| {
                b.cantidad_ventas.cmp(&a.cantidad_ventas)
            });
            ventas_por_producto
        }

        fn _mapear_nombres_productos(
            &self,
            publicaciones: Vec<Publicacion>,
            productos: &Vec<Producto>,
        ) -> Vec<(u32, u32, String)> {
            let mut pub_to_prod: Vec<(u32, u32, String)> = Vec::new();
            for pub_ in &publicaciones {
                let id_pub: u32 = pub_.get_id();
                let id_prod_publi: u32 = pub_.get_id_producto();
                for producto in productos {
                    if id_prod_publi == producto.get_id() {
                        pub_to_prod.push((id_pub, id_prod_publi, producto.get_nombre()));
                        break;
                    }
                }
            }
            pub_to_prod
        }

        fn _contar_ventas_por_producto(
            &self,
            productos: &Vec<Producto>,
            ordenes: Vec<Orden>,
            publis_x_prod: Vec<(u32, u32, String)>,
        ) -> Vec<VentasPorProducto> {
            let mut ventas_por_producto: Vec<VentasPorProducto> = Vec::new();
            for producto in productos {
                let mut total_ventas: u32 = 0;

                for orden in &ordenes {
                    if orden.get_status() == EstadoOrden::Recibida {
                        // Buscar si esta orden corresponde a este producto
                        for (pub_id, prod_id, _) in &publis_x_prod {
                            if orden.get_id_pub() == *pub_id && *prod_id == producto.get_id() {
                                total_ventas = total_ventas.saturating_add(orden.get_cantidad());
                                break;
                            }
                        }
                    }
                }

                let item: VentasPorProducto = VentasPorProducto {
                    nombre_producto: producto.get_nombre(),
                    cantidad_ventas: total_ventas,
                };
                ventas_por_producto.push(item);
            }
            ventas_por_producto
        }
    }

    impl ConsultasUsuarios for Reportes {
        fn _cantidad_de_ordenes_por_usuario(
            &self,
            usuarios: Vec<Usuario>,
            ordenes: Vec<Orden>,
        ) -> Vec<OrdenesPorUsuario> {
            let mut reporte: Vec<OrdenesPorUsuario> = Vec::new();

            for usuario in usuarios {
                let mut contador: u32 = 0;
                for orden in &ordenes {
                    if orden.get_id_comprador() == usuario.get_id() {
                        contador = contador.saturating_add(1);
                    }
                }
                let item = OrdenesPorUsuario {
                    nombre_usuario: usuario.get_name(),
                    cantidad_ordenes: contador,
                };
                reporte.push(item);
            }

            reporte
        }

        fn _mejores_usuarios_por_rol(
            &self,
            usuarios: Vec<Usuario>,
            target_role: &Rol,
        ) -> Vec<RatingPorUsuario> {
            let usuarios_filtrados: Vec<Usuario> =
                self._filtrar_usuarios_por_rol_desc(usuarios, target_role);
            let reporte: Vec<RatingPorUsuario> =
                self._listar_mejores_cinco_usuarios(usuarios_filtrados, target_role);
            reporte
        }

        fn _listar_mejores_cinco_usuarios(
            &self,
            usuarios_filtrados: Vec<Usuario>,
            target_role: &Rol,
        ) -> Vec<RatingPorUsuario> {
            let mut top_5: Vec<RatingPorUsuario> = Vec::new();
            for (count, u) in usuarios_filtrados.into_iter().enumerate() {
                if count >= 5 {
                    break;
                }
                let promedio_str: String = match target_role {
                    Rol::Comprador => u.get_calificacion_comprador().unwrap_or("0.0".to_string()),
                    Rol::Vendedor => u.get_calificacion_vendedor().unwrap_or("0.0".to_string()),
                    _ => "0.0".to_string(),
                };

                let item: RatingPorUsuario = RatingPorUsuario {
                    nombre_usuario: u.get_name(),
                    promedio_calificaciones: promedio_str,
                };
                top_5.push(item);
            }

            top_5
        }

        // Recibe un vector de usuarios y un Rol y retorna solo los usuarios con ese rol
        fn _filtrar_usuarios_por_rol_desc(
            &self,
            usuarios: Vec<Usuario>,
            target_role: &Rol,
        ) -> Vec<Usuario> {
            let mut usuarios_filtrados: Vec<Usuario> = Vec::new();

            for usuario in usuarios {
                if usuario.has_role(target_role.clone()) {
                    usuarios_filtrados.push(usuario);
                }
            }

            //Ordenar por calificación de mayor a menor
            usuarios_filtrados.sort_by(|a, b| {
                let cal_a = match target_role {
                    Rol::Comprador => a.get_calificacion_comprador().unwrap_or("0.0".to_string()),
                    Rol::Vendedor => a.get_calificacion_vendedor().unwrap_or("0.0".to_string()),
                    _ => "0.0".to_string(),
                };
                let cal_b = match target_role {
                    Rol::Comprador => b.get_calificacion_comprador().unwrap_or("0.0".to_string()),
                    Rol::Vendedor => b.get_calificacion_vendedor().unwrap_or("0.0".to_string()),
                    _ => "0.0".to_string(),
                };

                // Orden descendente (mayor a menor) - comparación directa de strings
                cal_b.cmp(&cal_a)
            });

            usuarios_filtrados
        }
    }

    impl ConsultasCategorias for Reportes {
        fn _estadisticas_por_categoria(
            &self,
            categorias: Vec<Categoria>,
            productos: Vec<Producto>,
            publicaciones: Vec<Publicacion>,
            ordenes: Vec<Orden>,
        ) -> Vec<EstadisticasPorCategoria> {
            let categoria_productos = self._mapear_productos_por_categoria(&categorias, &productos);
            let ventas_por_categoria =
                self._contar_ventas_por_categoria(&categoria_productos, &publicaciones, &ordenes);
            let calificaciones_por_categoria = self._calcular_promedio_calificaciones_categoria(
                &categoria_productos,
                &publicaciones,
                &ordenes,
            );

            let mut estadisticas = Vec::new();
            for (nombre_categoria, total_ventas) in ventas_por_categoria {
                let promedio_calificacion = calificaciones_por_categoria
                    .iter()
                    .find(|(nombre, _)| nombre == &nombre_categoria)
                    .map(|(_, promedio)| promedio.clone())
                    .unwrap_or("0.0".to_string());

                estadisticas.push(EstadisticasPorCategoria {
                    nombre_categoria,
                    cantidad_ventas: total_ventas,
                    promedio_calificacion,
                });
            }
            estadisticas
        }

        fn _mapear_productos_por_categoria(
            &self,
            categorias: &Vec<Categoria>,
            productos: &Vec<Producto>,
        ) -> Vec<(u32, String, Vec<u32>)> {
            let mut categoria_productos = Vec::new();

            for categoria in categorias {
                let mut productos_ids = Vec::new();
                for producto in productos {
                    if producto.get_id_categoria() == categoria.get_id() {
                        productos_ids.push(producto.get_id());
                    }
                }
                categoria_productos.push((
                    categoria.get_id(),
                    categoria.get_nombre(),
                    productos_ids,
                ));
            }
            categoria_productos
        }

        fn _contar_ventas_por_categoria(
            &self,
            categoria_productos: &Vec<(u32, String, Vec<u32>)>,
            publicaciones: &Vec<Publicacion>,
            ordenes: &Vec<Orden>,
        ) -> Vec<(String, u32)> {
            let mut ventas_por_categoria = Vec::new();

            for (_, nombre_categoria, productos_ids) in categoria_productos {
                let mut total_ventas = 0u32;

                for producto_id in productos_ids {
                    for publicacion in publicaciones {
                        if publicacion.get_id_producto() == *producto_id {
                            for orden in ordenes {
                                if orden.get_status() == EstadoOrden::Recibida
                                    && orden.get_id_pub() == publicacion.get_id()
                                {
                                    total_ventas =
                                        total_ventas.saturating_add(orden.get_cantidad());
                                }
                            }
                        }
                    }
                }
                ventas_por_categoria.push((nombre_categoria.clone(), total_ventas));
            }
            ventas_por_categoria
        }

        fn _calcular_promedio_calificaciones_categoria(
            &self,
            categoria_productos: &Vec<(u32, String, Vec<u32>)>,
            publicaciones: &Vec<Publicacion>,
            ordenes: &Vec<Orden>,
        ) -> Vec<(String, String)> {
            let mut promedios_por_categoria = Vec::new();

            for (_, nombre_categoria, productos_ids) in categoria_productos {
                let mut ordenes_categoria = Vec::new();

                for producto_id in productos_ids {
                    for publicacion in publicaciones {
                        if publicacion.get_id_producto() == *producto_id {
                            for orden in ordenes {
                                if orden.get_status() == EstadoOrden::Recibida
                                    && orden.get_id_pub() == publicacion.get_id()
                                {
                                    ordenes_categoria.push(orden);
                                }
                            }
                        }
                    }
                }

                let calificaciones = self._extraer_calificaciones_de_ordenes(&ordenes_categoria);
                let promedio = self._calcular_promedio_de_calificaciones(calificaciones);
                promedios_por_categoria.push((nombre_categoria.clone(), promedio));
            }
            promedios_por_categoria
        }

        fn _extraer_calificaciones_de_ordenes(&self, ordenes_categoria: &Vec<&Orden>) -> Vec<u8> {
            let mut calificaciones = Vec::new();
            for orden in ordenes_categoria {
                if let Some(calificacion) = orden.get_calificacion_vendedor() {
                    calificaciones.push(calificacion);
                }
            }
            calificaciones
        }

        fn _calcular_promedio_de_calificaciones(&self, calificaciones: Vec<u8>) -> String {
            if calificaciones.is_empty() {
                return "0.0".to_string();
            }

            let suma: u32 = calificaciones.iter().map(|&c| c as u32).sum();
            let cantidad = calificaciones.len() as u32;
            let promedio_escalado = suma.saturating_mul(10).checked_div(cantidad).unwrap_or(0);
            let parte_entera = promedio_escalado / 10;
            let parte_decimal = promedio_escalado % 10;
            format!("{}.{}", parte_entera, parte_decimal)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::reportes::*;
    use market::prelude::*;
    use ink::{
        primitives::AccountId,
    };
    use ink::env::call::FromAccountId;

    // Helper function para crear un usuario de prueba
    fn crear_usuario_test(
        id: AccountId,
        nombre: &str,
        email: &str,
        rol: Rol,
        calificacion_comprador: (u32, u32),
        calificacion_vendedor: (u32, u32),
    ) -> Usuario {
        let mut rating = Rating::new();
        rating.calificacion_comprador = calificacion_comprador;
        rating.calificacion_vendedor = calificacion_vendedor;
        
        let mut nuevo: Usuario = Usuario::new(id, nombre.to_string(), email.to_string());
        nuevo.set_roles(vec![rol]);
        nuevo.set_rating(rating);
        nuevo
    }

    // Helper function para crear AccountId de prueba
    fn crear_account_id(value: u8) -> AccountId {
        AccountId::from([value; 32])
    }

    #[test]
    fn test_filtrar_usuarios_por_rol_desc_comprador() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let usuarios = vec![
            crear_usuario_test(
                crear_account_id(1),
                "Usuario1",
                "user1@test.com",
                Rol::Comprador,
                (450, 100), // 4.5 como comprador
                (0, 0),     // Sin calificaciones como vendedor
            ),
            crear_usuario_test(
                crear_account_id(2),
                "Usuario2",
                "user2@test.com",
                Rol::Comprador,
                (320, 100), // 3.2 como comprador
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(3),
                "Usuario3",
                "user3@test.com",
                Rol::Comprador,
                (500, 100), // 5.0 como comprador
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(4),
                "Usuario4",
                "user4@test.com",
                Rol::Vendedor, // Este no debería ser incluido
                (0, 0),
                (420, 100),
            ),
            crear_usuario_test(
                crear_account_id(5),
                "Usuario5",
                "user5@test.com",
                Rol::Comprador,
                (390, 100), // 3.9 como comprador
                (0, 0),
            ),
        ];

        let target_role = Rol::Comprador;

        // Act
        let resultado = reportes._filtrar_usuarios_por_rol_desc(usuarios, &target_role);

        // Assert
        assert_eq!(resultado.len(), 4, "Solo 4 usuarios tienen rol Comprador");
        
        // Verificar que están ordenados por calificación de comprador descendente
        // Orden esperado: 5.0, 4.5, 3.9, 3.2
        assert_eq!(resultado[0].get_name(), "Usuario3"); // 5.0
        assert_eq!(resultado[1].get_name(), "Usuario1"); // 4.5
        assert_eq!(resultado[2].get_name(), "Usuario5"); // 3.9
        assert_eq!(resultado[3].get_name(), "Usuario2"); // 3.2
    }

    #[test]
    fn test_filtrar_usuarios_por_rol_desc_vendedor() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let usuarios = vec![
            crear_usuario_test(
                crear_account_id(1),
                "Usuario1",
                "user1@test.com",
                Rol::Vendedor,
                (0, 0), // Sin calificaciones como comprador
                (300, 100), // 3.0 como vendedor
            ),
            crear_usuario_test(
                crear_account_id(2),
                "Usuario2",
                "user2@test.com",
                Rol::Vendedor,
                (0, 0),
                (480, 100), // 4.8 como vendedor
            ),
            crear_usuario_test(
                crear_account_id(3),
                "Usuario3",
                "user3@test.com",
                Rol::Comprador, // Este no debería ser incluido
                (500, 100),
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(4),
                "Usuario4",
                "user4@test.com",
                Rol::Vendedor,
                (0, 0),
                (420, 100), // 4.2 como vendedor
            ),
            crear_usuario_test(
                crear_account_id(5),
                "Usuario5",
                "user5@test.com",
                Rol::Vendedor,
                (0, 0),
                (350, 100), // 3.5 como vendedor
            ),
        ];

        let target_role = Rol::Vendedor;

        // Act
        let resultado = reportes._filtrar_usuarios_por_rol_desc(usuarios, &target_role);

        // Assert
        assert_eq!(resultado.len(), 4, "Solo 4 usuarios tienen rol Vendedor");
        
        // Verificar que están ordenados por calificación de vendedor descendente
        // Orden esperado: 4.8, 4.2, 3.5, 3.0
        assert_eq!(resultado[0].get_name(), "Usuario2"); // 4.8
        assert_eq!(resultado[1].get_name(), "Usuario4"); // 4.2
        assert_eq!(resultado[2].get_name(), "Usuario5"); // 3.5
        assert_eq!(resultado[3].get_name(), "Usuario1"); // 3.0
    }

    #[test]
    fn test_listar_mejores_cinco_usuarios_vendedores() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let mut usuarios_filtrados = vec![
            crear_usuario_test(
                crear_account_id(1),
                "Vendedor1",
                "v1@test.com",
                Rol::Vendedor,
                (0, 0),
                (450, 100), // 4.5 como vendedor
            ),
            crear_usuario_test(
                crear_account_id(2),
                "Vendedor2",
                "v2@test.com",
                Rol::Vendedor,
                (0, 0),
                (320, 100), // 3.2 como vendedor
            ),
            crear_usuario_test(
                crear_account_id(3),
                "Vendedor3",
                "v3@test.com",
                Rol::Vendedor,
                (0, 0),
                (500, 100), // 5.0 como vendedor - el mejor
            ),
            crear_usuario_test(
                crear_account_id(4),
                "Vendedor4",
                "v4@test.com",
                Rol::Vendedor,
                (0, 0),
                (180, 100), // 1.8 como vendedor - el peor
            ),
            crear_usuario_test(
                crear_account_id(5),
                "Vendedor5",
                "v5@test.com",
                Rol::Vendedor,
                (0, 0),
                (390, 100), // 3.9 como vendedor
            ),
            crear_usuario_test(
                crear_account_id(6),
                "Vendedor6",
                "v6@test.com",
                Rol::Vendedor,
                (0, 0),
                (470, 100), // 4.7 como vendedor
            ),
        ];

        // Ordenar usuarios manualmente como lo haría _filtrar_usuarios_por_rol_desc
        let target_role = Rol::Vendedor;
        usuarios_filtrados.sort_by(|a, b| {
            let cal_a = a.get_calificacion_vendedor().unwrap_or("0.0".to_string());
            let cal_b = b.get_calificacion_vendedor().unwrap_or("0.0".to_string());
            cal_b.cmp(&cal_a) // Orden descendente
        });

        // Act
        let resultado = reportes._listar_mejores_cinco_usuarios(usuarios_filtrados, &target_role);

        // Assert
        assert_eq!(resultado.len(), 5, "Debe retornar solo los 5 mejores usuarios");

        // Verificar orden descendente: 5.0, 4.7, 4.5, 3.9, 3.2 (excluye 1.8)
        assert_eq!(resultado[0].nombre_usuario, "Vendedor3"); // 5.0
        assert_eq!(resultado[0].promedio_calificaciones, "5.0");
        
        assert_eq!(resultado[1].nombre_usuario, "Vendedor6"); // 4.7
        assert_eq!(resultado[1].promedio_calificaciones, "4.7");
        
        assert_eq!(resultado[2].nombre_usuario, "Vendedor1"); // 4.5
        assert_eq!(resultado[2].promedio_calificaciones, "4.5");
        
        assert_eq!(resultado[3].nombre_usuario, "Vendedor5"); // 3.9
        assert_eq!(resultado[3].promedio_calificaciones, "3.9");
        
        assert_eq!(resultado[4].nombre_usuario, "Vendedor2"); // 3.2
        assert_eq!(resultado[4].promedio_calificaciones, "3.2");

        // Verificar que el sexto usuario (el peor) no está incluido
        assert!(resultado.iter().all(|u| u.nombre_usuario != "Vendedor4"));
    }

    #[test]
    fn test_listar_mejores_cinco_usuarios_compradores() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let mut usuarios_filtrados = vec![
            crear_usuario_test(
                crear_account_id(1),
                "Comprador1",
                "c1@test.com",
                Rol::Comprador,
                (280, 100), // 2.8 como comprador
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(2),
                "Comprador2",
                "c2@test.com",
                Rol::Comprador,
                (490, 100), // 4.9 como comprador
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(3),
                "Comprador3",
                "c3@test.com",
                Rol::Comprador,
                (350, 100), // 3.5 como comprador
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(4),
                "Comprador4",
                "c4@test.com",
                Rol::Comprador,
                (500, 100), // 5.0 como comprador - el mejor
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(5),
                "Comprador5",
                "c5@test.com",
                Rol::Comprador,
                (150, 100), // 1.5 como comprador - el peor
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(6),
                "Comprador6",
                "c6@test.com",
                Rol::Comprador,
                (420, 100), // 4.2 como comprador
                (0, 0),
            ),
        ];

        // Ordenar usuarios manualmente como lo haría _filtrar_usuarios_por_rol_desc
        let target_role = Rol::Comprador;
        usuarios_filtrados.sort_by(|a, b| {
            let cal_a = a.get_calificacion_comprador().unwrap_or("0.0".to_string());
            let cal_b = b.get_calificacion_comprador().unwrap_or("0.0".to_string());
            cal_b.cmp(&cal_a) // Orden descendente
        });

        // Act
        let resultado = reportes._listar_mejores_cinco_usuarios(usuarios_filtrados, &target_role);

        // Assert
        assert_eq!(resultado.len(), 5, "Debe retornar solo los 5 mejores usuarios");

        // Verificar orden descendente: 5.0, 4.9, 4.2, 3.5, 2.8 (excluye 1.5)
        assert_eq!(resultado[0].nombre_usuario, "Comprador4"); // 5.0
        assert_eq!(resultado[0].promedio_calificaciones, "5.0");
        
        assert_eq!(resultado[1].nombre_usuario, "Comprador2"); // 4.9
        assert_eq!(resultado[1].promedio_calificaciones, "4.9");
        
        assert_eq!(resultado[2].nombre_usuario, "Comprador6"); // 4.2
        assert_eq!(resultado[2].promedio_calificaciones, "4.2");
        
        assert_eq!(resultado[3].nombre_usuario, "Comprador3"); // 3.5
        assert_eq!(resultado[3].promedio_calificaciones, "3.5");
        
        assert_eq!(resultado[4].nombre_usuario, "Comprador1"); // 2.8
        assert_eq!(resultado[4].promedio_calificaciones, "2.8");

        // Verificar que el sexto usuario (el peor) no está incluido
        assert!(resultado.iter().all(|u| u.nombre_usuario != "Comprador5"));
    }

    // Helper function para crear una orden de prueba
    fn crear_orden_test(
        id: u32,
        id_comprador: AccountId,
        id_vendedor: AccountId,
        id_publicacion: u32,
        cantidad: u32,
    ) -> Orden {
        Orden::new(id, id_publicacion, id_vendedor, id_comprador, cantidad, 0) // precio_total = 0 por defecto
    }

    #[test]
    fn test_mejores_usuarios_por_rol_comprador() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let usuarios = vec![
            crear_usuario_test(
                crear_account_id(1),
                "Comprador1",
                "c1@test.com",
                Rol::Comprador,
                (450, 100), // 4.5 como comprador
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(2),
                "Comprador2",
                "c2@test.com",
                Rol::Comprador,
                (320, 100), // 3.2 como comprador
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(3),
                "Comprador3",
                "c3@test.com",
                Rol::Comprador,
                (500, 100), // 5.0 como comprador - el mejor
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(4),
                "Vendedor1",
                "v1@test.com",
                Rol::Vendedor, // Este no debería ser incluido
                (0, 0),
                (420, 100),
            ),
            crear_usuario_test(
                crear_account_id(5),
                "Comprador4",
                "c4@test.com",
                Rol::Comprador,
                (390, 100), // 3.9 como comprador
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(6),
                "Comprador5",
                "c5@test.com",
                Rol::Comprador,
                (280, 100), // 2.8 como comprador
                (0, 0),
            ),
        ];

        let target_role = Rol::Comprador;

        // Act
        let resultado = reportes._mejores_usuarios_por_rol(usuarios, &target_role);

        // Assert
        assert_eq!(resultado.len(), 5, "Debe retornar solo los 5 mejores compradores");

        // Verificar que están ordenados por calificación descendente y limitados a 5
        // Orden esperado: 5.0, 4.5, 3.9, 3.2, 2.8 (excluye al vendedor)
        assert_eq!(resultado[0].nombre_usuario, "Comprador3"); // 5.0
        assert_eq!(resultado[0].promedio_calificaciones, "5.0");
        
        assert_eq!(resultado[1].nombre_usuario, "Comprador1"); // 4.5
        assert_eq!(resultado[1].promedio_calificaciones, "4.5");
        
        assert_eq!(resultado[2].nombre_usuario, "Comprador4"); // 3.9
        assert_eq!(resultado[2].promedio_calificaciones, "3.9");
        
        assert_eq!(resultado[3].nombre_usuario, "Comprador2"); // 3.2
        assert_eq!(resultado[3].promedio_calificaciones, "3.2");
        
        assert_eq!(resultado[4].nombre_usuario, "Comprador5"); // 2.8
        assert_eq!(resultado[4].promedio_calificaciones, "2.8");

        // Verificar que el vendedor no está incluido
        assert!(resultado.iter().all(|u| u.nombre_usuario != "Vendedor1"));
    }

    #[test]
    fn test_cantidad_de_ordenes_por_usuario() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let usuarios = vec![
            crear_usuario_test(
                crear_account_id(1),
                "Usuario1",
                "u1@test.com",
                Rol::Comprador,
                (0, 0),
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(2),
                "Usuario2",
                "u2@test.com",
                Rol::Comprador,
                (0, 0),
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(3),
                "Usuario3",
                "u3@test.com",
                Rol::Comprador,
                (0, 0),
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(4),
                "Usuario4",
                "u4@test.com",
                Rol::Comprador,
                (0, 0),
                (0, 0),
            ),
            crear_usuario_test(
                crear_account_id(5),
                "Usuario5",
                "u5@test.com",
                Rol::Comprador,
                (0, 0),
                (0, 0),
            ),
        ];

        let ordenes = vec![
            crear_orden_test(1, crear_account_id(1), crear_account_id(10), 1, 2), // Usuario1 - 1 orden
            crear_orden_test(2, crear_account_id(2), crear_account_id(10), 2, 1), // Usuario2 - 1 orden
            crear_orden_test(3, crear_account_id(2), crear_account_id(10), 3, 3), // Usuario2 - 2 orden
            crear_orden_test(4, crear_account_id(3), crear_account_id(10), 4, 1), // Usuario3 - 1 orden
            crear_orden_test(5, crear_account_id(3), crear_account_id(10), 5, 2), // Usuario3 - 2 orden
            crear_orden_test(6, crear_account_id(3), crear_account_id(10), 6, 1), // Usuario3 - 3 orden
            crear_orden_test(7, crear_account_id(4), crear_account_id(10), 7, 1), // Usuario4 - 1 orden
            crear_orden_test(8, crear_account_id(5), crear_account_id(10), 8, 2), // Usuario5 - 1 orden
            crear_orden_test(9, crear_account_id(5), crear_account_id(10), 9, 1), // Usuario5 - 2 orden
        ];

        // Act
        let resultado = reportes._cantidad_de_ordenes_por_usuario(usuarios, ordenes);

        // Assert
        assert_eq!(resultado.len(), 5, "Debe retornar info para todos los usuarios");

        // Verificar cantidad de órdenes por usuario
        assert_eq!(resultado[0].nombre_usuario, "Usuario1");
        assert_eq!(resultado[0].cantidad_ordenes, 1, "Usuario1 debe tener 1 orden");
        
        assert_eq!(resultado[1].nombre_usuario, "Usuario2");
        assert_eq!(resultado[1].cantidad_ordenes, 2, "Usuario2 debe tener 2 órdenes");
        
        assert_eq!(resultado[2].nombre_usuario, "Usuario3");
        assert_eq!(resultado[2].cantidad_ordenes, 3, "Usuario3 debe tener 3 órdenes");
        
        assert_eq!(resultado[3].nombre_usuario, "Usuario4");
        assert_eq!(resultado[3].cantidad_ordenes, 1, "Usuario4 debe tener 1 orden");
        
        assert_eq!(resultado[4].nombre_usuario, "Usuario5");
        assert_eq!(resultado[4].cantidad_ordenes, 2, "Usuario5 debe tener 2 órdenes");
    }
}