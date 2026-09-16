## Cargo.lock

- Il `Cargo.lock` funziona in questo modo, quando scrivi la dipendenza `rand` con la versione `0.8.5` nel tuo `Cargo.toml` e quindi fai la compilazione, Cargo risolve le dipendenze e scarica la versione più recente che soddisfa il vincolo specificato, `0.8.5` significa `^0.8.5` il quale vuol dire `>=0.8.5, <0.9.0`. Quindi se esiste una versione `0.8.6`, Cargo la scaricherà e la userà per la compilazione. Dopo la prima risoluzione delle dipendenze, Cargo.lock viene creato e fissa la versione specifica della libreria `rand` che è stata effettivamente utilizzata, in questo caso `0.8.5` (assumendo che quella fosse la versione più recente disponibile al momento della compilazione). Se nel frattempo esce `0.8.6`, `cargo build` di nuovo, poiché il `Cargo.lock` esiste, Cargo non ri-risolve le dipendenze, non guarda nemmeno crates.io per rand e continua a usare la versione `0.8.5` specificata nel lock file che ha già in cache.

- Il lock non fissa solo `rand` in questo caso, ma anche le dipendenze da cui `rand` dipende con le specifiche versioni.

- Per aggiornare davvero c'è `cargo update`: ignora il `Cargo.lock` e ri-risolve le dipendenze partendo dai vincoli del `Cargo.toml`, poi riscrive il lock con le versioni trovate. Resta però dentro i vincoli SemVer: con `rand = "0.8.5"` (`>=0.8.5, <0.9.0`), se esistono `0.8.6` e `0.999.0`, aggiorna a `0.8.6` e ignora `0.999.0` perché fuori range.

---

# Come strutturare i progetti Rust

## Package

Un package deve contenere almeno un crate.

Può contenere:

- zero o più **binary crate**;
- al massimo un **library crate**.

Il crate root predefinito del binary principale è `src/main.rs`, mentre quello del library crate è `src/lib.rs`.

Se un package contiene più binary crate, quelli aggiuntivi possono essere messi nella cartella `src/bin/`. Ogni file `.rs` direttamente dentro questa cartella viene considerato un binary crate separato.

Se si vuole creare un binary crate composto da più file, cioè con dei propri moduli, si può creare una cartella con il nome del binary dentro `src/bin/` e inserire al suo interno un file `main.rs` insieme agli altri moduli `.rs`.

`main.rs` e `lib.rs` sono i crate root predefiniti, non necessariamente i nomi dei crate. Cargo determina automaticamente i nomi dei target in base al package, ai file o alle directory, ma è possibile configurare esplicitamente nomi e percorsi tramite `Cargo.toml`.

Una possibile struttura comune è:

```text
src/
├── main.rs
├── lib.rs
├── module1.rs
├── module2.rs
└── bin/
    ├── binary1.rs
    └── binary2/
        ├── main.rs
        └── module.rs

tests/
├── common/
│   └── mod.rs
├── integration_test1.rs
├── integration_test2.rs
└── ...
```

I file come `module1.rs` e `module2.rs` non diventano automaticamente dei moduli: devono essere dichiarati nella module tree, per esempio con:

```rust
mod module1;
```

### Integration test

I file `.rs` direttamente dentro la cartella `tests/` vengono compilati da Cargo come **crate separati**.

Per esempio:

```text
tests/
├── integration_test1.rs
└── integration_test2.rs
```

corrisponde concettualmente a:

```text
integration_test1.rs → integration test crate 1
integration_test2.rs → integration test crate 2
```

Questo è importante perché gli integration test vengono compilati come codice esterno rispetto al library crate che stanno testando. Possono quindi utilizzare la sua **public API**, ma non accedere direttamente ai suoi elementi privati.

Per esempio:

```rust
use my_crate::some_public_function;

#[test]
fn some_test() {
    assert!(some_public_function());
}
```

Un integration test non può invece importare direttamente le funzioni interne di un binary crate come se questo fosse una library.

È comunque possibile fare integration test del comportamento di un binary eseguendo il programma e verificandone input, output, exit status, ecc.

Tuttavia, se si vuole testare direttamente la logica principale dell'applicazione, un approccio molto utile è spostare tale logica nel library crate del package e lasciare il binary crate come un semplice wrapper che si occupa principalmente di avviare il programma.

In questo modo:

```text
main.rs
    ↓
public API di lib.rs
    ↓
logica principale
```

Gli integration test possono quindi utilizzare la public API del library crate per testare direttamente la logica principale dell'applicazione.

### Codice di supporto condiviso tra integration test

Anche se ogni file `.rs` direttamente dentro `tests/` rappresenta un integration test crate separato, è possibile creare del **codice di supporto condiviso** da utilizzare in più integration test crate.

Per esempio, se più test devono eseguire la stessa preparazione iniziale, si può creare:

```text
tests/
├── common/
│   └── mod.rs
├── user_tests.rs
└── product_tests.rs
```

> **Nota:** questa struttura usa intenzionalmente la convenzione `mod.rs`, tipica del vecchio sistema di organizzazione dei moduli di Rust. In questo caso è utile perché permette di evitare la regola speciale di Cargo secondo cui i file `.rs` direttamente dentro `tests/` vengono considerati integration test target.

Con il sistema dei moduli più moderno, `tests/common.rs` può comunque essere usato come modulo dagli altri integration test tramite `mod common;`. Tuttavia, Cargo lo considera contemporaneamente anche un integration test target autonomo, proprio perché si trova direttamente nella directory `tests/`.

Per questo motivo si preferisce:

```text
tests/common/mod.rs
```

che può essere incluso come modulo dagli integration test senza essere automaticamente considerato da Cargo un integration test target separato.

Dentro `tests/common/mod.rs` si possono definire funzioni di supporto:

```rust
pub fn setup() {
    // preparazione comune ai test
}
```

Ogni integration test crate che vuole utilizzare questo codice deve dichiarare esplicitamente il modulo.

Per esempio:

```rust
// tests/user_tests.rs

mod common;

#[test]
fn create_user() {
    common::setup();
    // ...
}
```

e allo stesso modo:

```rust
// tests/product_tests.rs

mod common;

#[test]
fn create_product() {
    common::setup();
    // ...
}
```

È importante capire che `common` **non è un crate separato condiviso tra gli integration test**.

`user_tests.rs` e `product_tests.rs` rimangono due crate distinti:

```text
user_tests crate
├── common module
└── tests

product_tests crate
├── common module
└── tests
```

Entrambi includono un modulo `common` utilizzando lo stesso file sorgente `tests/common/mod.rs`.

In altre parole, `mod common;` dice a ciascun integration test crate di aggiungere `common` alla propria module tree.

Le funzioni di `common` che devono essere chiamate dal test devono avere una visibilità appropriata, per esempio:

```rust
pub fn setup() {
    // ...
}
```

oppure, se si vuole limitarne la visibilità al crate di test:

```rust
pub(crate) fn setup() {
    // ...
}
```

È quindi preferibile usare:

```text
tests/common/mod.rs
```

invece di:

```text
tests/common.rs
```

perché Cargo considera normalmente ogni file `.rs` direttamente dentro `tests/` come un integration test target separato.

Quindi, usando:

```text
tests/
├── common.rs
├── user_tests.rs
└── product_tests.rs
```

Cargo interpreterebbe anche `common.rs` come un integration test crate autonomo, nonostante contenga soltanto funzioni di supporto.

Durante `cargo test` potrebbe quindi comparire qualcosa come:

```text
Running tests/common.rs

running 0 tests
```

Mettendo invece il modulo dentro una sottocartella:

```text
tests/common/mod.rs
```

non viene trattato come integration test crate autonomo e può essere importato dagli altri test tramite:

```rust
mod common;
```

Il modulo `common` può a sua volta essere suddiviso in altri moduli se il codice di supporto diventa più complesso.

Per esempio:

```text
tests/
├── common/
│   ├── mod.rs
│   ├── database.rs
│   └── fixtures.rs
├── user_tests.rs
└── product_tests.rs
```

In `common/mod.rs`:

```rust
pub mod database;
pub mod fixtures;
```

Il codice di supporto dei test può quindi essere organizzato con il normale module system di Rust, senza trasformare ogni file di supporto in un integration test crate separato.

### Unit test

Gli **unit test**, invece, fanno parte dello stesso crate del codice che stanno testando.

Per convenzione vengono spesso inseriti in un sottomodulo:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_test() {
        // ...
    }
}
```

Poiché il modulo `tests` è un sottomodulo del modulo da testare, può accedere anche ai suoi elementi privati.

Per questo motivo gli unit test sono adatti anche a testare dettagli interni che non fanno parte della public API.

## Workspace

TODO

---

## Regression

Una regressione è un bug introdotto in una versione nuova che rompe un comportamento che prima funzionava. Non è un cambio di API dichiarato, ma un errore involontario dell'autore. Per esempio 0.8.6 potrebbe correggere un bug e per sbaglio far sì che gen_range(1..=100) non restituisca mai 100, o introdurre un panic in un caso limite. Il tuo codice continua a compilare perché le firme sono identiche, ma si comporta diversamente. Quindi non viola le regole di SemVer che è una promessa sull'API pubblica intenzionale, non è, e non potrebbe essere, una promessa che il codice sia privo di bug.

---

## Shadowing

TODO