# Ownership

> Le riflessioni di un principiante di Rust.
>
> Capitoli coinvolti: 04, 10, 15


## References and Borrowing

TODO

## Lifetimes

TODO

## The Slice Type

- La definizione generale del tipo slice non è `&str` o `&[T]`, come sembra di capire dal capitolo corrispondente del libro: quelli sono i *riferimenti* a uno slice. I tipi slice veri e propri sono `[T]` e `str`. `str` è un po' più speciale, ha una forma sua: in memoria è rappresentato esattamente come uno slice di `u8`, cioè come `[u8]`, ma con la restrizione aggiuntiva di contenere sempre UTF-8 valido. Non è però *lo stesso* tipo di `[u8]`: per passare dall'uno all'altro servono `as_bytes()` (che riesce sempre) e `str::from_utf8()` (che invece può fallire, perché deve verificare l'invariante).

- Si usa quindi quasi sempre il riferimento allo slice, perché lo slice in sé non può essere usato dove è richiesto un tipo `Sized`. Per esempio `let`, per la definizione di una variabile, richiede un tipo `Sized`, perché il compilatore deve sapere quanto spazio allocare per la variabile sullo stack; ma lo slice è un Dynamically Sized Type (DST), e il compilatore non sa di quanto spazio ha bisogno in fase di compilazione. Lo stesso vale per gli argomenti e per i valori di ritorno delle funzioni: per default hanno un vincolo `Sized` implicito. Quindi questo è illegale:

  ```rust
  let s: [i32] = [1, 2, 3];
  ```

  Mentre questo è legale:

  ```rust
  let s: &[i32] = &[1, 2, 3];
  ```

  perché il riferimento allo slice è un tipo `Sized`, e la variabile riceve come valore il puntatore allo slice (più la sua lunghezza). Da notare che `&[1, 2, 3]` ha in realtà tipo `&[i32; 3]`, cioè un riferimento a un array di dimensione nota, e viene convertito in `&[i32]` tramite *unsized coercion*.

- **Attenzione:** `&[T]` è `Sized`, ma la sua dimensione non è quella di un puntatore normale, è il doppio: puntatore + lunghezza, cioè 16 byte su una piattaforma a 64 bit. Si può verificare con `std::mem::size_of::<&[i32]>()`. È `Sized` perché la sua dimensione è nota staticamente, non perché sia grande quanto un puntatore.

- **Fat pointer:** il puntatore che contiene anche informazioni aggiuntive, per esempio l'informazione della lunghezza dei dati a cui punta. `&str` e `&[T]` sono per esempio dei fat pointer: puntano direttamente ai dati, che sono di un tipo dinamicamente dimensionato, quindi il puntatore da solo non basterebbe e deve portarsi dietro anche la lunghezza. Invece `&String` è un puntatore normale (thin pointer), perché le informazioni necessarie (puntatore ai dati, lunghezza, capacità) sono già contenute nel valore di tipo `String`: il puntatore si occupa semplicemente di puntare all'indirizzo di quel valore.

- Due precisazioni sui fat pointer:
  - L'informazione aggiuntiva non è sempre la lunghezza. In `&dyn Trait` la seconda parola è il puntatore alla vtable, cioè alla tabella dei metodi da chiamare.
  - Non sono solo i riferimenti a essere fat pointer: lo sono anche `Box<[T]>`, `Rc<str>` e in generale qualsiasi puntatore verso un DST.