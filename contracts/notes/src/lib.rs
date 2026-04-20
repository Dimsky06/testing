#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk menyimpan item inventory rumah sakit
#[contracttype]
#[derive(Clone, Debug)]
pub struct InventoryItem {
    id: u64,
    name: String,      // nama barang
    category: String,  // kategori barang (obat, alat medis, APD, dll)
    stock: u32,        // jumlah stok
    unit: String,      // satuan (box, pcs, botol, dll)
}

// Storage key untuk data inventory
const INVENTORY_DATA: Symbol = symbol_short!("INV_DATA");

#[contract]
pub struct HospitalInventoryContract;

#[contractimpl]
impl HospitalInventoryContract {
    // Ambil semua data inventory
    pub fn get_items(env: Env) -> Vec<InventoryItem> {
        env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah item baru ke inventory
    pub fn add_item(
        env: Env,
        name: String,
        category: String,
        stock: u32,
        unit: String,
    ) -> String {
        let mut items: Vec<InventoryItem> = env
            .storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        let item = InventoryItem {
            id: env.prng().gen::<u64>(),
            name,
            category,
            stock,
            unit,
        };

        items.push_back(item);
        env.storage().instance().set(&INVENTORY_DATA, &items);

        String::from_str(&env, "Item inventory berhasil ditambahkan")
    }

    // Hapus item berdasarkan id
    pub fn remove_item(env: Env, id: u64) -> String {
        let mut items: Vec<InventoryItem> = env
            .storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            if items.get(i).unwrap().id == id {
                items.remove(i);
                env.storage().instance().set(&INVENTORY_DATA, &items);
                return String::from_str(&env, "Item berhasil dihapus");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }

    // Update stok barang berdasarkan id
    pub fn update_stock(env: Env, id: u64, new_stock: u32) -> String {
        let mut items: Vec<InventoryItem> = env
            .storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            let mut item = items.get(i).unwrap();

            if item.id == id {
                item.stock = new_stock;
                items.set(i, item);

                env.storage().instance().set(&INVENTORY_DATA, &items);
                return String::from_str(&env, "Stok berhasil diperbarui");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }
}

mod test;