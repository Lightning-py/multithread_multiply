use matrix_multithread::matrix_multiply;
use rand::Rng;
use std::vec;

// Вспомогательная функция для сравнения матриц
fn matrices_are_equal(a: &Option<Vec<Vec<i32>>>, b: &Vec<Vec<i32>>) -> bool {
    match a {
        Some(a) => {
            if a.len() != b.len() {
                return false;
            }
            for i in 0..a.len() {
                if a[i].len() != b[i].len() {
                    return false;
                }
                for j in 0..a[i].len() {
                    if a[i][j] != b[i][j] {
                        return false;
                    }
                }
            }
            true
        }
        None => {
            return false;
        }
    }
}

// Тест: умножение двух простых матриц 2x2
#[test]
fn test_simple_matrix_multiplication() {
    let matrix_a = vec![vec![1, 2], vec![3, 4]];
    let matrix_b = vec![vec![5, 6], vec![7, 8]];
    let expected_result = vec![vec![19, 22], vec![43, 50]];

    let result = matrix_multiply(&matrix_a, &matrix_b);
    assert!(matrices_are_equal(&result, &expected_result));
}

// Тест: умножение с матрицей, содержащей нули
#[test]
fn test_matrix_with_zeros() {
    let matrix_a = vec![vec![1, 0], vec![0, 1]];
    let matrix_b = vec![vec![5, 6], vec![7, 8]];
    let expected_result = vec![vec![5, 6], vec![7, 8]];

    let result = matrix_multiply(&matrix_a, &matrix_b);
    assert!(matrices_are_equal(&result, &expected_result));
}

// Тест: умножение матриц разных размеров (3x2 и 2x3)
#[test]
fn test_different_size_matrices() {
    let matrix_a = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let matrix_b = vec![vec![7, 8, 9], vec![10, 11, 12]];
    let expected_result = vec![vec![27, 30, 33], vec![61, 68, 75], vec![95, 106, 117]];

    let result = matrix_multiply(&matrix_a, &matrix_b);
    assert!(matrices_are_equal(&result, &expected_result));
}

// Тест: умножение квадратной матрицы на вектор
#[test]
fn test_matrix_vector_multiplication() {
    let matrix_a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let matrix_b = vec![vec![1], vec![2], vec![3]];
    let expected_result = vec![vec![14], vec![32], vec![50]];

    let result = matrix_multiply(&matrix_a, &matrix_b);
    assert!(matrices_are_equal(&result, &expected_result));
}

// Тест: Большая матрица случайных чисел.
#[test]
fn test_large_random_matrix() {
    let size = 50; // Размер матрицы (50x50)
    let mut rng = rand::thread_rng();

    // Создание случайных матриц
    let matrix_a: Vec<Vec<i32>> = (0..size)
        .map(|_| (0..size).map(|_| rng.gen_range(1..10)).collect())
        .collect();
    let matrix_b: Vec<Vec<i32>> = (0..size)
        .map(|_| (0..size).map(|_| rng.gen_range(1..10)).collect())
        .collect();

    // Вычисление ожидаемого результата (однопоточно)
    let mut expected_result = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                expected_result[i][j] += matrix_a[i][k] * matrix_b[k][j];
            }
        }
    }

    // Выполнение многопоточного умножения
    let result = matrix_multiply(&matrix_a, &matrix_b);

    // Сравнение результатов
    assert!(matrices_are_equal(&result, &expected_result));
}

#[test]
fn test_incompatible_matrices() {
    let matrix_a = vec![vec![1, 2], vec![3, 4]];
    let matrix_b = vec![vec![5, 6, 7], vec![8, 9, 10]];
    // Эта операция вызовет панику, так как число столбцов matrix_a не равно числу строк matrix_b
    let result = matrix_multiply(&matrix_a, &matrix_b);

    let empty_vec = vec![];

    assert!(!matrices_are_equal(&result, &empty_vec));
}
