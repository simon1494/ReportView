#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[allow(dead_code)]
#[ink::contract]
mod reportes {
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
        ) -> Result<Vec<VentasPorProducto>, ErroresReportes>;

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
        fn _get_estadisticas_por_categoria(&self, categoria: &str)
            -> Vec<EstadisticasPorCategoria>;
    }

    pub trait ConsultasUsuarios {
        fn _cantidad_de_ordenes_por_usuario(
            &self,
            usuarios: Vec<Usuario>,
            ordenes: Vec<Orden>,
        ) -> Result<Vec<OrdenesPorUsuario>, ErroresReportes>;

        fn _mejores_usuarios_por_rol(
            &self,
            usuarios: Vec<Usuario>,
            target_role: &Rol,
        ) -> Result<Vec<RatingPorUsuario>, ErroresReportes>;

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
        original: SistemaRef,
    }

    impl Reportes {
        #[ink(constructor)]
        pub fn new(address: AccountId) -> Self {
            let original = SistemaRef::from_account_id(address);
            Self { original }
        }

        /// Retorna una listado con los nombres de todos los usuarios y sus órdenes generadas.
        #[ink(message)]
        pub fn get_cantidad_de_ordenes_por_usuario(
            &self,
        ) -> Result<Vec<OrdenesPorUsuario>, ErroresReportes> {
            let usuarios: Vec<Usuario> = self._get_usuarios()?;
            let ordenes: Vec<Orden> = self._get_ordenes()?;
            self._cantidad_de_ordenes_por_usuario(usuarios, ordenes)
        }

        #[ink(message)]
        /// Retorna una lista descendente de los cinco usuarios mejor calificados dentro de un rol.
        pub fn get_mejores_usuarios_por_rol(
            &self,
            target_role: Rol,
        ) -> Result<Vec<RatingPorUsuario>, ErroresReportes> {
            self._eleccion_disponible(&target_role)?;
            let usuarios: Vec<Usuario> = self._get_usuarios()?;
            self._mejores_usuarios_por_rol(usuarios, &target_role)
        }

        #[ink(message)]
        /// Retorna una lista descendente de los productos más vendidos.
        pub fn get_productos_mas_vendidos(
            &self,
        ) -> Result<Vec<VentasPorProducto>, ErroresReportes> {
            let productos: Vec<Producto> = self._get_productos()?;
            let ordenes: Vec<Orden> = self._get_ordenes()?;
            let publicaciones: Vec<Publicacion> = self._get_publicaciones()?;
            self._productos_mas_vendidos(productos, ordenes, publicaciones)
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
        ) -> Result<Vec<VentasPorProducto>, ErroresReportes> {
            let productos_x_publicaciones: Vec<(u32, u32, String)> =
                self._mapear_nombres_productos(publicaciones, &productos);

            let mut ventas_por_producto: Vec<VentasPorProducto> =
                self._contar_ventas_por_producto(&productos, ordenes, productos_x_publicaciones);
            
            ventas_por_producto.sort_by(|a: &VentasPorProducto, b: &VentasPorProducto| b.cantidad_ventas.cmp(&a.cantidad_ventas));

            Ok(ventas_por_producto)
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
        ) -> Result<Vec<OrdenesPorUsuario>, ErroresReportes> {
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

            Ok(reporte)
        }

        fn _mejores_usuarios_por_rol(
            &self,
            usuarios: Vec<Usuario>,
            target_role: &Rol,
        ) -> Result<Vec<RatingPorUsuario>, ErroresReportes> {
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
}
