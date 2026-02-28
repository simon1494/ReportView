#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[allow(dead_code)]
#[ink::contract]
mod reportes {
    use ink::env::call::FromAccountId;
    use ink::prelude::{string::String, string::ToString, vec::Vec};
    use market::prelude::{Usuario, *};
    use scale_info::prelude::format;

    /// Errores que pueden ocurrir durante la generación de reportes.
    /// Representa diferentes casos donde no hay suficientes datos para generar el reporte solicitado.
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

    /// Representa la cantidad de órdenes generadas por un usuario específico.
    /// Utilizado en reportes para mostrar la actividad de compras de cada usuario.
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Clone)]
    pub struct OrdenesPorUsuario {
        pub nombre_usuario: String,
        pub cantidad_ordenes: u32,
    }

    /// Representa las ventas totales de un producto específico.
    /// Contiene el nombre del producto y la cantidad total de unidades vendidas.
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Clone)]
    pub struct VentasPorProducto {
        pub nombre_producto: String,
        pub cantidad_ventas: u32,
    }

    /// Representa estadísticas completas de una categoría de productos.
    /// Incluye el total de ventas y el promedio de calificaciones de todos los productos de la categoría.
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Clone)]
    pub struct EstadisticasPorCategoria {
        pub nombre_categoria: String,
        pub cantidad_ventas: u32,
        pub promedio_calificacion: String,
    }

    /// Representa la calificación promedio de un usuario en un rol específico.
    /// Utilizado para mostrar los usuarios mejor calificados como compradores o vendedores.
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Clone)]
    #[derive(Debug, PartialEq)]
    pub struct RatingPorUsuario {
        pub nombre_usuario: String,
        pub promedio_calificaciones: String,
    }

    /// Trait que define operaciones para generar reportes relacionados con productos.
    /// Incluye funcionalidades para analizar ventas, mapear relaciones entre publicaciones y productos,
    /// y contar las ventas por producto.
    pub trait ConsultasProductos {
        /// Genera un reporte de los productos más vendidos ordenados de forma descendente.
        /// 
        /// # Parámetros
        /// * `productos` - Vector de productos disponibles en el sistema
        /// * `ordenes` - Vector de órdenes para contar las ventas
        /// * `publicaciones` - Vector de publicaciones que relacionan productos con órdenes
        /// 
        /// # Retorna
        /// Vector de `VentasPorProducto` ordenado por cantidad de ventas (mayor a menor)
        fn _productos_mas_vendidos(
            &self,
            productos: Vec<Producto>,
            ordenes: Vec<Orden>,
            publicaciones: Vec<Publicacion>,
        ) -> Vec<VentasPorProducto>;

        /// Mapea las publicaciones con sus productos correspondientes.
        /// 
        /// # Parámetros
        /// * `publicaciones` - Vector de publicaciones a mapear
        /// * `productos` - Referencia al vector de productos para obtener nombres
        /// 
        /// # Retorna
        /// Vector de tuplas (id_publicacion, id_producto, nombre_producto)
        fn _mapear_nombres_productos(
            &self,
            publicaciones: Vec<Publicacion>,
            productos: &Vec<Producto>,
        ) -> Vec<(u32, u32, String)>;

        /// Cuenta las ventas totales por cada producto basándose en órdenes recibidas.
        /// 
        /// # Parámetros
        /// * `productos` - Referencia al vector de productos
        /// * `ordenes` - Vector de órdenes para contar ventas
        /// * `publis_x_prod` - Mapeo entre publicaciones y productos
        /// 
        /// # Retorna
        /// Vector de `VentasPorProducto` con el conteo de ventas por producto
        fn _contar_ventas_por_producto(
            &self,
            productos: &Vec<Producto>,
            ordenes: Vec<Orden>,
            publis_x_prod: Vec<(u32, u32, String)>,
        ) -> Vec<VentasPorProducto>;
    }

    /// Trait que define operaciones para generar reportes relacionados con categorías de productos.
    /// Incluye funcionalidades para calcular estadísticas de ventas y calificaciones por categoría.
    pub trait ConsultasCategorias {
        /// Genera estadísticas completas por categoría incluyendo ventas y calificaciones promedio.
        /// 
        /// # Parámetros
        /// * `categorias` - Vector de categorías del sistema
        /// * `productos` - Vector de productos para mapear con categorías
        /// * `publicaciones` - Vector de publicaciones para relacionar con órdenes
        /// * `ordenes` - Vector de órdenes para calcular estadísticas
        /// 
        /// # Retorna
        /// Vector de `EstadisticasPorCategoria` con ventas totales y promedio de calificaciones
        fn _estadisticas_por_categoria(
            &self,
            categorias: Vec<Categoria>,
            productos: Vec<Producto>,
            publicaciones: Vec<Publicacion>,
            ordenes: Vec<Orden>,
        ) -> Vec<EstadisticasPorCategoria>;

        /// Mapea los productos por categoría para facilitar el análisis estadístico.
        /// 
        /// # Parámetros
        /// * `categorias` - Referencia al vector de categorías
        /// * `productos` - Referencia al vector de productos
        /// 
        /// # Retorna
        /// Vector de tuplas (id_categoria, nombre_categoria, vector_ids_productos)
        fn _mapear_productos_por_categoria(
            &self,
            categorias: &Vec<Categoria>,
            productos: &Vec<Producto>,
        ) -> Vec<(u32, String, Vec<u32>)>;

        /// Cuenta las ventas totales por categoría sumando las ventas de todos sus productos.
        /// 
        /// # Parámetros
        /// * `categoria_productos` - Mapeo de categorías con sus productos
        /// * `publicaciones` - Referencia al vector de publicaciones
        /// * `ordenes` - Referencia al vector de órdenes
        /// 
        /// # Retorna
        /// Vector de tuplas (nombre_categoria, total_ventas)
        fn _contar_ventas_por_categoria(
            &self,
            categoria_productos: &Vec<(u32, String, Vec<u32>)>,
            publicaciones: &Vec<Publicacion>,
            ordenes: &Vec<Orden>,
        ) -> Vec<(String, u32)>;

        /// Calcula el promedio de calificaciones por categoría basándose en las órdenes recibidas.
        /// 
        /// # Parámetros
        /// * `categoria_productos` - Mapeo de categorías con sus productos
        /// * `publicaciones` - Referencia al vector de publicaciones
        /// * `ordenes` - Referencia al vector de órdenes
        /// 
        /// # Retorna
        /// Vector de tuplas (nombre_categoria, promedio_calificacion_string)
        fn _calcular_promedio_calificaciones_categoria(
            &self,
            categoria_productos: &Vec<(u32, String, Vec<u32>)>,
            publicaciones: &Vec<Publicacion>,
            ordenes: &Vec<Orden>,
        ) -> Vec<(String, String)>;

        /// Extrae todas las calificaciones de vendedor válidas de un conjunto de órdenes.
        /// 
        /// # Parámetros
        /// * `ordenes_categoria` - Referencia al vector de referencias de órdenes
        /// 
        /// # Retorna
        /// Vector de calificaciones (u8) que no son None
        fn _extraer_calificaciones_de_ordenes(&self, ordenes_categoria: &Vec<&Orden>) -> Vec<u8>;

        /// Calcula el promedio de un vector de calificaciones y lo retorna como string con un decimal.
        /// 
        /// # Parámetros
        /// * `calificaciones` - Vector de calificaciones numéricas
        /// 
        /// # Retorna
        /// String del promedio en formato "X.Y" o "0.0" si no hay calificaciones
        fn _calcular_promedio_de_calificaciones(&self, calificaciones: Vec<u8>) -> String;
    }

    /// Trait que define operaciones para generar reportes relacionados con usuarios.
    /// Incluye funcionalidades para contar órdenes por usuario y obtener los mejores usuarios por rol.
    pub trait ConsultasUsuarios {
        /// Cuenta la cantidad total de órdenes generadas por cada usuario.
        /// 
        /// # Parámetros
        /// * `usuarios` - Vector de usuarios del sistema
        /// * `ordenes` - Vector de órdenes para contar
        /// 
        /// # Retorna
        /// Vector de `OrdenesPorUsuario` con el conteo de órdenes por usuario
        fn _cantidad_de_ordenes_por_usuario(
            &self,
            usuarios: Vec<Usuario>,
            ordenes: Vec<Orden>,
        ) -> Vec<OrdenesPorUsuario>;

        /// Obtiene los cinco mejores usuarios de un rol específico ordenados por calificación.
        /// 
        /// # Parámetros
        /// * `usuarios` - Vector de usuarios del sistema
        /// * `target_role` - Rol específico para filtrar (Comprador o Vendedor)
        /// 
        /// # Retorna
        /// Result con vector de `RatingPorUsuario` o error si el rol no es válido
        fn _mejores_usuarios_por_rol(
            &self,
            usuarios: Vec<Usuario>,
            target_role: &Rol,
        ) -> Result<Vec<RatingPorUsuario>, ErroresReportes>;

        /// Lista los mejores cinco usuarios de un vector ya filtrado y ordenado.
        /// 
        /// # Parámetros
        /// * `usuario_filtrados` - Vector de usuarios ya filtrados por rol y ordenados
        /// * `target_role` - Rol para determinar qué calificación usar
        /// 
        /// # Retorna
        /// Vector de máximo 5 `RatingPorUsuario` con los mejores usuarios
        fn _listar_mejores_cinco_usuarios(
            &self,
            usuario_filtrados: Vec<Usuario>,
            target_role: &Rol,
        ) -> Vec<RatingPorUsuario>;

        /// Filtra usuarios por rol específico y los ordena por calificación descendente.
        /// 
        /// # Parámetros
        /// * `usuarios` - Vector de usuarios a filtrar
        /// * `target_role` - Rol específico para filtrar
        /// 
        /// # Retorna
        /// Vector de usuarios filtrados y ordenados por calificación (mayor a menor)
        fn _filtrar_usuarios_por_rol_desc(
            &self,
            usuarios: Vec<Usuario>,
            target_role: &Rol,
        ) -> Vec<Usuario>;
    }

    /// Contrato principal de reportes que proporciona análisis y estadísticas del sistema de marketplace.
    /// Actúa como una capa de análisis sobre el contrato principal del sistema.
    #[ink(storage)]
    pub struct Reportes {
        pub original: SistemaRef,
    }

    impl Reportes {
        /// Crea una nueva instancia del contrato de reportes.
        /// 
        /// # Parámetros
        /// * `address` - Dirección del contrato principal del sistema de marketplace
        /// 
        /// # Retorna
        /// Nueva instancia de Reportes conectada al sistema principal
        #[ink(constructor)]
        pub fn new(address: AccountId) -> Self {
            let original = SistemaRef::from_account_id(address);
            Self { original }
        }

        /// Retorna un reporte con la cantidad de órdenes generadas por cada usuario.
        /// 
        /// # Retorna
        /// Result con vector de `OrdenesPorUsuario` o error si no hay datos suficientes
        #[ink(message)]
        pub fn listar_cantidad_de_ordenes_por_usuario(
            &self,
        ) -> Result<Vec<OrdenesPorUsuario>, ErroresReportes> {
            Ok(self._cantidad_de_ordenes_por_usuario(
                self.original.listar_usuarios(), self.original.listar_ordenes(),
            ))
        }

        /// Retorna los cinco mejores usuarios de un rol específico ordenados por calificación.
        /// 
        /// # Parámetros
        /// * `target_role` - Rol a filtrar (Comprador o Vendedor, no Ambos)
        /// 
        /// # Retorna
        /// Result con vector de `RatingPorUsuario` o error si el rol no es válido
        #[ink(message)]
        pub fn listar_mejores_usuarios_por_rol(
            &self,
            target_role: Rol,
        ) -> Result<Vec<RatingPorUsuario>, ErroresReportes> {
            Ok(self._mejores_usuarios_por_rol(self.original.listar_usuarios(), &target_role)?)
        }

        /// Retorna un reporte de productos ordenados por cantidad de ventas (descendente).
        /// 
        /// # Retorna
        /// Result con vector de `VentasPorProducto` ordenado por ventas o error si no hay datos
        #[ink(message)]
        pub fn listar_productos_mas_vendidos(
            &self,
        ) -> Result<Vec<VentasPorProducto>, ErroresReportes> {
            Ok(self._productos_mas_vendidos(
                self.original.listar_productos(), self.original.listar_ordenes(), self.original.listar_publicaciones(),
            ))
        }

        /// Retorna estadísticas completas por categoría incluyendo ventas y calificaciones promedio.
        /// 
        /// # Retorna
        /// Result con vector de `EstadisticasPorCategoria` o error si no hay datos suficientes
        #[ink(message)]
        pub fn listar_estadisticas_por_categoria(
            &self,
        ) -> Result<Vec<EstadisticasPorCategoria>, ErroresReportes> {
            Ok(self._estadisticas_por_categoria(
                self.original.listar_categorias(), self.original.listar_productos(),
                self.original.listar_publicaciones(), self.original.listar_ordenes(),
            ))
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
        ) -> Result<Vec<RatingPorUsuario>, ErroresReportes> {
            if target_role == &Rol::Ambos {
                return Err(ErroresReportes::EleccionNoDisponible);
            }
            let usuarios_filtrados: Vec<Usuario> =
                self._filtrar_usuarios_por_rol_desc(usuarios, target_role);
            let reporte: Vec<RatingPorUsuario> =
                self._listar_mejores_cinco_usuarios(usuarios_filtrados, target_role);
            Ok(reporte)
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
    use ink::env::call::FromAccountId;
    use ink::primitives::AccountId;
    use market::prelude::*;

    // Helper function para crear AccountId de prueba
    fn crear_account_id(value: u8) -> AccountId {
        AccountId::from([value; 32])
    }

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
        rating.set_calificacion_comprador(calificacion_comprador);
        rating.set_calificacion_vendedor(calificacion_vendedor);

        let mut nuevo: Usuario = Usuario::new(id, nombre.to_string(), email.to_string());
        nuevo.set_roles(vec![rol]);
        nuevo.set_rating(rating);
        nuevo
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

    // Helper function para crear un producto de prueba
    fn crear_producto_test(id: u32, nombre: &str, categoria: u32) -> Producto {
        Producto::new(
            id,
            crear_account_id(99), // id_vendedor
            nombre.to_string(),
            "Descripción".to_string(),
            categoria,
            100, // stock
        )
    }

    // Helper function para crear una publicación de prueba
    fn crear_publicacion_test(id: u32, id_producto: u32, precio: u128, stock: u32) -> Publicacion {
        Publicacion::new(
            id,
            id_producto,
            crear_account_id(99), // id_user
            stock,
            precio, // precio_unitario (Balance)
        )
    }

    // Helper function para crear una categoría de prueba
    fn crear_categoria_test(id: u32, nombre: &str) -> Categoria {
        Categoria::new(id, nombre.to_string())
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
                (0, 0),     // Sin calificaciones como comprador
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
        assert_eq!(
            resultado.len(),
            5,
            "Debe retornar solo los 5 mejores usuarios"
        );

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
        assert_eq!(
            resultado.len(),
            5,
            "Debe retornar solo los 5 mejores usuarios"
        );

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
        let resultado = reportes._mejores_usuarios_por_rol(usuarios, &target_role).unwrap();

        // Assert
        assert_eq!(
            resultado.len(),
            5,
            "Debe retornar solo los 5 mejores compradores"
        );

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
    fn test_mejores_usuarios_por_rol_ambos_error() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let usuarios = vec![
            crear_usuario_test(
                crear_account_id(1),
                "Usuario1",
                "user1@test.com",
                Rol::Ambos,
                (450, 100),
                (300, 100),
            ),
            crear_usuario_test(
                crear_account_id(2),
                "Usuario2",
                "user2@test.com",
                Rol::Comprador,
                (320, 100),
                (0, 0),
            ),
        ];

        let target_role = Rol::Ambos;

        // Act
        let resultado = reportes._mejores_usuarios_por_rol(usuarios, &target_role);

        // Assert
        assert!(resultado.is_err(), "Debe retornar error para rol Ambos");
        assert_eq!(resultado.unwrap_err(), ErroresReportes::EleccionNoDisponible);
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
        assert_eq!(
            resultado.len(),
            5,
            "Debe retornar info para todos los usuarios"
        );

        // Verificar cantidad de órdenes por usuario
        assert_eq!(resultado[0].nombre_usuario, "Usuario1");
        assert_eq!(
            resultado[0].cantidad_ordenes, 1,
            "Usuario1 debe tener 1 orden"
        );

        assert_eq!(resultado[1].nombre_usuario, "Usuario2");
        assert_eq!(
            resultado[1].cantidad_ordenes, 2,
            "Usuario2 debe tener 2 órdenes"
        );

        assert_eq!(resultado[2].nombre_usuario, "Usuario3");
        assert_eq!(
            resultado[2].cantidad_ordenes, 3,
            "Usuario3 debe tener 3 órdenes"
        );

        assert_eq!(resultado[3].nombre_usuario, "Usuario4");
        assert_eq!(
            resultado[3].cantidad_ordenes, 1,
            "Usuario4 debe tener 1 orden"
        );

        assert_eq!(resultado[4].nombre_usuario, "Usuario5");
        assert_eq!(
            resultado[4].cantidad_ordenes, 2,
            "Usuario5 debe tener 2 órdenes"
        );
    }

    #[test]
    fn test_productos_mas_vendidos() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let productos = vec![
            crear_producto_test(0, "Producto A", 0),
            crear_producto_test(1, "Producto B", 1),
            crear_producto_test(2, "Producto C", 0),
        ];

        let publicaciones = vec![
            crear_publicacion_test(0, 0, 100, 50), // Publicación del Producto A
            crear_publicacion_test(1, 1, 200, 30), // Publicación del Producto B
            crear_publicacion_test(2, 2, 150, 20), // Publicación del Producto C
            crear_publicacion_test(3, 0, 110, 25), // Otra publicación del Producto A
        ];

        let mut ordenes = vec![
            crear_orden_test(1, crear_account_id(1), crear_account_id(2), 0, 5), // Pub 0 (Producto A) - 5 ventas
            crear_orden_test(2, crear_account_id(2), crear_account_id(3), 1, 3), // Pub 1 (Producto B) - 3 ventas
            crear_orden_test(3, crear_account_id(3), crear_account_id(4), 2, 2), // Pub 2 (Producto C) - 2 ventas
            crear_orden_test(4, crear_account_id(4), crear_account_id(5), 3, 3), // Pub 3 (Producto A) - 3 ventas más
            crear_orden_test(5, crear_account_id(5), crear_account_id(1), 1, 1), // Pub 1 (Producto B) - 1 venta más
        ];

        // Configurar órdenes como recibidas para que cuenten como ventas
        for orden in &mut ordenes {
            orden.set_status(EstadoOrden::Recibida);
        }

        // Act
        let resultado = reportes._productos_mas_vendidos(productos, ordenes, publicaciones);

        // Assert
        assert_eq!(resultado.len(), 3, "Debe retornar todos los productos");

        // Verificar orden descendente: Producto A (8 ventas), Producto B (4 ventas), Producto C (2 ventas)
        // Producto A: 5 + 3 = 8 ventas, Producto B: 3 + 1 = 4 ventas, Producto C: 2 ventas
        assert_eq!(resultado[0].nombre_producto, "Producto A");
        assert_eq!(resultado[0].cantidad_ventas, 8);

        assert_eq!(resultado[1].nombre_producto, "Producto B");
        assert_eq!(resultado[1].cantidad_ventas, 4);

        assert_eq!(resultado[2].nombre_producto, "Producto C");
        assert_eq!(resultado[2].cantidad_ventas, 2);
    }

    #[test]
    fn test_mapear_nombres_productos() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let productos = vec![
            crear_producto_test(10, "Laptop", 0),
            crear_producto_test(20, "Mouse", 1),
            crear_producto_test(30, "Teclado", 0),
        ];

        let publicaciones = vec![
            crear_publicacion_test(100, 10, 1000, 5), // Pub 100 -> Producto 10 (Laptop)
            crear_publicacion_test(101, 20, 50, 10),  // Pub 101 -> Producto 20 (Mouse)
            crear_publicacion_test(102, 10, 1100, 3), // Pub 102 -> Producto 10 (Laptop)
            crear_publicacion_test(103, 40, 200, 8),  // Pub 103 -> Producto 40 (inexistente)
        ];

        // Act
        let resultado = reportes._mapear_nombres_productos(publicaciones, &productos);

        // Assert
        assert_eq!(
            resultado.len(),
            3,
            "Debe mapear solo publicaciones con productos existentes"
        );

        // Verificar mapeos correctos
        assert!(resultado.contains(&(100, 10, "Laptop".to_string())));
        assert!(resultado.contains(&(101, 20, "Mouse".to_string())));
        assert!(resultado.contains(&(102, 10, "Laptop".to_string())));

        // Verificar que no incluye productos inexistentes
        assert!(!resultado.iter().any(|(_, prod_id, _)| *prod_id == 40));
    }

    #[test]
    fn test_contar_ventas_por_producto() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let productos = vec![
            crear_producto_test(1, "ProductoA", 0),
            crear_producto_test(2, "ProductoB", 1),
            crear_producto_test(3, "ProductoC", 0),
        ];

        let publis_x_prod = vec![
            (10, 1, "ProductoA".to_string()), // Pub 10 -> Producto 1
            (11, 2, "ProductoB".to_string()), // Pub 11 -> Producto 2
            (12, 1, "ProductoA".to_string()), // Pub 12 -> Producto 1
            (13, 3, "ProductoC".to_string()), // Pub 13 -> Producto 3
        ];

        let mut ordenes = vec![
            crear_orden_test(1, crear_account_id(1), crear_account_id(2), 10, 5), // ProductoA - 5 ventas
            crear_orden_test(2, crear_account_id(2), crear_account_id(3), 11, 3), // ProductoB - 3 ventas
            crear_orden_test(3, crear_account_id(3), crear_account_id(4), 12, 2), // ProductoA - 2 ventas más
            crear_orden_test(4, crear_account_id(4), crear_account_id(5), 13, 0), // ProductoC - 0 ventas (cantidad 0)
            crear_orden_test(5, crear_account_id(5), crear_account_id(1), 10, 1), // ProductoA - 1 venta más
        ];

        // Configurar diferentes estados de órdenes
        ordenes[0].set_status(EstadoOrden::Recibida); // Cuenta como venta
        ordenes[1].set_status(EstadoOrden::Recibida); // Cuenta como venta
        ordenes[2].set_status(EstadoOrden::Pendiente); // NO cuenta como venta
        ordenes[3].set_status(EstadoOrden::Recibida); // Cuenta pero cantidad = 0
        ordenes[4].set_status(EstadoOrden::Recibida); // Cuenta como venta

        // Act
        let resultado = reportes._contar_ventas_por_producto(&productos, ordenes, publis_x_prod);

        // Assert
        assert_eq!(resultado.len(), 3, "Debe retornar todos los productos");

        // Verificar conteos: ProductoA = 6 (5+1, no cuenta la pendiente), ProductoB = 3, ProductoC = 0
        let producto_a = resultado
            .iter()
            .find(|p| p.nombre_producto == "ProductoA")
            .unwrap();
        assert_eq!(producto_a.cantidad_ventas, 6);

        let producto_b = resultado
            .iter()
            .find(|p| p.nombre_producto == "ProductoB")
            .unwrap();
        assert_eq!(producto_b.cantidad_ventas, 3);

        let producto_c = resultado
            .iter()
            .find(|p| p.nombre_producto == "ProductoC")
            .unwrap();
        assert_eq!(producto_c.cantidad_ventas, 0);
    }

    #[test]
    fn test_estadisticas_por_categoria() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let categorias = vec![
            crear_categoria_test(0, "Electrónicos"),
            crear_categoria_test(1, "Ropa"),
            crear_categoria_test(2, "Libros"),
        ];

        let productos = vec![
            crear_producto_test(10, "Laptop", 0), // Electrónicos
            crear_producto_test(20, "Camisa", 1), // Ropa
            crear_producto_test(30, "Novela", 2), // Libros
            crear_producto_test(40, "Mouse", 0),  // Electrónicos
        ];

        let publicaciones = vec![
            crear_publicacion_test(100, 10, 1000, 5), // Laptop
            crear_publicacion_test(101, 20, 50, 10),  // Camisa
            crear_publicacion_test(102, 30, 25, 8),   // Novela
            crear_publicacion_test(103, 40, 30, 15),  // Mouse
        ];

        let mut ordenes = vec![
            crear_orden_test(1, crear_account_id(1), crear_account_id(2), 100, 2), // Laptop - 2 ventas
            crear_orden_test(2, crear_account_id(2), crear_account_id(3), 101, 3), // Camisa - 3 ventas
            crear_orden_test(3, crear_account_id(3), crear_account_id(4), 102, 1), // Novela - 1 venta
            crear_orden_test(4, crear_account_id(4), crear_account_id(5), 103, 4), // Mouse - 4 ventas
            crear_orden_test(5, crear_account_id(5), crear_account_id(1), 100, 1), // Laptop - 1 venta más
        ];

        // Configurar órdenes como recibidas y agregar calificaciones
        for (i, orden) in ordenes.iter_mut().enumerate() {
            orden.set_status(EstadoOrden::Recibida);
            // Agregar calificaciones diferentes para testear promedios
            match i {
                0 => orden.set_calificacion_vendedor(Some(4)), // Laptop: 4
                1 => orden.set_calificacion_vendedor(Some(5)), // Camisa: 5
                2 => orden.set_calificacion_vendedor(Some(3)), // Novela: 3
                3 => orden.set_calificacion_vendedor(Some(4)), // Mouse: 4
                4 => orden.set_calificacion_vendedor(Some(2)), // Laptop: 2
                _ => {}
            }
        }

        // Act
        let resultado =
            reportes._estadisticas_por_categoria(categorias, productos, publicaciones, ordenes);

        // Assert
        assert_eq!(resultado.len(), 3, "Debe retornar todas las categorías");

        // Verificar estadísticas por categoría
        let electronicos = resultado
            .iter()
            .find(|c| c.nombre_categoria == "Electrónicos")
            .unwrap();
        assert_eq!(electronicos.cantidad_ventas, 7); // Laptop: 3, Mouse: 4
        assert_eq!(electronicos.promedio_calificacion, "3.3"); // (4+2+4)/3 = 3.33 ≈ 3.3

        let ropa = resultado
            .iter()
            .find(|c| c.nombre_categoria == "Ropa")
            .unwrap();
        assert_eq!(ropa.cantidad_ventas, 3); // Camisa: 3
        assert_eq!(ropa.promedio_calificacion, "5.0"); // 5/1 = 5.0

        let libros = resultado
            .iter()
            .find(|c| c.nombre_categoria == "Libros")
            .unwrap();
        assert_eq!(libros.cantidad_ventas, 1); // Novela: 1
        assert_eq!(libros.promedio_calificacion, "3.0"); // 3/1 = 3.0
    }

    #[test]
    fn test_mapear_productos_por_categoria() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let categorias = vec![
            crear_categoria_test(1, "Tecnología"),
            crear_categoria_test(2, "Hogar"),
            crear_categoria_test(3, "Deportes"),
        ];

        let productos = vec![
            crear_producto_test(101, "Smartphone", 1), // Tecnología
            crear_producto_test(102, "Mesa", 2),       // Hogar
            crear_producto_test(103, "Balón", 3),      // Deportes
            crear_producto_test(104, "Tablet", 1),     // Tecnología
            crear_producto_test(105, "Silla", 2),      // Hogar
            crear_producto_test(106, "Producto Sin Categoría", 99), // Categoría inexistente
        ];

        // Act
        let resultado = reportes._mapear_productos_por_categoria(&categorias, &productos);

        // Assert
        assert_eq!(
            resultado.len(),
            3,
            "Debe retornar un mapeo para cada categoría"
        );

        // Verificar mapeo de Tecnología
        let tecnologia = resultado
            .iter()
            .find(|(_, nombre, _)| nombre == "Tecnología")
            .unwrap();
        assert_eq!(tecnologia.2, vec![101, 104]); // Smartphone y Tablet

        // Verificar mapeo de Hogar
        let hogar = resultado
            .iter()
            .find(|(_, nombre, _)| nombre == "Hogar")
            .unwrap();
        assert_eq!(hogar.2, vec![102, 105]); // Mesa y Silla

        // Verificar mapeo de Deportes
        let deportes = resultado
            .iter()
            .find(|(_, nombre, _)| nombre == "Deportes")
            .unwrap();
        assert_eq!(deportes.2, vec![103]); // Solo Balón

        // Verificar que productos sin categoría válida no aparecen
        for (_, _, productos_ids) in &resultado {
            assert!(!productos_ids.contains(&106));
        }
    }

    #[test]
    fn test_contar_ventas_por_categoria() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let categoria_productos = vec![
            (1, "Gaming".to_string(), vec![10, 11]), // Gaming: productos 10, 11
            (2, "Oficina".to_string(), vec![20]),    // Oficina: producto 20
            (3, "Vacía".to_string(), vec![]),        // Sin productos
        ];

        let publicaciones = vec![
            crear_publicacion_test(100, 10, 500, 10), // Producto 10
            crear_publicacion_test(101, 11, 300, 5),  // Producto 11
            crear_publicacion_test(102, 20, 200, 8),  // Producto 20
            crear_publicacion_test(103, 10, 550, 3),  // Otra pub del producto 10
        ];

        let mut ordenes = vec![
            crear_orden_test(1, crear_account_id(1), crear_account_id(2), 100, 3), // Producto 10 - 3 ventas
            crear_orden_test(2, crear_account_id(2), crear_account_id(3), 101, 2), // Producto 11 - 2 ventas
            crear_orden_test(3, crear_account_id(3), crear_account_id(4), 102, 4), // Producto 20 - 4 ventas
            crear_orden_test(4, crear_account_id(4), crear_account_id(5), 103, 1), // Producto 10 - 1 venta más
            crear_orden_test(5, crear_account_id(5), crear_account_id(1), 101, 2), // Producto 11 - 2 ventas más
            crear_orden_test(6, crear_account_id(1), crear_account_id(3), 102, 0), // Producto 20 - 0 ventas
        ];

        // Configurar diferentes estados
        ordenes[0].set_status(EstadoOrden::Recibida); // Cuenta
        ordenes[1].set_status(EstadoOrden::Recibida); // Cuenta
        ordenes[2].set_status(EstadoOrden::Pendiente); // NO cuenta
        ordenes[3].set_status(EstadoOrden::Recibida); // Cuenta
        ordenes[4].set_status(EstadoOrden::Recibida); // Cuenta
        ordenes[5].set_status(EstadoOrden::Recibida); // Cuenta pero cantidad = 0

        // Act
        let resultado =
            reportes._contar_ventas_por_categoria(&categoria_productos, &publicaciones, &ordenes);

        // Assert
        assert_eq!(
            resultado.len(),
            3,
            "Debe retornar conteo para todas las categorías"
        );

        // Gaming: Producto 10 (3+1=4) + Producto 11 (2+2=4) = 8 ventas
        let gaming = resultado
            .iter()
            .find(|(nombre, _)| nombre == "Gaming")
            .unwrap();
        assert_eq!(gaming.1, 8);

        // Oficina: Producto 20 (0, porque la orden está pendiente) = 0 ventas
        let oficina = resultado
            .iter()
            .find(|(nombre, _)| nombre == "Oficina")
            .unwrap();
        assert_eq!(oficina.1, 0);

        // Vacía: Sin productos = 0 ventas
        let vacia = resultado
            .iter()
            .find(|(nombre, _)| nombre == "Vacía")
            .unwrap();
        assert_eq!(vacia.1, 0);
    }

    #[test]
    fn test_calcular_promedio_calificaciones_categoria() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let categoria_productos = vec![
            (1, "Premium".to_string(), vec![50, 51]), // Premium: productos 50, 51
            (2, "Básico".to_string(), vec![60]),      // Básico: producto 60
            (3, "Sin Ventas".to_string(), vec![70]),  // Sin ventas
        ];

        let publicaciones = vec![
            crear_publicacion_test(200, 50, 1000, 2), // Producto 50
            crear_publicacion_test(201, 51, 800, 3),  // Producto 51
            crear_publicacion_test(202, 60, 400, 5),  // Producto 60
            crear_publicacion_test(203, 70, 100, 1),  // Producto 70
        ];

        let mut ordenes = vec![
            crear_orden_test(1, crear_account_id(1), crear_account_id(2), 200, 1), // Producto 50
            crear_orden_test(2, crear_account_id(2), crear_account_id(3), 200, 1), // Producto 50
            crear_orden_test(3, crear_account_id(3), crear_account_id(4), 201, 1), // Producto 51
            crear_orden_test(4, crear_account_id(4), crear_account_id(5), 202, 1), // Producto 60
            crear_orden_test(5, crear_account_id(5), crear_account_id(1), 202, 1), // Producto 60
            crear_orden_test(6, crear_account_id(1), crear_account_id(4), 203, 1), // Producto 70 - Pendiente
        ];

        // Configurar estados y calificaciones
        for (i, orden) in ordenes.iter_mut().enumerate() {
            match i {
                0..=4 => orden.set_status(EstadoOrden::Recibida),
                5 => orden.set_status(EstadoOrden::Pendiente), // No cuenta
                _ => {}
            }

            // Asignar calificaciones
            match i {
                0 => orden.set_calificacion_vendedor(Some(5)), // Producto 50: 5
                1 => orden.set_calificacion_vendedor(Some(3)), // Producto 50: 3
                2 => orden.set_calificacion_vendedor(Some(4)), // Producto 51: 4
                3 => orden.set_calificacion_vendedor(Some(2)), // Producto 60: 2
                4 => orden.set_calificacion_vendedor(Some(4)), // Producto 60: 4
                5 => orden.set_calificacion_vendedor(Some(1)), // Producto 70: no cuenta
                _ => {}
            }
        }

        // Act
        let resultado = reportes._calcular_promedio_calificaciones_categoria(
            &categoria_productos,
            &publicaciones,
            &ordenes,
        );

        // Assert
        assert_eq!(
            resultado.len(),
            3,
            "Debe retornar promedios para todas las categorías"
        );

        // Premium: (5+3+4)/3 = 12/3 = 4.0
        let premium = resultado
            .iter()
            .find(|(nombre, _)| nombre == "Premium")
            .unwrap();
        assert_eq!(premium.1, "4.0");

        // Básico: (2+4)/2 = 6/2 = 3.0
        let basico = resultado
            .iter()
            .find(|(nombre, _)| nombre == "Básico")
            .unwrap();
        assert_eq!(basico.1, "3.0");

        // Sin Ventas: sin órdenes recibidas = 0.0
        let sin_ventas = resultado
            .iter()
            .find(|(nombre, _)| nombre == "Sin Ventas")
            .unwrap();
        assert_eq!(sin_ventas.1, "0.0");
    }

    #[test]
    fn test_extraer_calificaciones_de_ordenes() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        let mut ordenes = vec![
            crear_orden_test(1, crear_account_id(1), crear_account_id(2), 100, 1),
            crear_orden_test(2, crear_account_id(2), crear_account_id(3), 101, 1),
            crear_orden_test(3, crear_account_id(3), crear_account_id(4), 102, 1),
            crear_orden_test(4, crear_account_id(4), crear_account_id(5), 103, 1),
        ];

        // Configurar calificaciones
        ordenes[0].set_calificacion_vendedor(Some(5));
        ordenes[1].set_calificacion_vendedor(None); // Sin calificación
        ordenes[2].set_calificacion_vendedor(Some(3));
        ordenes[3].set_calificacion_vendedor(Some(4));

        let ordenes_ref: Vec<&Orden> = ordenes.iter().collect();

        // Act
        let resultado = reportes._extraer_calificaciones_de_ordenes(&ordenes_ref);

        // Assert
        assert_eq!(
            resultado.len(),
            3,
            "Solo debe extraer calificaciones que existen"
        );
        assert!(resultado.contains(&5));
        assert!(resultado.contains(&3));
        assert!(resultado.contains(&4));
        assert!(!resultado.contains(&0)); // No debe incluir valores por defecto
    }

    #[test]
    fn test_calcular_promedio_de_calificaciones() {
        // Arrange
        let reportes = Reportes {
            original: SistemaRef::from_account_id(crear_account_id(1)),
        };

        // Test con lista vacía
        let vacia: Vec<u8> = vec![];
        assert_eq!(reportes._calcular_promedio_de_calificaciones(vacia), "0.0");

        // Test con una calificación
        let una = vec![4];
        assert_eq!(reportes._calcular_promedio_de_calificaciones(una), "4.0");

        // Test con múltiples calificaciones - promedio exacto
        let exacto = vec![2, 4]; // (2+4)/2 = 3.0
        assert_eq!(reportes._calcular_promedio_de_calificaciones(exacto), "3.0");

        // Test con múltiples calificaciones - promedio con decimal
        let decimal = vec![3, 3, 4]; // (3+3+4)/3 = 10/3 = 3.33... ≈ 3.3
        assert_eq!(
            reportes._calcular_promedio_de_calificaciones(decimal),
            "3.3"
        );

        // Test con calificaciones que dan decimal diferente
        let otro_decimal = vec![1, 2, 3, 4, 5]; // (1+2+3+4+5)/5 = 15/5 = 3.0
        assert_eq!(
            reportes._calcular_promedio_de_calificaciones(otro_decimal),
            "3.0"
        );

        // Test con promedio que requiere redondeo
        let redondeo = vec![1, 5]; // (1+5)/2 = 6/2 = 3.0
        assert_eq!(
            reportes._calcular_promedio_de_calificaciones(redondeo),
            "3.0"
        );
    }
}
