use quizx::circuit::Circuit;
use quizx::equality::compare_tensors;
use quizx::extract::ToCircuit;
use quizx::simplify::full_simp;
use quizx::vec_graph::Graph;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::time::Instant;

fn main() {
    for nqubits in 2..8 {
        // Start measuring time
        let time = Instant::now();

        // Read both circuits and combine them into one
        let ca = Circuit::from_file(&format!("temp/{}a.qasm", nqubits)).unwrap();
        let cb = Circuit::from_file(&format!("temp/{}b.qasm", nqubits)).unwrap();
        let c = ca.to_adjoint() + cb;

        // Simplify circuit
        let mut g: Graph = c.to_graph();
        full_simp(&mut g);
        let c_new = g.to_circuit().unwrap();

        // Check if circuit is the identity and not empty
        let c_iden = Circuit::new(nqubits);
        if compare_tensors(&c_new, &c_iden) && c_iden != c_new {
            // Save circuit to file
            let qasm = c_new.to_qasm();
            let mut hasher = DefaultHasher::new();
            qasm.hash(&mut hasher);
            let hash_result = hasher.finish();
            let short_hash = format!("{:x}", hash_result);
            let short_hash = &short_hash[..8];
            match fs::write(&format!("circuits/{}/{}.qasm", nqubits, short_hash), qasm) {
                Ok(_) => println!("Found unsimplifiable identity circuit!"),
                Err(e) => eprintln!("Failed to write file: {}", e),
            }
        }

        // Print required time
        println!(
            "...circuit with {} qubits done in {:.2?}",
            nqubits,
            time.elapsed()
        );
    }
}
