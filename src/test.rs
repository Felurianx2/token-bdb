#![cfg(test)]

use super::*; 
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

/// Test básico de inicialización del token
/// 
/// Verifica que:
/// - El contrato se inicializa correctamente con metadatos válidos
/// - Los metadatos se pueden leer después de la inicialización
/// - El supply inicial es 0
#[test]
fn test_initialize() {
    // Arrange: Setup del entorno de testing
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Builder Token");
    let symbol = String::from_str(&env, "BDB");
    
    // Act: Inicializar el token
    client.initialize(&admin, &name, &symbol, &7);  // Retirar .unwrap()
    
    // Assert: Verificar que los metadatos se guardaron correctamente
    assert_eq!(client.name(), name);
    assert_eq!(client.symbol(), symbol);
    assert_eq!(client.decimals(), 7);
    assert_eq!(client.total_supply(), 0);
}

/// Test de protección contra doble inicialización
/// 
/// Verifica que el contrato no puede ser inicializado dos veces,
/// lo cual es crítico para la seguridad del token.
#[test]
fn test_initialize_twice_fails() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Token");
    let symbol = String::from_str(&env, "TOK");
    
    // Primera inicialización debe funcionar
    client.initialize(&admin, &name, &symbol, &7);  // Retirar .unwrap()
    
    // Segunda debe fallar con AlreadyInitialized
    let result = client.try_initialize(&admin, &name, &symbol, &7);  // try_
    assert_eq!(result, Err(Ok(TokenError::AlreadyInitialized)));
}

/// Test de validación de decimales
/// 
/// Los decimales deben estar en el rango 0-18.
/// 18 es el máximo para compatibilidad con Ethereum,
/// 7 es el estándar en Stellar (alineado con XLM).
#[test]
fn test_invalid_decimals() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    // Decimales > 18 debe fallar
    let result = client.try_initialize(  // try_
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &19  // ❌ Inválido: excede MAX_DECIMALS (18)
    );
    assert_eq!(result, Err(Ok(TokenError::InvalidDecimals)));
}

/// Test básico de mint y consulta de balance
/// 
/// Verifica el flujo completo:
/// 1. Initialize del token
/// 2. Mint de tokens a un usuario
/// 3. Consulta de balance
/// 4. Verificación de total supply
#[test]
fn test_mint_and_balance() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    // Initialize el token
    client.initialize(
        &admin, 
        &String::from_str(&env, "Builder Token"),
        &String::from_str(&env, "BDB"),
        &7
    );  // Retirar .unwrap()
    
    // Mock auth: En tests, simulamos autorizaciones sin firmas reales
    env.mock_all_auths();
    
    // Mintear 1000 tokens
    client.mint(&user, &1000);  // Retirar .unwrap()
    
    // Verificar estado actualizado
    assert_eq!(client.balance(&user), 1000);
    assert_eq!(client.total_supply(), 1000);
}

/// Test: mint con amount = 0 debe fallar
/// 
/// Mintear 0 tokens no tiene sentido y podría
/// causar eventos innecesarios o confusión.
#[test]
fn test_mint_zero_fails() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );  // // Retirar .unwrap()
    
    env.mock_all_auths();
    
    // Mintear 0 debe fallar con InvalidAmount
    let result = client.try_mint(&user, &0);  // Retirar .unwrap()
    assert_eq!(result, Err(Ok(TokenError::InvalidAmount)));
}

/// Test básico de transferencia entre dos usuarios
/// 
/// Verifica el flujo completo de transfer:
/// 1. Alice tiene 1000 tokens
/// 2. Alice transfiere 250 tokens a Bob
/// 3. Ambos balances se actualizan correctamente
#[test]
fn test_transfer() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    // Setup: Initialize y dar tokens a Alice
    client.initialize(
        &admin,
        &String::from_str(&env, "Builder Token"),
        &String::from_str(&env, "BDB"),
        &7
    );  // Retirar .unwrap()
    
    env.mock_all_auths();
    client.mint(&alice, &1000);  // Retirar .unwrap()
    
    // Act: Alice transfiere a Bob
    client.transfer(&alice, &bob, &250);  // Retirar .unwrap()
    
    // Assert: Verificar ambos balances
    assert_eq!(client.balance(&alice), 750);  // 1000 - 250
    assert_eq!(client.balance(&bob), 250);
}

/// Test: transfer con balance insuficiente debe fallar
/// 
/// No puedes transferir más tokens de los que tienes.
/// Este es uno de los errores más comunes en tokens.
#[test]
fn test_transfer_insufficient_balance() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );  // Retirar .unwrap()
    
    env.mock_all_auths();
    client.mint(&alice, &100);  // Retirar .unwrap()
    
    // Intentar transferir más de lo que tiene debe fallar
    let result = client.try_transfer(&alice, &bob, &200);  // try_
    assert_eq!(result, Err(Ok(TokenError::InsufficientBalance)));
}

/// Test: transfer a sí mismo debe fallar
/// 
/// Decisión de diseño: prohibimos transferencias a sí mismo por:
/// - Ahorro de gas (operación inútil)
/// - Evitar eventos confusos
/// - Prevenir errores del usuario
#[test]
fn test_transfer_to_self() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );  // Retirar .unwrap()
    
    env.mock_all_auths();
    client.mint(&alice, &1000);  // Retirar .unwrap()
    
    // Transfer a sí mismo debe fallar con InvalidRecipient
    let result = client.try_transfer(&alice, &alice, &100);  // try
    assert_eq!(result, Err(Ok(TokenError::InvalidRecipient)));
    assert_eq!(client.balance(&alice), 1000); // Balance no debe cambiar
}

/// Test del flujo completo de approve + transfer_from
/// 
/// Este es el patrón "allowance" usado en DeFi:
/// 1. Alice aprueba a Bob para gastar hasta 300 tokens
/// 2. Bob usa transfer_from para mover 200 tokens de Alice a Charlie
/// 3. El allowance se reduce automáticamente a 100
#[test]
fn test_approve_and_transfer_from() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);
    
    // Setup
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );  // Retirar .unwrap()
    
    env.mock_all_auths();
    client.mint(&alice, &1000);  // Retirar .unwrap()
    
    // Alice aprueba a Bob para gastar hasta 300 tokens
    client.approve(&alice, &bob, &300);  // Retirar .unwrap()
    assert_eq!(client.allowance(&alice, &bob), 300);
    
    // Bob transfiere 200 tokens de Alice a Charlie
    client.transfer_from(&bob, &alice, &charlie, &200);  // Retirar .unwrap()
    
    // Verificar estado final
    assert_eq!(client.balance(&alice), 800);          // 1000 - 200
    assert_eq!(client.balance(&charlie), 200);        // 0 + 200
    assert_eq!(client.allowance(&alice, &bob), 100);  // 300 - 200
}

/// Test: transfer_from con allowance insuficiente debe fallar
/// 
/// Bob solo puede gastar hasta el límite aprobado por Alice.
#[test]
fn test_transfer_from_insufficient_allowance() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );  // Retirar .unwrap()
    
    env.mock_all_auths();
    client.mint(&alice, &1000);  // Retirar .unwrap()
    client.approve(&alice, &bob, &100);  // // Retirar .unwrap()
    
    // Bob intenta transferir más de lo aprobado
    let result = client.try_transfer_from(&bob, &alice, &charlie, &200);  // // try
    assert_eq!(result, Err(Ok(TokenError::InsufficientAllowance)));
}

/// Test básico de burn (quemar tokens)
/// 
/// Burn reduce tanto el balance del usuario como el supply total.
/// Es usado para reducir supply (deflación), fees, etc.
#[test]
fn test_burn() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );  // // Retirar .unwrap()
    
    env.mock_all_auths();
    client.mint(&alice, &1000);  // // Retirar .unwrap()
    
    // Alice quema 300 de sus tokens
    client.burn(&alice, &300);  // // Retirar .unwrap()
    
    // Verificar que tanto balance como supply se redujeron
    assert_eq!(client.balance(&alice), 700);    // 1000 - 300
    assert_eq!(client.total_supply(), 700);     // 1000 - 300
}

/// Todas las operaciones deben fallar si no se inicializó
/// Verifica que el flag de inicialización se verifica en
/// TODAS las funciones que modifican estado.
#[test]
fn test_operations_without_init() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Todas las operaciones deben fallar con NotInitialized
    assert_eq!(
        client.try_mint(&alice, &100),  // // try
        Err(Ok(TokenError::NotInitialized))
    );
    
    assert_eq!(
        client.try_transfer(&alice, &bob, &50),  // // try
        Err(Ok(TokenError::NotInitialized))
    );
    
    assert_eq!(
        client.try_burn(&alice, &10),  // // try
        Err(Ok(TokenError::NotInitialized))
    );
}

// ============================================================================
// TESTS DE EDGE CASES (Casos Extremos)
// ============================================================================

/// Mintear cantidades muy grandes (pero seguras)
#[test]
fn test_mint_large_amount() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    
    // Mintear cantidad muy grande (pero segura)
    let large_amount = 1_000_000_000_000i128; // 1 trillón
    client.mint(&user, &large_amount);
    
    assert_eq!(client.balance(&user), large_amount);
    assert_eq!(client.total_supply(), large_amount);
}

/// Prevenir overflow al mintear más allá del límite i128
#[test]
fn test_mint_overflow_prevention() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    
    // Mintear casi el máximo
    let almost_max = i128::MAX - 1000;
    client.mint(&user, &almost_max);
    
    // Intentar mintear más debería fallar con OverflowError
    let result = client.try_mint(&user, &2000);
    assert_eq!(result, Err(Ok(TokenError::OverflowError)));
}

/// Balance = 0 después de transferir todo
#[test]
fn test_balance_zero_after_full_transfer() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    client.mint(&alice, &1000);
    
    // Alice transfiere TODO su balance
    client.transfer(&alice, &bob, &1000);
    
    // Balance de Alice debe ser 0
    assert_eq!(client.balance(&alice), 0);
    assert_eq!(client.balance(&bob), 1000);
}

/// Burn de todo el balance deja balance en 0
#[test]
fn test_burn_entire_balance() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    client.mint(&alice, &1000);
    
    // Quemar TODO el balance
    client.burn(&alice, &1000);
    
    // Balance debe ser 0 y supply también
    assert_eq!(client.balance(&alice), 0);
    assert_eq!(client.total_supply(), 0);
}

/// Allowance = 0 después de transfer_from que consume todo
#[test]
fn test_allowance_exhausted() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    client.mint(&alice, &1000);
    
    // Alice aprueba 500 a Bob
    client.approve(&alice, &bob, &500);
    
    // Bob usa TODA la allowance
    client.transfer_from(&bob, &alice, &charlie, &500);
    
    // Allowance debe ser 0
    assert_eq!(client.allowance(&alice, &bob), 0);
    
    // Bob no puede transferir más
    let result = client.try_transfer_from(&bob, &alice, &charlie, &1);
    assert_eq!(result, Err(Ok(TokenError::InsufficientAllowance)));
}

/// Revocar allowance estableciendo a 0
#[test]
fn test_revoke_allowance() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    
    // Alice aprueba a Bob
    client.approve(&alice, &bob, &1000);
    assert_eq!(client.allowance(&alice, &bob), 1000);
    
    // Alice revoca la aprobación
    client.approve(&alice, &bob, &0);
    assert_eq!(client.allowance(&alice, &bob), 0);
}

/// Múltiples mints acumulan correctamente
#[test]
fn test_multiple_mints_accumulate() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    
    // Múltiples mints
    client.mint(&user, &100);
    client.mint(&user, &200);
    client.mint(&user, &300);
    
    // Balance debe ser la suma
    assert_eq!(client.balance(&user), 600);
    assert_eq!(client.total_supply(), 600);
}

/// Nombre vacío debe fallar
#[test]
fn test_empty_name_fails() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let empty_name = String::from_str(&env, "");
    let symbol = String::from_str(&env, "TOK");
    
    // Debe fallar con InvalidMetadata
    let result = client.try_initialize(&admin, &empty_name, &symbol, &7);
    assert_eq!(result, Err(Ok(TokenError::InvalidMetadata)));
}

/// Symbol vacío debe fallar
#[test]
fn test_empty_symbol_fails() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Token");
    let empty_symbol = String::from_str(&env, "");
    
    // Debe fallar con InvalidMetadata
    let result = client.try_initialize(&admin, &name, &empty_symbol, &7);
    assert_eq!(result, Err(Ok(TokenError::InvalidMetadata)));
}

// ============================================================================
// TESTS DE MÚLTIPLOS USUÁRIOS
// ============================================================================

/// Múltiples usuarios pueden tener balances simultáneamente
#[test]
fn test_multiple_users_balances() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    
    // Mintear a múltiples usuarios
    client.mint(&user1, &100);
    client.mint(&user2, &200);
    client.mint(&user3, &300);
    
    // Verificar balances individuales
    assert_eq!(client.balance(&user1), 100);
    assert_eq!(client.balance(&user2), 200);
    assert_eq!(client.balance(&user3), 300);
    assert_eq!(client.total_supply(), 600);
}

/// Transferencias en cadena entre múltiples usuarios
#[test]
fn test_chain_transfers() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    client.mint(&alice, &1000);
    
    // Alice -> Bob -> Charlie
    client.transfer(&alice, &bob, &500);
    client.transfer(&bob, &charlie, &250);
    
    assert_eq!(client.balance(&alice), 500);
    assert_eq!(client.balance(&bob), 250);
    assert_eq!(client.balance(&charlie), 250);
}

/// Múltiples approvals del mismo owner a diferentes spenders
#[test]
fn test_multiple_approvals() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    
    // Alice aprueba a Bob y Charlie
    client.approve(&alice, &bob, &100);
    client.approve(&alice, &charlie, &200);
    
    // Verificar allowances independientes
    assert_eq!(client.allowance(&alice, &bob), 100);
    assert_eq!(client.allowance(&alice, &charlie), 200);
}

// ============================================================================
// TESTS DE CONSULTAS (Getters)
// ============================================================================

/// Consultar balance de cuenta sin tokens retorna 0
#[test]
fn test_balance_of_empty_account() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let empty_account = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    // Balance de cuenta sin tokens debe ser 0
    assert_eq!(client.balance(&empty_account), 0);
}

/// Consultar allowance sin approve previo retorna 0
#[test]
fn test_allowance_without_approval() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    // Allowance sin approve previo debe ser 0
    assert_eq!(client.allowance(&alice, &bob), 0);
}

/// Total supply se incrementa correctamente con múltiples mints
#[test]
fn test_total_supply_increments() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    
    // Supply inicial es 0
    assert_eq!(client.total_supply(), 0);
    
    client.mint(&user1, &500);
    assert_eq!(client.total_supply(), 500);
    
    client.mint(&user2, &300);
    assert_eq!(client.total_supply(), 800);
}

/// Total supply se decrementa correctamente con burn
#[test]
fn test_total_supply_decrements_on_burn() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    client.mint(&alice, &1000);
    assert_eq!(client.total_supply(), 1000);
    
    client.burn(&alice, &300);
    assert_eq!(client.total_supply(), 700);
    
    client.burn(&alice, &200);
    assert_eq!(client.total_supply(), 500);
}

/// Decimales se configuran correctamente
#[test]
fn test_decimals_configuration() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    // Probar diferentes decimales válidos
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &18  // Máximo permitido
    );
    
    assert_eq!(client.decimals(), 18);
}

/// Admin address se guarda correctamente
#[test]
fn test_admin_address_stored() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    // Admin debe coincidir con el configurado
    assert_eq!(client.admin(), admin);
}

// ============================================================================
// TESTS DE VALORES LÍMITE
// ============================================================================

/// Transferir exactamente el balance (edge case)
#[test]
fn test_transfer_exact_balance() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    client.mint(&alice, &999);
    
    // Transferir exactamente el balance
    client.transfer(&alice, &bob, &999);
    
    assert_eq!(client.balance(&alice), 0);
    assert_eq!(client.balance(&bob), 999);
}

/// Test: Usar exactamente toda la allowance (edge case)
#[test]
fn test_use_exact_allowance() {
    let env = Env::default();
    let contract_id = env.register(TokenBDB, ());
    let client = TokenBDBClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);
    
    client.initialize(
        &admin,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TOK"),
        &7
    );
    
    env.mock_all_auths();
    client.mint(&alice, &1000);
    client.approve(&alice, &bob, &456);
    
    // Usar exactamente toda la allowance
    client.transfer_from(&bob, &alice, &charlie, &456);
    
    assert_eq!(client.allowance(&alice, &bob), 0);
    assert_eq!(client.balance(&charlie), 456);
}