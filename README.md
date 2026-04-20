# Stellar Inventory DApp

**Stellar Inventory DApp** - Blockchain-Based Inventory Management System

## Project Description

Stellar Inventory DApp adalah smart contract berbasis blockchain yang dibangun di atas jaringan Stellar menggunakan Soroban SDK. Aplikasi ini menyediakan sistem manajemen inventory (barang) yang transparan, aman, dan terdesentralisasi.

Semua data barang disimpan langsung di blockchain, sehingga tidak bergantung pada database terpusat. Setiap item memiliki identitas unik dan dapat dikelola melalui fungsi smart contract.

Sistem ini memungkinkan pengguna untuk menambahkan, melihat, dan menghapus data barang dengan mudah dan aman.

---

## Project Vision

Tujuan dari project ini:

- **Desentralisasi Data**: Menghilangkan ketergantungan pada server terpusat
- **Kepemilikan Data**: Data inventory sepenuhnya dimiliki oleh user
- **Transparansi**: Semua aktivitas dapat diverifikasi di blockchain
- **Keamanan Tinggi**: Data tidak bisa dimanipulasi sembarangan
- **Efisiensi Sistem**: Mengurangi risiko kehilangan data

---

## Key Features

### 1. **Tambah Barang (Create Item)**

- Tambah item baru ke inventory
- Input: nama barang, jumlah stok, dan deskripsi
- ID otomatis di-generate
- Data tersimpan permanen di blockchain

---

### 2. **Ambil Semua Data (Get Items)**

- Mengambil seluruh data inventory
- Format terstruktur, mudah dipakai frontend
- Real-time sesuai kondisi blockchain

---

### 3. **Hapus Barang (Delete Item)**

- Hapus item berdasarkan ID
- Data langsung terhapus dari storage contract
- Update data secara instan

---

### 4. **Keamanan & Transparansi**

- Semua aktivitas tercatat di blockchain
- Tidak bisa dimodifikasi tanpa melalui contract
- Integritas data terjamin

---

### 5. **Integrasi Stellar Network**

- Biaya transaksi rendah
- Performa cepat
- Menggunakan Soroban Smart Contract SDK

---

## Contract Functions

- `create_item(name, stock, description)`
- `get_items()`
- `delete_item(id)`

---

## Data Structure

```rust
pub struct Item {
    id: u64,
    name: String,
    stock: u32,
    description: String,
}


ID Smartcontract: CASIE7WYXENNX2RV7Z6AWQ4T5TLNR2433EVVV4DS5G4A4LONWJMLSBPG




