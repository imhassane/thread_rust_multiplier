mod types;

extern crate rand;

use rand::Rng;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {

    let mut product = types::Product::new();
    product.init();

    let mut random_generator = rand::thread_rng();

    for idx in 0..product.get_size() {
        // On initialise les tableaux V1 et V2.
        let fir: i32 = random_generator.gen_range(1, 20);
        let sec: i32 = random_generator.gen_range(1, 20);
        product.set_v1(idx, fir);
        product.set_v2(idx, sec);
    }

    // On partage le produit entre les threads.
    let synchro = Arc::new((Mutex::new(product.clone()), Condvar::new()));
    
    // On créé une liste de threads.
    let mut threads_list = vec![];

    // Calcul
    for idx in 0..product.get_size() {
        let synchronizer = synchro.clone();

        threads_list.push(
            thread::spawn(
                move ||
                {
                    let (lock, cvar) = &*synchronizer;
                    let mut prod = lock.lock().unwrap();
                    let sum = prod.get_v1(idx) + prod.get_v2(idx);
                    prod.set_v3(idx, sum);
                    prod.set_pending(idx, 0);

                    if prod.get_nb_pending() == 0 {
                        prod.set_state(types::State::ADD);
                        cvar.notify_one();
                    }
                }
            )
        );
    }

    // On attend que tous les threads se terminent.
    while let Some(thread) = threads_list.pop() {
        thread.join().unwrap();
    }

    {
        let (lock, cvar) = &*synchro;
        let mut prod = lock.lock().unwrap();
        match prod.state {
            types::State::ADD => {
                prod.find_final_result();
                prod.state = types::State::PRINT;
                cvar.notify_one();
            },
            _ => {
                cvar.wait(prod).unwrap();
            }
        }
    }

    {
        let (lock, cvar) = &*synchro;
        let mut prod = lock.lock().unwrap();
        match prod.state {
            types::State::PRINT => {
                println!("Le resultat final est: {}", prod.get_result());
                println!("{:?}", prod);
            },
            _ => {
                cvar.wait(prod).unwrap();
            }
        }
    }
}
