#[derive(Debug, Clone)]
struct Vuelo {
    id: String,
    altitud: u32, // Este será nuestra clave (key)
}

struct Nodo {
    vuelo: Vuelo,
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

impl Nodo {
    fn nuevo(vuelo: Vuelo) -> Self {
        Nodo {
            vuelo,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}

// --- UTILIDADES DE BALANCEO (NO MODIFICAR) ---

fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}

fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    let mut x = y.izquierdo.take().expect("Error de radar");
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}

fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Error de radar");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}

// --- FUNCIÓN DE INSERCIÓN ---

fn insertar(nodo_opt: Option<Box<Nodo>>, vuelo: Vuelo) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(vuelo)),
        Some(n) => n,
    };

    if vuelo.altitud < nodo.vuelo.altitud {
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), vuelo.clone()));
    } else if vuelo.altitud > nodo.vuelo.altitud {
        nodo.derecho = Some(insertar(nodo.derecho.take(), vuelo.clone()));
    } else {
        return nodo;
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    // Caso Izquierda-Izquierda
    if balance > 1 && vuelo.altitud < nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Derecha
    if balance < -1 && vuelo.altitud > nodo.derecho.as_ref().unwrap().vuelo.altitud {
        return rotar_izquierda(nodo);
    }
    // Caso Izquierda-Derecha
    if balance > 1 && vuelo.altitud > nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Izquierda
    if balance < -1 && vuelo.altitud < nodo.derecho.as_ref().unwrap().vuelo.altitud {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }

    nodo
}

//FASE 2 EXAMEN PARCIAL 3 ESTRUCTURA DE DATOS II
//LOCALIZACIÓN DE LOS VUELOS

fn buscar_vuelo(nodo: &Option<Box<Nodo>>, altitud: u32) -> Option<&Vuelo> {
    match nodo {
        None => None,
        Some(n) => {
            if altitud < n.vuelo.altitud {
                buscar_vuelo(&n.izquierdo, altitud)
            } else if altitud > n.vuelo.altitud {
                buscar_vuelo(&n.derecho, altitud)
            } else {
                Some(&n.vuelo)
            }
        }
    }
}

//FASE 3 EXAMEN PARCIAL 3 ESTRUCTURA DE DATOS II
//DESCENSO Y ATERRIZAJE (ELIMINACIÓN)

fn extraer_maximo(mut nodo: Box<Nodo>) -> (Box<Nodo>, Vuelo) {
    match nodo.derecho.take() {
        None => {
            let subarbol_izq = nodo.izquierdo.take();
            let valor_vuelo = nodo.vuelo.clone();
            if let Some(mut n) = subarbol_izq {
                actualizar_altura(&mut n);
                (n, valor_vuelo)
            } else {
                (nodo, valor_vuelo)
            }
        }
        Some(derecho_nodo) => {
            let (nuevo_derecho, valor) = extraer_maximo(derecho_nodo);
            if nuevo_derecho.izquierdo.is_none()
                && nuevo_derecho.derecho.is_none()
                && obtener_balance(&nuevo_derecho) == 0
            {
                nodo.derecho = None;
            } else {
                nodo.derecho = Some(nuevo_derecho);
            }
            actualizar_altura(&mut nodo);
            (nodo, valor)
        }
    }
}

fn eliminar_vuelo(nodo_opt: Option<Box<Nodo>>, altitud: u32) -> Option<Box<Nodo>> {
    let mut nodo = match nodo_opt {
        None => return None,
        Some(n) => n,
    };

if altitud < nodo.vuelo.altitud {
        nodo.izquierdo = eliminar_vuelo(nodo.izquierdo.take(), altitud);
    } else if altitud > nodo.vuelo.altitud {
        nodo.derecho = eliminar_vuelo(nodo.derecho.take(), altitud);
    } else {
        if nodo.izquierdo.is_none() {
            return nodo.derecho;
        } else if nodo.derecho.is_none() {
            return nodo.izquierdo;
        } else {
            let izq = nodo.izquierdo.take().unwrap();
            if izq.derecho.is_none() {
                let mut reemplazo = izq;
                reemplazo.derecho = nodo.derecho.take();
                nodo = reemplazo;
            } else {
                let (nueva_izquierda, vuelo_reemplazo) = extraer_maximo(izq);
                nodo.vuelo = vuelo_reemplazo;
                nodo.izquierdo = Some(nueva_izquierda);
            }
        }
    }
    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) >= 0 {
        return Some(rotar_derecha(nodo));
    }
    if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) < 0 {
        let izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(izq));
        return Some(rotar_derecha(nodo));
    }
    if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) <= 0 {
        return Some(rotar_izquierda(nodo));
    }
    if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) > 0 {
        let der = nodo.derecho.take().unwrap();
        nodo.derecho =Some(rotar_derecha(der));
        return Some(rotar_izquierda(nodo));

    }

    Some(nodo)
    
}

//FASE 4 EXAMEN PARCIAL 3 ESTRUCTURA DE DATOS II
//OPTE POR HACER LA OPCIÓN A) ALERTA DE PROXIMIDAD
fn vuelos_en_rango(nodo: &Option<Box<Nodo>>, min: u32, max: u32) -> usize {
    match nodo {
        None => 0,
        Some(n) => {
            let mut count = 0;
            if n.vuelo.altitud >= min && n.vuelo.altitud <= max {
                count += 1;
            }
            if n.vuelo.altitud > min {
                count += vuelos_en_rango(&n.izquierdo, min, max);
            }
            if n.vuelo.altitud < max {
                count += vuelos_en_rango(&n.derecho, min, max);
            }
            count
        }
    }
}
fn main() {
    let mut radar: Option<Box<Nodo>> = None;

    // Simulación de entrada de vuelos
    let datos = vec![
        ("AV123", 5000),
        ("UA456", 3000),
        ("IB101", 2000),
        ("AF999", 4000),
        ("TA222", 3500),
        ("AM777", 6000),
    ];

    for (id, alt) in datos {
        let v = Vuelo {
            id: id.to_string(),
            altitud: alt,
        };
        radar = Some(insertar(radar.take(), v));
    }

    println!("--- Radar de Control Aéreo (AVL) ---");
    // Aquí el estudiante debe invocar sus funciones de búsqueda y eliminación

    //FASE 2
    match buscar_vuelo(&radar, 4000) {
        Some(vuelo) => println!("Vuelo encontrado: ID: {}, Altitud: {} pies", vuelo.id, vuelo.altitud),
        None => println!("Vuelo no detectado en el radar."),
    }

    //FASE 4
    let alertas = vuelos_en_rango(&radar, 3000, 4200);
    println!("Alerta de trafico: {} vuelos detectados en zona critica.", alertas);

    //FASE 3
    println!("\n RADAR: Procesando el aterrizaje del vuelo de 4000 pies...");
    radar = eliminar_vuelo(radar.take(), 4000);

    match buscar_vuelo(&radar, 4000) {
        Some(_) => println!("Error: El vuelo sigue activo."),
        None => println!("Confirmación: El vuelo aterrizó y fue removido de forma segura."),
    }

}

//LA FASE 1 EXPLICACA EN EL FASE_1.txt
