// 1. const é avaliado em tempo de compilação e inserido diretamente onde é usado.
const NUM_CONST: i32 = 18; // Não possui endereço fixo na memória, é "inlined".

// 2. static é avaliado em tempo de compilação e possui um local fixo na memória.
// Ou seja, tem lifetime 'static.
static NUM: i32 = 18; // Pode ser referenciado com &'static, vive durante toda a execução.

// 3. const não pode ser referenciado com lifetime 'static, pois não tem endereço fixo.
const NAME_CONST: &str = "Rust"; // É copiado onde for usado, não tem local fixo.

// 4. static pode ser referenciado com lifetime 'static, ideal para ponteiros fixos.
static NAME_STATIC: &str = "Rust"; // &'static str — permanece no mesmo local da memória.

// 5. static pode ser mutável, mas requer bloco unsafe para acesso.
static mut COUNTER: i32 = 0; // unsafe { COUNTER += 1; } — cuidado com concorrência.

// 6. const não pode ser mutável, sempre será imutável.
const MAX_USERS: u32 = 100; // Valor fixo e imutável durante toda a execução.

// Resumo:
// const  -> valor fixo, sem endereço garantido, usado como substituição no código.
// static -> valor fixo com endereço garantido, vive durante todo o programa.
