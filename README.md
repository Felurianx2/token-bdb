# 🪙 Token BDB 🎧- Token Fungible en Stellar/Soroban

[![Stellar](https://img.shields.io/badge/Stellar-Soroban-blue?logo=stellar)](https://stellar.org)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/Tests-32%2F32%20passing-brightgreen)]()
[![License](https://img.shields.io/badge/License-MIT-yellow)]()

**Builder Token (BDB)** es un token fungible completo implementado en Stellar/Soroban, siguiendo el estándar CAP-46 (Stellar Token Interface). Este proyecto fue desarrollado como parte del programa **Buen Día Builders - Código Futura**.

🔗 **Contract ID (Testnet):** `CAWSMAP3EKUYB375RIGOHYOET5ISHT36MHZZTUUFOITGFVIXT4D2YF7T`

---

## 📋 Índice

- [Características](#-características)
- [Requisitos Previos](#-requisitos-previos)
- [Instalación](#-instalación)
- [Estructura del Proyecto](#-estructura-del-proyecto)
- [Build & Deploy](#-build--deploy)
- [Tests](#-tests)
- [Uso del Token](#-uso-del-token)
- [Arquitectura](#-arquitectura)
- [Contribuyendo](#-contribuyendo)
---

## ✨ Características

### Funcionalidades Implementadas

- ✅ **Initialize**: Inicialización del token con metadatos (nombre, símbolo, decimales)
- ✅ **Mint**: Creación de nuevos tokens (solo admin)
- ✅ **Burn**: Destrucción de tokens existentes
- ✅ **Transfer**: Transferencia directa entre cuentas
- ✅ **Approve**: Sistema de allowances (permisos de gasto)
- ✅ **Transfer From**: Transferencias delegadas vía allowances
- ✅ **Balance**: Consulta de saldo de cualquier cuenta
- ✅ **Total Supply**: Consulta del supply total en circulación
- ✅ **Metadata Queries**: Nombre, símbolo, decimales y admin

### Seguridad

- 🛡️ **9 Errores Customizados**: Manejo robusto de casos edge
- 🔒 **Authorization**: Control de acceso vía Soroban Auth
- ✅ **32 Tests**: Cobertura completa incluyendo edge cases
- 🚨 **Validaciones**: Prevención de overflow, validación de amounts, verificación de recipients

### Estándares Seguidos

- 📚 **CAP-46**: Stellar Token Interface Standard
- 🏗️ **Soroban SDK 23.0.3**: Última versión estable
- 🦀 **Rust Best Practices**: Código limpio, modularidad, documentación

---

## 🔧 Requisitos Previos

Antes de comenzar, necesitarás tener instalado:

- [Rust](https://www.rust-lang.org/tools/install) 1.75+
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools) 21.0.0+
- [Git](https://git-scm.com/downloads) (para clonar el repositorio)

### Verificar Instalación

```bash
# Verificar Rust
rustc --version
cargo --version

# Verificar Stellar CLI
stellar --version

# Instalar target WASM (necesario para Soroban)
rustup target add wasm32v1-none
```

---

## 📥 Instalación

### Clonar el Repositorio

```bash
git clone https://github.com/TU_USUARIO/token_bdb.git
cd token_bdb
```

### Instalar Dependencias

```bash
# Las dependencias de Rust se instalan automáticamente en el build
cargo build
```

---

## 📁 Estructura del Proyecto
```
token_bdb/
├── Cargo.toml           # Configuración del proyecto y dependencias
├── .gitignore           # Archivos ignorados por Git
├── README.md            # Este archivo
└── src/
    ├── lib.rs           # Contrato principal + trait TokenTrait
    ├── errors.rs        # Enum de errores customizados (9 tipos)
    ├── storage.rs       # DataKeys y estructuras de datos
    └── test.rs          # 32 tests unitarios

# Carpetas generadas (ignoradas por Git):
├── target/              # Binarios compilados (generado por cargo build)
└── test_snapshots/      # Snapshots de tests (generado por cargo test)
```

---

## Build & Deploy

### Build Local

```bash
# Build del contrato para WASM
stellar contract build

# Archivo generado:
# target/wasm32v1-none/release/token_bdb.wasm
```

### Deploy en Testnet

#### 1. Crear Identidad

```bash
# Generar clave
stellar keys generate alice --network testnet

# Ver dirección pública
stellar keys address alice

# Agregar fondos de prueba
stellar keys fund alice --network testnet
```

#### 2. Deploy del Contrato

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/token_bdb.wasm \
  --source alice \
  --network testnet
```

¡Guarda el `CONTRACT_ID` que retorna!

#### 3. Inicializar el Token

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account alice \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --name "Builder Token" \
  --symbol "BDB" \
  --decimals 7
```

---

## 🧪 Tests

### Ejecutar Todos los Tests

```bash
cargo test
```

**Resultado esperado:**
```
test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Categorías de Tests

#### 1. Tests Básicos (12 tests)
- `test_initialize` - Inicialización correcta del token
- `test_initialize_twice_fails` - Prevención de doble inicialización
- `test_invalid_decimals` - Validación de decimales (máximo 18)
- `test_mint_and_balance` - Minteo y consulta de balance
- `test_mint_zero_fails` - Validación de amount > 0
- `test_transfer` - Transferencia entre usuarios
- `test_transfer_insufficient_balance` - Validación de balance suficiente
- `test_transfer_to_self` - Prevención de self-transfer
- `test_approve_and_transfer_from` - Sistema de allowances
- `test_transfer_from_insufficient_allowance` - Validación de allowance
- `test_burn` - Quema de tokens
- `test_operations_without_init` - Todas las operaciones requieren init

#### 2. Tests de Edge Cases (12 tests)
- `test_mint_large_amount` - Cantidades muy grandes
- `test_mint_overflow_prevention` - Prevención de overflow
- `test_balance_zero_after_full_transfer` - Balance = 0 después de transferir todo
- `test_burn_entire_balance` - Burn de todo el balance
- `test_allowance_exhausted` - Consumir toda la allowance
- `test_revoke_allowance` - Revocar allowance con approve(0)
- `test_multiple_mints_accumulate` - Múltiples mints acumulan
- `test_empty_name_fails` - Validación de nombre no vacío
- `test_empty_symbol_fails` - Validación de símbolo no vacío
- `test_transfer_exact_balance` - Transfer exacto del balance
- `test_use_exact_allowance` - Usar exactamente toda la allowance
- Y más...

#### 3. Tests de Múltiples Usuarios (3 tests)
- `test_multiple_users_balances` - Varios usuarios con balances
- `test_chain_transfers` - Transferencias en cadena
- `test_multiple_approvals` - Múltiples approvals del mismo owner

#### 4. Tests de Consultas/Getters (5 tests)
- `test_balance_of_empty_account` - Balance de cuenta vacía = 0
- `test_allowance_without_approval` - Allowance sin approve = 0
- `test_total_supply_increments` - Total supply se incrementa
- `test_total_supply_decrements_on_burn` - Total supply decrementa con burn
- `test_decimals_configuration` - Configuración de decimales
- `test_admin_address_stored` - Admin address se guarda correctamente

### Ejecutar Tests Específicos

```bash
# Test individual
cargo test test_initialize -- --nocapture

# Tests de una categoría
cargo test test_mint -- --nocapture
cargo test test_transfer -- --nocapture
```

---

## 💰 Uso del Token

### Ejemplos de Operaciones

#### Mintear Tokens

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account alice \
  --network testnet \
  --send yes \
  -- mint \
  --to <RECIPIENT_ADDRESS> \
  --amount 10000000
```

**Nota:** Con `decimals = 7`, `10000000` stroops = 1 token

#### Consultar Balance

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account alice \
  --network testnet \
  -- balance \
  --account <ACCOUNT_ADDRESS>
```

#### Transferir Tokens

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account alice \
  --network testnet \
  --send yes \
  -- transfer \
  --from <FROM_ADDRESS> \
  --to <TO_ADDRESS> \
  --amount 5000000
```

#### Aprobar Allowance

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account alice \
  --network testnet \
  --send yes \
  -- approve \
  --from <OWNER_ADDRESS> \
  --spender <SPENDER_ADDRESS> \
  --amount 2000000
```

#### Transfer From (Usando Allowance)

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account bob \
  --network testnet \
  --send yes \
  -- transfer_from \
  --spender <SPENDER_ADDRESS> \
  --from <OWNER_ADDRESS> \
  --to <RECIPIENT_ADDRESS> \
  --amount 1000000
```

#### Quemar Tokens

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account alice \
  --network testnet \
  --send yes \
  -- burn \
  --from <ACCOUNT_ADDRESS> \
  --amount 3000000
```

### Conversión de Valores

| Tokens | Stroops (decimals=7) |
|--------|---------------------|
| 1 | 10,000,000 |
| 10 | 100,000,000 |
| 100 | 1,000,000,000 |
| 1,000 | 10,000,000,000 |
| 1,000,000 | 10,000,000,000,000 |

---

## 🏛️ Arquitectura

### Storage Layout

```rust
pub enum DataKey {
    Balance(Address),              // Persistent Storage
    Allowance(Address, Address),   // Persistent Storage
    TotalSupply,                   // Instance Storage
    Admin,                         // Instance Storage
    TokenName,                     // Instance Storage
    TokenSymbol,                   // Instance Storage
    Decimals,                      // Instance Storage
    Initialized,                   // Instance Storage
}
```

### Eventos Emitidos (usando #[contractevent])

El contrato utiliza el macro `#[contractevent]` para definir eventos tipados:

```rust
// InitEvent
#[contractevent]
pub struct InitEvent {
    #[topic]
    pub admin: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

// MintEvent
#[contractevent]
pub struct MintEvent {
    #[topic]
    pub to: Address,
    pub amount: i128,
    pub new_balance: i128,
    pub new_total: i128,
}

// TransferEvent
#[contractevent]
pub struct TransferEvent {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    pub amount: i128,
    pub new_from_balance: i128,
    pub new_to_balance: i128,
}

// BurnEvent
#[contractevent]
pub struct BurnEvent {
    #[topic]
    pub from: Address,
    pub amount: i128,
    pub new_balance: i128,
    pub new_total: i128,
}

// ApproveEvent
#[contractevent]
pub struct ApproveEvent {
    #[topic]
    pub from: Address,
    #[topic]
    pub spender: Address,
    pub old_allowance: i128,
    pub new_allowance: i128,
}
```

**Emisión de eventos:**
```rust
MintEvent {
    to: to.clone(),
    amount,
    new_balance,
    new_total,
}.publish(&env);
```

### Manejo de Errores

```rust
pub enum TokenError {
    AlreadyInitialized = 1,    // Contrato ya inicializado
    InvalidAmount = 2,          // Amount <= 0
    InsufficientBalance = 3,    // Balance insuficiente
    InsufficientAllowance = 4,  // Allowance insuficiente
    NotInitialized = 5,         // Contrato no inicializado
    InvalidDecimals = 6,        // Decimals > 18
    OverflowError = 7,          // Overflow aritmético
    InvalidRecipient = 8,       // Transfer a sí mismo
    InvalidMetadata = 9,        // Nombre/símbolo inválido
}
```

---

## 🤝 Contribuyendo

¡Las contribuciones son bienvenidas!

Si quieres contribuir, por favor, sigue estos pasos:
1. Fork el proyecto
2. Crea una branch para tu feature (`git checkout -b feature/NuevaFuncionalidad`)
3. Commit tus cambios (`git commit -m 'Agrega nueva funcionalidad'`)
4. Push a la branch (`git push origin feature/NuevaFuncionalidad`)
5. Abre un Pull Request

### Ejecutar Tests Antes del PR

```bash
# Ejecutar todos los tests
cargo test

# Verificar formateo
cargo fmt --check

# Verificar linting
cargo clippy
```

---

## 📚 Recursos

### Documentación Oficial
- [Soroban Documentation](https://developers.stellar.org/docs/soroban)
- [CAP-46 Token Standard](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0046.md)
- [Rust Programming Language](https://doc.rust-lang.org/book/)

### Herramientas
- [Stellar Expert](https://stellar.expert/explorer/testnet) - Explorer de testnet
- [Stellar Laboratory](https://laboratory.stellar.org/) - Probar transacciones
- [Freighter Wallet](https://www.freighter.app/) - Wallet para navegador

---

## 📊 Estadísticas del Proyecto

| Métrica | Valor |
|---------|-------|
| Líneas de Código | ~1,600 |
| Tests | 32 |
| Cobertura | ~95% |
| Funciones Públicas | 13 |
| Errores Customizados | 9 |
| Eventos | 5 |

---

## 👥 Autores

- **Isamar Suarez 🦈** - *Desarrollo* - [@Felurianx2](https://github.com/Felurianx2)

Proyecto desarrollado durante el programa **Buen Día Builders - Código Futura**.

---

## 🙏 Agradecimientos

- [Buen Día Builders](https://x.com/buendiabuilders) - Por la iniciativa Código Futura
- [Stellar Development Foundation](https://stellar.org/) - Por la tecnología Soroban
- [Blockchain Acceleration Foundation](https://www.blockchainacceleration.org/) - Por el soporte al curso y a las desarrolladoras
- [Lumen Loop](https://x.com/lumenloop) - Por el soporte técnico

---

## 🔗 Links Importantes

- **Contract ID (Testnet):** `CAWSMAP3EKUYB375RIGOHYOET5ISHT36MHZZTUUFOITGFVIXT4D2YF7T`
- **Explorer:** [Ver en Stellar Expert](https://stellar.expert/explorer/testnet/contract/CAWSMAP3EKUYB375RIGOHYOET5ISHT36MHZZTUUFOITGFVIXT4D2YF7T)
- **Repositorio:** [GitHub](https://github.com/Felurian/token-bdb)

---

<div align="center">

**Isamar Suarez 🦈**

[![Twitter](https://img.shields.io/twitter/follow/isasuarezx2?style=social)](https://twitter.com/isasuarezx2)
[![GitHub](https://img.shields.io/github/followers/Felurianx2?style=social)](https://github.com/Felurianx2)

</div>