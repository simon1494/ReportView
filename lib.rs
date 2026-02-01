#![cfg_attr(not(feature = "std"), no_std, no_main)]

use marketplacedescentralizado::SistemaRef;

#[ink::contract]
mod reportes {
    use super::*;
    use ink::prelude::vec::Vec;
    use marketplacedescentralizado::Publicacion;

    #[ink(storage)]
    pub struct Reportes {
        original: SistemaRef,
    }

    impl Reportes {
        #[ink(constructor)]
        pub fn new(other_contract_code_hash: Hash) -> Self {
            let original = SistemaRef::new(true)
                .code_hash(other_contract_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self { original }
        }

        #[ink(message)]
        pub fn avergaston(&mut self) -> Vec<Publicacion> {
            self.original.listar_publicaciones()
        }
    }
}
