#![cfg_attr(not(feature = "std"), no_std, no_main)]

use marketplacedescentralizado::SistemaRef;

#[ink::contract]
mod reportes {
    use super::*;
    use ink::prelude::vec::Vec;
    use marketplacedescentralizado::Usuario;
    use ink::env::call::FromAccountId;

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
}
