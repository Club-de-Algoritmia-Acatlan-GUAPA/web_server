use serde::{Deserialize, Serialize};

/// # Id concurso (20 bits):
///
///    Se auto serializa en la base de datos de uno en uno,se guarda en 20 bits
///    es decir podemos tener un maximo de 2^20 concursos con 32 problemas cada
/// uno.
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ContestId(u32);

///# Id problema (29bits):
///
///    Pueden existir dos tipos de problemas :
///
///        - Problemas que forman parte de un concurso ( estos problemas son
///          originales del concurso ).
///        - Problemas que son individuales, es decir que no formar parte de un
///          concurso.
///
///    Para diferenciar si el problema es individual o de concurso, se reserva
/// el primer bit para indicarlo.
///
///        1 para indicar que es de concurso
///        0 para indicar que es indiviadual
///
///    - Problemas que forman parte de un concurso: Se genera uniendo el
///      id_concurso mas el numero correspondiente del problema en el concurso.
///      A lo mas un concurso puede tener 256 problemas, es decir 8 bits,
///      entonces:
///
///        1 bit para indicar que es de concurso
///        20 bits del id_concurso
///        8 bits del numero de problema
///
///        Ejemplo :
///
///            El problem_id de el quinto problema del concurso con contest_id =
/// 1234 tendria la siguiente estructura:
///
///         ```
///            Indica que es concurso      numero de problema 8 bits
///                  v                             v
///                | 1 | 00000000010011010010 | 00000101
///                               ^
///                            20 bits
///         ```
///    - Problemas individuales Este es un numero seriado generado por la base
///      de datos, para que concuerde con el contest_id se guarda 1 bit para
///      indicar que es de concurso 20 bits del id_concurso 8 bits del numero de
///      problema
///
///        Ejemplo :
///
///            El problem_id de el quinto problema del concurso con contest_id =
/// 1234 tendria la siguiente estructura:
///
///         ```
///            Indica que es concurso      numero de problema
///                  v                             v
///                | 0 | 00000000010011010010 | 00000101
///                               ^
///                            20 bits
///         ```
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ProblemID(u32);

///# Id submision (128 bits)
///
///    Se compone de concatenar :
///        - la hora desde unix_epoch en milisegundos, se guarda en 41 bits
///        - el id_problema  -> 29 bits
///        - el id_concurso -> 20 bits
///        - 10 bits aleatorios del user_id -> 10 bits
///
///    Total 100 bits
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct SubmissionId(u128);
