# Contrato de votacion simple - Luis Fernandez Garcia

Contrato inteligente de Stellar (Soroban) para votar una pregunta con opciones: **una direccion, un voto**.

## El problema (en simple)
Un grupo pequeno (un club, un curso, una comunidad de fans) quiere decidir algo, por ejemplo *"que recital organizamos?"*. Con una encuesta normal, el organizador puede cambiar los resultados, una persona puede votar varias veces o se puede votar despues de cerrar.

## La solucion
Un contrato que guarda en la blockchain la pregunta, las opciones y la fecha limite, y que:
- Solo deja votar **una vez** por direccion.
- Exige que el votante **firme** su voto (`require_auth`).
- **Rechaza** opciones que no existen y votos fuera de plazo.
- Permite que **cualquiera** consulte los resultados.

## Por que elegi este problema
Es pequeno y claro: cubre autenticacion, almacenamiento, tiempo del ledger y errores, que son los conceptos base de Soroban. No es la idea de la hackathon: es solo practica.

## Funciones
| Funcion | Que hace |
|---|---|
| `init(question, options, deadline)` | Crea la votacion (una sola vez, minimo 2 opciones). `deadline` en segundos (timestamp del ledger). |
| `vote(voter, option)` | Registra el voto de `voter` por el indice `option`. |
| `votes(option)` | Cantidad de votos de una opcion. |
| `has_voted(voter)` | Si una direccion ya voto. |
| `question()` / `options()` | Lee la pregunta y las opciones. |

Errores: `AlreadyInitialized`, `NotInitialized`, `AlreadyVoted`, `InvalidOption`, `VotingClosed`, `TooFewOptions`.

## Como correrlo
Requiere [Rust](https://rustup.rs) y el target wasm:
```
rustup target add wasm32v1-none
cd voting-contract
cargo test
cargo build --target wasm32v1-none --release
```
Desplegar en testnet con [Stellar CLI](https://developers.stellar.org/docs/tools/cli):
```
stellar contract deploy --wasm target/wasm32v1-none/release/voting.wasm --source <tu-identidad> --network testnet
stellar contract invoke --id <CONTRACT_ID> --source <tu-identidad> --network testnet -- init --question "Que recital sigue?" --options '["Rock","Jazz"]' --deadline 1900000000
stellar contract invoke --id <CONTRACT_ID> --source <tu-identidad> --network testnet -- vote --voter <TU_DIRECCION> --option 0
stellar contract invoke --id <CONTRACT_ID> --network testnet -- votes --option 0
```

## Pruebas (`contracts/voting/src/test.rs`)
- `counts_votes`: suma correctamente votos de distintas direcciones.
- `marks_voter`: `has_voted` pasa de falso a verdadero.
- `rejects_double_vote`: el segundo voto falla con `AlreadyVoted` y no cambia el conteo.
- `rejects_invalid_option`: opcion inexistente falla con `InvalidOption`.
- `rejects_after_deadline`: voto despues del plazo falla con `VotingClosed`.
- `rejects_double_init_and_few_options`: no se puede re-inicializar ni crear con menos de 2 opciones.

Las pruebas tambien corren automaticamente en GitHub Actions (`.github/workflows/voting-contract.yml`).
