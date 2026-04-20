#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk inventory barang
#[contracttype]
#[derive(Clone, Debug)]
pub struct Item {
    id: u64,
    name: String,
    stock: u32,
    description: String,
}

// Storage key untuk data inventory
const ITEM_DATA: Symbol = symbol_short!("ITEM_DATA");

#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {
    // ambil semua item
    pub fn get_items(env: Env) -> Vec<Item> {
        env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env))
    }

    // tambah barang baru
    pub fn create_item(env: Env, name: String, stock: u32, description: String) -> String {
        let mut items: Vec<Item> = env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env));

        let item = Item {
            id: env.prng().gen::<u64>(),
            name,
            stock,
            description,
        };

        items.push_back(item);
        env.storage().instance().set(&ITEM_DATA, &items);

        String::from_str(&env, "Item berhasil ditambahkan")
    }

    // hapus barang
    pub fn delete_item(env: Env, id: u64) -> String {
        let mut items: Vec<Item> = env.storage().instance().get(&ITEM_DATA).unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            if items.get(i).unwrap().id == id {
                items.remove(i);
                env.storage().instance().set(&ITEM_DATA, &items);
                return String::from_str(&env, "Item berhasil dihapus");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }
}

mod test;