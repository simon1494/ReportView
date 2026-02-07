#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod reportes {
    use super::*;
    use ink::env::call::FromAccountId;
    use ink::prelude::vec::Vec;
    use marketplacedescentralizado::prelude::*;
    
    //TODO: Los tipos de retorno son genericos. Hay que crear 
    //      un struct que contenga producto_id, nombre del producto
    //      y cantidad total de ventas (entregadas).
    pub trait ConsultasProductos{
        fn get_productos_mas_vendidos(&self, limit_to: u32) -> Vec<Producto>;
    }


    //TODO: Los tipos de retorno son genericos. Hay que crear 
    //      un struct que contenga categoria_id, nombre categoria
    //      y cantidad total de ventas (entregadas) de la categoria
    //      y calificacion promedio de la categoria. Se retorna un Vec.   
    pub trait ConsultasCategorias{
        fn get_estadisticas_por_categoria(&self, categoria: &str) -> Vec<String>;
    }

    //TODO: Los tipos de retorno son genericos. Hay que crear 
    //      un struct que contenga usuario_id, nombre_usuario,
    //      y cantidad total ordenes (todas). Se retorna un Vec
    //      para (get_cantidad_de_ordenes_por_usuario).   
    ///
    //      Despues, hay que crear un struct que contenga account_id, 
    //      nombre del usuario y su reputacion. Se retorna el Vec
    //      ordenado DESC por reputacion de usuario (ver como ordenar, si
    ///     por str o por numerico)
    pub trait ConsultasUsuarios{

        
        fn get_cantidad_de_ordenes_por_usuario(&self) -> Vec<ReporteOrdenesUsuario>;

        fn get_mejores_usuarios_por_rol(&self, target_role: Rol) -> Vec<Usuario>;  //separar por rol compra vender
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
        pub fn listar_usuarios(&self) -> Vec<Usuario> {
            self.original.listar_usuarios()
        }
    }
    impl ConsultasUsuarios for Reportes {
        #[ink(message)]
        fn get_cantidad_de_ordenes_por_usuario(&self) -> Vec<ReporteOrdenesUsuario> {
            let usuarios = self.original.listar_usuarios();
            let ordenes = self.original.listar_ordenes();    
            let mut reporte = Vec::new();

            for usuario in usuarios {
                let mut contador = 0;
                for orden in &ordenes {
                    if orden.get_id_comprador() == usuario.get_id() {
                        contador += 1;
                    }}
                let item = ReporteOrdenesUsuario {
                    nombre_usuario: usuario.get_name(),
                    cantidad_ordenes: contador,
                };
                reporte.push(item);
            }
            reporte
        }
    }
}
