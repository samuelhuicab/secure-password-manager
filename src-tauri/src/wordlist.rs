//! Lista de palabras para generar frases de contraseña (passphrases).
//!
//! La entropía por palabra es log2(WORDS.len()) (~8 bits). El generador
//! calcula la entropía total en función del nº de palabras elegido y
//! recomienda 6 o más.

pub const WORDS: &[&str] = &[
    "abaco", "abeja", "abrigo", "acero", "agua", "aire", "ajedrez", "alba",
    "alga", "almeja", "ancla", "anillo", "arbol", "arco", "arena", "arpa",
    "avena", "avion", "ayuda", "azul", "bache", "bahia", "baile", "balde",
    "banco", "barco", "barro", "beso", "bloque", "boina", "bosque", "bota",
    "bronce", "buho", "burro", "cabra", "cactus", "cadena", "cafe", "caja",
    "cal", "calle", "cama", "campo", "canoa", "capa", "cara", "carbon",
    "carro", "casa", "cebra", "cedro", "cielo", "cima", "circo", "clave",
    "clima", "cobre", "cofre", "col", "cometa", "copa", "coral", "corcho",
    "coro", "crema", "cresta", "cubo", "cuerda", "cueva", "dado", "danza",
    "dedo", "delta", "diente", "disco", "duna", "eco", "espada", "espiga",
    "estrella", "faro", "fibra", "fiera", "fila", "flauta", "flor", "foca",
    "fresa", "fruta", "fuego", "fuente", "galaxia", "ganso", "garra", "gato",
    "gema", "gemelo", "globo", "goma", "gorra", "grano", "grulla", "guante",
    "guitarra", "halcon", "harina", "hebra", "helecho", "hielo", "hierba",
    "higo", "hilo", "hoja", "hongo", "horno", "hueso", "humo", "iglu",
    "iman", "isla", "jabon", "jamon", "jarra", "jaula", "jazmin", "joya",
    "juego", "junco", "kilo", "labio", "lago", "lampara", "lana", "lapiz",
    "leche", "lente", "leon", "libro", "lima", "limon", "lince", "lino",
    "lirio", "loma", "loro", "loza", "luna", "lupa", "luz", "maceta",
    "madera", "maiz", "malla", "mango", "mano", "mapa", "mar", "marea",
    "masa", "mesa", "miel", "mimbre", "mina", "mono", "monte", "mora",
    "morsa", "musgo", "nabo", "nardo", "nave", "nido", "niebla", "nieve",
    "nube", "nuez", "ola", "olivo", "olmo", "onda", "oro", "oruga",
    "oso", "ostra", "pala", "palma", "pan", "panal", "papa", "parra",
    "pato", "pecera", "pera", "perla", "pez", "pino", "pinza", "piña",
    "pipa", "pluma", "polea", "polen", "polvo", "poste", "pozo", "prado",
    "puente", "puerta", "pulpo", "queso", "quilla", "rama", "rana", "raton",
    "rayo", "red", "reja", "remo", "resina", "rio", "risa", "roble",
    "roca", "rombo", "ropa", "rosa", "rueda", "sal", "salmon", "sapo",
    "sauce", "seda", "selva", "seta", "seto", "sierra", "silla", "sol",
    "sombra", "sopa", "surco", "tabla", "taco", "talco", "tallo", "tambor",
    "tapa", "taza", "techo", "tejado", "tela", "tigre", "tinta", "topo",
    "torre", "trigo", "trineo", "tronco", "tuba", "tuna", "uva", "vaca",
    "valle", "vapor", "vela", "venado", "vid", "vidrio", "viento", "zorro",
];
