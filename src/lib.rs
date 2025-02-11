use std::sync::{Arc, Mutex};
use std::thread;

pub fn matrix_multiply(
    matrix_a: &Vec<Vec<i32>>,
    matrix_b: &Vec<Vec<i32>>,
) -> Option<Vec<Vec<i32>>> {
    if !(matrix_a.len() > 0 && matrix_b.len() > 0 && matrix_a[0].len() == matrix_b.len()) {
        return None;
    }

    let rows_a = matrix_a.len();
    let cols_a = matrix_a[0].len();
    let cols_b = matrix_b[0].len();

    let result = Arc::new(Mutex::new(vec![vec![0; cols_b]; rows_a]));

    let num_threads = num_cpus::get();

    let chunk_size = rows_a / num_threads;

    let matrix_a = Arc::new(matrix_a.clone());
    let matrix_b = Arc::new(matrix_b.clone());

    let mut handles = vec![];

    for i in 0..num_threads {
        let start_row = i * chunk_size;
        let end_row = if i == num_threads - 1 {
            rows_a
        } else {
            (i + 1) * chunk_size
        };

        let matrix_a = Arc::clone(&matrix_a);
        let matrix_b = Arc::clone(&matrix_b);

        let result = Arc::clone(&result);

        let handle = thread::spawn(move || {
            for row_a in start_row..end_row {
                for col_b in 0..cols_b {
                    let mut sum = 0;

                    for k in 0..cols_a {
                        sum += matrix_a[row_a][k] * matrix_b[k][col_b];
                    }

                    let mut result = result.lock().unwrap();
                    result[row_a][col_b] = sum;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = result.lock().unwrap();

    return Some(result.clone());
}
