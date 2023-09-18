var searchIndex = JSON.parse('{\
"TP_Taller_Definitivo":{"doc":"","t":"AAAFFFANENNNNLLLLLLLLLLLLLMMMMMMMMDLLLLLLLLLMLLLMMLLLLDMLLMMLLLLLLLLLLLLDLLLLFLMLLLLLLLL","n":["celda","enemigo","explosion","formatear_input","guardar_error_y_salir","main","mapa","Bomba","Celda","Desvio","Enemigo","Obstaculo","Vacio","borrow","borrow_mut","clone","clone_into","eq","fmt","from","into","obtener_representacion","to_owned","try_from","try_into","type_id","alcance","de_traspaso","direccion","enemigo","representacion","representacion","representacion","representacion","Enemigo","borrow","borrow_mut","clone","clone_into","eq","esta_vivo","fmt","from","herir","id","into","new","obtener_representacion","pv","representacion","to_owned","try_from","try_into","type_id","Explosion","alcance","borrow","borrow_mut","de_traspaso","enemigos_afectados","explotar_abajo","explotar_arriba","explotar_celda","explotar_derecha","explotar_izquierda","from","iniciar_explosion","into","new","try_from","try_into","type_id","Mapa","borrar","borrow","borrow_mut","crear_mapa","crear_objeto","from","grilla","guardar_mapa","into","mostrar_mapa","obtener_celda","obtener_largo","try_from","try_into","type_id"],"q":[[0,"TP_Taller_Definitivo"],[7,"TP_Taller_Definitivo::celda"],[26,"TP_Taller_Definitivo::celda::Celda"],[34,"TP_Taller_Definitivo::enemigo"],[54,"TP_Taller_Definitivo::explosion"],[72,"TP_Taller_Definitivo::mapa"]],"d":["","","","","","","","Celda de bomba con una representación de un carácter, …","Enum que representa los tipos de celdas/casilleros en el …","Celda de desvío con una representación de carácter y …","Celda de enemigo que contiene una instancia de Enemigo.","Celda de obstáculo con una representación de un …","Celda vacia con una representación de un carácter.","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Obtiene la representación de la celda como un string.","","","","","","","","","","","","","Representa a un enemigo en algún contexto.","","","","","","Verifica si el enemigo todavía tiene puntos de vida y …","","Returns the argument unchanged.","Reduce los puntos de vida del enemigo en 1, simbolizando …","Identificador único para distinguir entre distintas …","Calls <code>U::from(self)</code>.","Crea una nueva instancia de <code>Enemigo</code> con la representación …","Obtiene una representación del enemigo como una cadena de …","Puntos de vida del enemigo.","Carácter que representa visualmente al enemigo.","","","","","Simula una explosion ","Alcance de la explosión","","","Indicador de si la explosión fue generada por una bomba …","Enemigos afectados por la explosión, se utiliza para no …","","","","","","Returns the argument unchanged.","Simula la explosión de una bomba en el lugar indicado, …","Calls <code>U::from(self)</code>.","Crea una nueva instancia de Explosión, con alcance y …","","","","Representa a un mapa que contiene celdas con objetos","Borra una celda en la posición indicada, para eso …","","","Instancia un mapa con sus elementos a partir de un archivo …","Funcion para poder obtener una celda dado un string.","Returns the argument unchanged.","Grilla que representa el mapa","Guarda el mapa en la direccion indicada, si no existe el …","Calls <code>U::from(self)</code>.","Muestra el mapa en consola&amp;mut","Obtiene una referencia mutable a una celda contenida en un …","Devuelve el largo del mapa, que al ser cuadrado coincide …","","",""],"i":[0,0,0,0,0,0,0,6,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,19,19,20,21,22,19,23,20,0,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,0,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,0,17,17,17,17,0,17,17,17,17,17,17,17,17,17,17],"f":[0,0,0,[[1,1],[[5,[2,[4,[3]]]]]],[[1,1],[[5,[[4,[3]]]]]],[[],[[5,[[4,[3]]]]]],0,0,0,0,0,0,0,[[]],[[]],[6,6],[[]],[[6,6],7],[[6,8],9],[[]],[[]],[6,10],[[]],[[],5],[[],5],[[],11],0,0,0,0,0,0,0,0,0,[[]],[[]],[12,12],[[]],[[12,12],7],[12,7],[[12,8],9],[[]],[12],0,[[]],[[13,14,15],12],[12,10],0,0,[[]],[[],5],[[],5],[[],11],0,0,[[]],[[]],0,0,[[16,17,2,2,2],[[5,[[4,[3]]]]]],[[16,17,2,2,2],[[5,[[4,[3]]]]]],[[16,17,14,14,2],[[5,[7,[4,[3]]]]]],[[16,17,2,2,2],[[5,[[4,[3]]]]]],[[16,17,2,2,2],[[5,[[4,[3]]]]]],[[]],[[16,17,2,2],[[5,[[4,[3]]]]]],[[]],[[2,7],16],[[],5],[[],5],[[],11],0,[[17,14,14]],[[]],[[]],[1,[[5,[17,[4,[3]]]]]],[[1,15],[[5,[6,[4,[3]]]]]],[[]],0,[[17,1],18],[[]],[17],[[17,14,14],[[5,[6,[4,[3]]]]]],[17,14],[[],5],[[],5],[[],11]],"c":[],"p":[[15,"str"],[15,"i32"],[8,"Error"],[3,"Box"],[4,"Result"],[4,"Celda"],[15,"bool"],[3,"Formatter"],[6,"Result"],[3,"String"],[3,"TypeId"],[3,"Enemigo"],[15,"char"],[15,"usize"],[15,"u32"],[3,"Explosion"],[3,"Mapa"],[6,"Result"],[13,"Bomba"],[13,"Desvio"],[13,"Enemigo"],[13,"Vacio"],[13,"Obstaculo"]]},\
"tp_individual":{"doc":"","t":"AAAAANENNNNLLLLLLLLLLLLLLLLLLLLMMMMMMMMDLLLLLLLLLLLLLLLLDLLLLLLLLLFFDLLLLLLLLLLLLLL","n":["celda","enemigo","explosion","io","mapa","Bomba","Celda","Desvio","Enemigo","Obstaculo","Vacio","bomba_normal","bomba_traspaso","borrow","borrow_mut","clone","clone_into","desvio","enemigo","eq","fmt","from","into","obtener_representacion","pared","roca","to_owned","try_from","try_into","type_id","vacio","alcance","de_traspaso","direccion","enemigo","representacion","representacion","representacion","representacion","Enemigo","borrow","borrow_mut","clone","clone_into","eq","esta_vivo","fmt","from","herir","into","new","obtener_representacion","to_owned","try_from","try_into","type_id","Explosion","borrow","borrow_mut","explotar","from","into","new","try_from","try_into","type_id","guardar_error_y_salir","obtener_input","Mapa","borrar","borrow","borrow_mut","crear_mapa","from","guardar_mapa","into","mostrar_mapa","new","obtener_celda","obtener_largo","try_from","try_into","type_id"],"q":[[0,"tp_individual"],[5,"tp_individual::celda"],[31,"tp_individual::celda::Celda"],[39,"tp_individual::enemigo"],[56,"tp_individual::explosion"],[66,"tp_individual::io"],[68,"tp_individual::mapa"]],"d":["","","","","","Celda de bomba con una representación de un carácter, …","Enum que representa los tipos de celdas/casilleros en el …","Celda de desvío con una representación de carácter y …","Celda de enemigo que contiene una instancia de Enemigo.","Celda de obstáculo con una representación de un …","Celda vacia con una representación de un carácter.","Crea una celda de bomba normal con una representación, …","Crea una celda de bomba de traspaso con una …","","","","","Crea una celda de desvío con una representación y …","Crea una celda de enemigo con puntos de vida y un …","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Obtiene la representación de la celda como un string.","Crea una celda de pared.","Crea una celda de roca.","","","","","Crea una celda vacía con una representación por defecto.","","","","","","","","","Representa a un enemigo en algún contexto.","","","","","","Verifica si el enemigo todavía tiene puntos de vida y …","","Returns the argument unchanged.","Reduce los puntos de vida del enemigo en 1, simbolizando …","Calls <code>U::from(self)</code>.","Crea una nueva instancia de <code>Enemigo</code> con la representación …","Obtiene una representación del enemigo como una cadena de …","","","","","","","","Obtiene la celda indicada en el mapa indicado y luego, en …","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Crea una nueva instancia de Explosión, con alcance y …","","","","Guarda un mensaje de error en un archivo cuyo nombre …","Obtiene de los argumentos y devuelve los file paths …","Representa a un mapa que contiene celdas con objetos","Borra una celda en la posición indicada, para eso …","","","Instancia un mapa con sus elementos a partir de un archivo …","Returns the argument unchanged.","Guarda el mapa en la direccion indicada, si no existe el …","Calls <code>U::from(self)</code>.","Muestra el mapa en consola","Dada una grilla crea una instanc}ia de Mapa","Obtiene una referencia mutable a una celda contenida en un …","Devuelve el largo del mapa, que al ser cuadrado coincide …","","",""],"i":[0,0,0,0,0,2,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,20,20,21,22,23,20,24,21,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,17,17,17,17,17,17,17,17,17,0,0,0,13,13,13,13,13,13,13,13,13,13,13,13,13,13],"f":[0,0,0,0,0,0,0,0,0,0,0,[1,2],[1,2],[[]],[[]],[2,2],[[]],[3,2],[[1,4],2],[[2,2],5],[[2,6],7],[[]],[[]],[2,8],[[],2],[[],2],[[]],[[],9],[[],9],[[],10],[[],2],0,0,0,0,0,0,0,0,0,[[]],[[]],[11,11],[[]],[[11,11],5],[11,5],[[11,6],7],[[]],[11],[[]],[[3,1,4],11],[11,8],[[]],[[],9],[[],9],[[],10],0,[[]],[[]],[[12,13,14,14],[[9,[[16,[15]]]]]],[[]],[[]],[[14,5],17],[[],9],[[],9],[[],10],[[12,12],[[9,[[16,[15]]]]]],[[],[[9,[[16,[15]]]]]],0,[[13,1,1]],[[]],[[]],[12,[[9,[13,[16,[15]]]]]],[[]],[[13,12],18],[[]],[13],[[[19,[[19,[2]]]]],13],[[13,1,1],[[9,[2,[16,[15]]]]]],[13,1],[[],9],[[],9],[[],10]],"c":[],"p":[[15,"usize"],[4,"Celda"],[15,"char"],[15,"u32"],[15,"bool"],[3,"Formatter"],[6,"Result"],[3,"String"],[4,"Result"],[3,"TypeId"],[3,"Enemigo"],[15,"str"],[3,"Mapa"],[15,"i32"],[8,"Error"],[3,"Box"],[3,"Explosion"],[6,"Result"],[3,"Vec"],[13,"Bomba"],[13,"Desvio"],[13,"Enemigo"],[13,"Vacio"],[13,"Obstaculo"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
