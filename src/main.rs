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
        if compare_tensors(&c_new, &c_iden) && c_new.num_gates() != 0 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn every_circuit_is_identity_and_unsimplifiable() {
        // Iterate through every circuit size
        for nqubits in 2..8 {
            let path_str = format!("circuits/{}", nqubits);
            let path = Path::new(&path_str);

            // Iterate through every circuit
            for entry in fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let file_path = entry.path();
                let file_path_str = format!("{}", file_path.display());
                let c_old = Circuit::from_file(&file_path_str).unwrap();

                // Check if the circuit simplifies to the identity
                let mut g: Graph = c_old.to_graph();
                full_simp(&mut g);
                let c_new = g.to_circuit().unwrap();
                assert!(
                    c_new.num_gates() != 0,
                    "The circuit {} is fully simplifiable.",
                    file_path_str
                );

                // Check if the circuit is actually the identity
                let c_iden = Circuit::new(nqubits);
                assert!(
                    compare_tensors(&c_old, &c_iden),
                    "The circuit {} is not the identity.",
                    file_path_str
                );
            }
        }
    }
}
