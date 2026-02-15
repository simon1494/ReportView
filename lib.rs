#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[allow(dead_code)]

#[ink::contract]
mod reportes {
    use ink::env::call::FromAccountId;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use RustMarket::prelude::{Usuario, *};


    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Debug, PartialEq)]
    pub enum ErroresReportes {
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
        pub cantidad_ordenes: String,
    }


    pub trait ConsultasProductos {
        fn _get_productos_mas_vendidos(&self, limit_to: u32) -> Vec<OrdenesPorUsuario>;
    }


    pub trait ConsultasCategorias {
        fn _get_estadisticas_por_categoria(&self, categoria: &str) -> Vec<EstadisticasPorCategoria>;
    }

    pub trait ConsultasUsuarios {
        fn reporte_ordenes_por_usuario(&self) -> Result<Vec<OrdenesPorUsuario>, ErroresReportes>;

        fn reporte_mejores_usuarios_por_rol(&self, target_role: &Rol) -> Result<Vec<RatingPorUsuario>, ErroresReportes>;

        fn _listar_ordenes_por_usuario(&self, usuarios: Vec<Usuario>, ordenes: Vec<Orden>) -> Vec<OrdenesPorUsuario>;        

        fn _listar_mejores_cinco_usuarios(&self, usuario_filtrados: Vec<Usuario>, target_role: &Rol) -> Vec<RatingPorUsuario>;

        fn _filtrar_usuarios_por_rol_desc(&self, usuarios: Vec<Usuario>, target_role: &Rol) -> Vec<Usuario>;

        fn _calcular_promedio(&self, usuario: &Usuario, rol: &Rol) -> u32;

        fn _prom_to_str(&self, promedio_escalado: u32) -> String;
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

        /// Devuelve una lista de todos los usuarios registrados en el contrato original.
        #[ink(message)]
        pub fn get_cantidad_de_ordenes_por_usuario(&self) -> Result<Vec<OrdenesPorUsuario>, ErroresReportes> {
            self.reporte_ordenes_por_usuario()
        }

        #[ink(message)]
        pub fn get_mejores_usuarios_por_rol(&self, target_role: Rol) -> Result<Vec<RatingPorUsuario>, ErroresReportes> {
            self._eleccion_disponible(&target_role)?;
            self.reporte_mejores_usuarios_por_rol(&target_role)
        }

        fn _get_ordenes(&self) ->  Result<Vec<Orden>, ErroresReportes> {
            let ordenes = self.original.listar_ordenes();
            if ordenes.is_empty() {
                return Err(ErroresReportes::NoHayOrdenesCreadas)
            }
            Ok(ordenes)
        }

        fn _get_usuarios(&self) -> Result<Vec<Usuario>, ErroresReportes> {
            let usuarios = self.original.listar_usuarios();
            if usuarios.is_empty() {
                return Err(ErroresReportes::NoHayUsuariosCreados)
            }
            Ok(usuarios)
        }

        fn _get_publicaciones(&self) -> Result<Vec<Publicacion>, ErroresReportes> {
            let publicaciones = self.original.listar_publicaciones();
            if publicaciones.is_empty() {
                return Err(ErroresReportes::NoHayPublicacionesCreadas)
            }
            Ok(publicaciones)
        }

        fn _eleccion_disponible(&self, eleccion_rol: &Rol) -> Result<(), ErroresReportes> {
            if eleccion_rol == &Rol::Ambos{
                return Err(ErroresReportes::EleccionNoDisponible)
            }
            Ok(())
        }
    }

    impl ConsultasUsuarios for Reportes {

        fn reporte_ordenes_por_usuario(&self) -> Result<Vec<OrdenesPorUsuario>, ErroresReportes> {
            let usuarios: Vec<Usuario> = self._get_usuarios()?;
            let ordenes: Vec<Orden> = self._get_ordenes()?;
            let reporte: Vec<OrdenesPorUsuario> = self._listar_ordenes_por_usuario(usuarios,ordenes);

            Ok(reporte)
        }


        fn _listar_ordenes_por_usuario(&self, usuarios: Vec<Usuario>, ordenes: Vec<Orden>) -> Vec<OrdenesPorUsuario> {
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

        fn reporte_mejores_usuarios_por_rol(&self, target_role: &Rol) -> Result<Vec<RatingPorUsuario>, ErroresReportes> {
            let usuarios: Vec<Usuario> = self._get_usuarios()?;
            let usuarios_filtrados: Vec<Usuario> = self._filtrar_usuarios_por_rol_desc(usuarios, target_role);
            let reporte: Vec<RatingPorUsuario> = self._listar_mejores_cinco_usuarios(usuarios_filtrados, target_role);

            Ok(reporte)
        }

        fn _listar_mejores_cinco_usuarios(&self, usuarios_filtrados: Vec<Usuario>, target_role: &Rol) -> Vec<RatingPorUsuario> {
            let mut top_5: Vec<RatingPorUsuario> = Vec::new();
            for (count, u) in usuarios_filtrados.into_iter().enumerate() {
                if count >= 5 {
                    break;
                }
                let promedio_str: String = self._prom_to_str(self._calcular_promedio(&u, target_role));
                
                let item: RatingPorUsuario = RatingPorUsuario {
                    nombre_usuario: u.get_name(),
                    cantidad_ordenes: promedio_str,
                };
                top_5.push(item);
            }

            top_5
        }

        fn _filtrar_usuarios_por_rol_desc(&self, usuarios: Vec<Usuario>, target_role: &Rol) -> Vec<Usuario> {
            let mut usuarios_filtrados: Vec<Usuario> = Vec::new();
            
            for usuario in usuarios {
                if usuario.has_role(target_role.clone()) {
                    usuarios_filtrados.push(usuario);
                }
            }

            usuarios_filtrados.sort_by(|a, b| {
                let prom_a: u32 = self._calcular_promedio(a, target_role);
                let prom_b: u32 = self._calcular_promedio(b, target_role);
                prom_b.cmp(&prom_a)
            });

            usuarios_filtrados
        }

        fn _calcular_promedio(&self, usuario: &Usuario, rol: &Rol) -> u32 {
            let (puntos, cantidad) = match rol {
                Rol::Comprador => usuario.rating.calificacion_comprador,
                Rol::Vendedor => usuario.rating.calificacion_vendedor,
                _ => (0, 0),
            };
            if cantidad == 0 {
                0
            } else {
                // Escalamos por 100 para tener 2 decimales de precisión, evitando overflow
                puntos.saturating_mul(100).checked_div(cantidad).unwrap_or(0)
            }
        }

        fn _prom_to_str(&self, promedio_escalado: u32) -> String {
            let parte_entera = promedio_escalado / 100;
            let parte_decimal = promedio_escalado % 100;
            format!("{}.{:02}", parte_entera, parte_decimal)
        }
    }
}
