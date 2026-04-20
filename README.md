# Hospital Inventory Contract

**Hospital Inventory Contract** is a smart contract built with **Soroban SDK** on the **Stellar blockchain** to manage hospital inventory data.

## Project Description

This project replaces the default template project and implements a new smart contract for hospital inventory management. The contract allows users to store and manage item data on-chain.

Users can:

* add new inventory items
* view all stored items
* remove items by ID
* update stock by ID

All data is stored in the smart contract storage, making it persistent and transparent on the blockchain.

## Main Functions

* `add_item()` - Add a new inventory item
* `get_items()` - Retrieve all inventory items
* `remove_item()` - Delete an item by ID
* `update_stock()` - Update item stock by ID

## Technologies Used

* Rust
* Soroban SDK
* Stellar Blockchain
* Soroban Studio

## Build

To build the contract, run:

```bash
stellar contract build
```

## Exported Functions

* `add_item`
* `get_items`
* `remove_item`
* `update_stock`

## Conclusion

This project is a custom implementation that replaces the default Soroban template with a hospital inventory management smart contract.
