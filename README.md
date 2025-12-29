Rust_EK Dokumentation


Beispiel Funktion:
```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

Die restlichen Funktionen können in scr/lib.rs nachgelesen werden.

| Code | Funktion | Beispiel |
| ----------- | ----------- | --------- |
| `pub`     | public    | |
| `fn`    | function    | |
| `name: &str` | String Slice, ausgeborgter String Parameter namens name| |
| `-> String` | Rückgabetyp String, Owned String der Funktion | |
| `format!` | ! bedeutet es ist ein Macro, Macros generieren Code zur Kompilerzeit (baut einen neues String zusammen| |
| `nums: &[i32]` | Slice von 32-Bit initialisieren | |
| `.iter()` | Iteratir erstellen der die Zahlen durchgeht | |
| `.sum()` | verwendet den Iterator und summiert ihn auf | |
| `!` | logischer NOT-Operator | |
| `s: String` | Ownership Move, wer die Funktion aufruft überträgt seinen String an die Funktion, dieser kann von der Funktion auch gelöscht werden | |
| `usize` | Datentyp für Längen und Indizes | |
| `s.len()` | Länge des Strings | |
| `&mut` | mutable Referenz, erlaubt den Wert zu verändern ohne ih zu besitzen, immer nur 1 in Rust | |
| `s.push()` | Wert des Strings verändern| |
| `#[derive(...))` | Attribut zur automatischen Code generierung | |
| `struct` | definiert ein Objekt mit Feldern, wie Klasse ohne Methoden | `pub struct Point { ... }`|
| `impl` | in diesem Bereich werden die Methoden für Struct definiert | `impl Point { }`|
| `&self` | wie this, Objekt wird nur ausgeliehen | `pub fn function1(&self, other: &Other) ..` |
| `.powi(2)` | Power Integer, hoch 2 | |
| `<T: Plottable>` | ein Generic, Typparameter T solange das Interface Trait Plottable erfüllt | |
| `&[T]` | Slice, refernez auf Teil eines Arrays oder Vektors | |
| `Option<&T>` | Enum, gibt entweder Same(&T) oder None (Liste war leer) zurück | |
| `\|a, b\|` | Parameter einer Closure (wie Lamda) | `items.iter().max_by(\|a, b\| { ... })` |
| `Result<u16, String>` | Entweder es funktioniert Ok(u16) oder es gibt einen Fehler err(String) | `pub fn parse_port(s: &str) -> Result<u16, String> {}` |
| `::<u16>` | | |
| `.map_err` | wenn parse fehlschlägt wird der Fehler mit eigener fehlermeldung versehen | `s.parse::<u16>().map_err(\|e\| format!("Invalid port: {}", e))` | `0..=n` | Ein Range, = bedeutet inclusive, also inc´klusive der Zahl n | |
| `.filter` | | |
| `.map` | | |
| `.collect()` | sammelt die Werte des Iterators in eine Kollektion | `(0..=n).filter(|x| x % 2 == 0).map(|x| x * x).collect()` |


